#[cfg(test)]
mod tests;
mod utils;

use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use clap::{crate_version, Parser};
use gethostname::gethostname;
use log::{debug, info, LevelFilter};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;
use tokio::sync::OnceCell;
use utils::logging::log_init;
use utils::structs::{Result, WesterError};

#[derive(Parser, PartialEq)]
#[command(name = "rustwester", author, version, about, long_about = None)]
struct Cli {
    /// Host to listen to
    #[arg(short, long, env, global = true, default_value = "0.0.0.0")]
    bind: String,

    /// Service port
    #[arg(short, long, env, global = true, default_value = "9999")]
    port: u16,

    /// Don't allow json response
    #[arg(short = 'j', long, env, global = true)]
    no_json: bool,

    /// Turn debugging information on
    /// repetitive use increases verbosity, at most 2 times
    #[arg(short = 'v', long = "verbose", global = true, action = clap::ArgAction::Count)]
    debug: u8,

    /// Show logging information as json
    #[arg(long, env, global = true)]
    use_json_logging: bool,

    /// Log file location
    #[arg(long, env, global = true)]
    log_file: Option<PathBuf>,
}

struct AppState {
    allow_json: bool,
}

#[derive(Deserialize)]
struct RequestInfo {
    json: Option<String>,
}

static HOSTNAME: OnceCell<String> = OnceCell::const_new();

async fn get_hostname() -> String {
    HOSTNAME
        .get_or_try_init(|| async {
            let hostname = gethostname().to_string_lossy().to_string();
            Ok::<_, WesterError>(hostname)
        })
        .await
        .cloned()
        .unwrap_or("Unknown".to_string())
}

async fn render_markup(
    hostname: &str,
    user_agent: &str,
    hello_str: Option<&str>,
    echo_str: Option<Value>,
) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            title { "Hello world!" }
        }
        body {
            h1 { (hello_str.unwrap_or("Hello world")) " from " (hostname) }
            p { "Your User agent is: " (user_agent) }
            @if let Some(echo_value) = echo_str {
                pre { (format!("{:#}", echo_value)) }
            } @else {
                hr;
            }
        }
    }
}

async fn prepare_response(
    json: bool,
    user_agent: &str,
    hello_str: Option<&str>,
    echo_str: Option<Value>,
) -> impl Responder {
    let hostname = get_hostname().await;
    if json {
        debug!("Returning JSON response");
        let json_response = json!({
            "response": echo_str.unwrap_or(json!(hello_str.unwrap_or("Hello world"))),
            "hostname": hostname,
            "user_agent": user_agent
        });
        HttpResponse::Ok().json(json_response)
    } else {
        debug!("Returning HTML response");
        let html_response = render_markup(&hostname, user_agent, hello_str, echo_str).await;
        HttpResponse::Ok()
            .append_header(header::ContentType::html())
            .body(html_response.into_string())
    }
}

#[get("/")]
async fn hello(
    req: HttpRequest,
    info: web::Query<RequestInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user_agent = req
        .headers()
        .get(header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");

    // Get the 'Accept' header from the request
    let accept_header = req
        .headers()
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok());
    debug!("Accept header: {:?}", accept_header);

    prepare_response(
        data.allow_json
            && (info.json.is_some()
                || accept_header.map_or(false, |v| v.contains("application/json"))),
        user_agent,
        None,
        None,
    )
    .await
}

#[post("/echo")]
async fn echo(
    req: HttpRequest,
    info: web::Query<RequestInfo>,
    req_body: web::Json<serde_json::Value>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let user_agent = req
        .headers()
        .get(header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");

    let parsed = req_body.into_inner();

    // Get the 'Accept' header from the request
    let accept_header = req
        .headers()
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok());
    debug!("Accept header: {:?}", accept_header);

    Ok(prepare_response(
        data.allow_json
            && (info.json.is_some()
                || accept_header.map_or(false, |v| v.contains("application/json"))),
        user_agent,
        None,
        Some(parsed),
    )
    .await)
}

async fn manual_hello(
    req: HttpRequest,
    info: web::Query<RequestInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user_agent = req
        .headers()
        .get(http::header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");

    // Get the 'Accept' header from the request
    let accept_header = req
        .headers()
        .get(http::header::ACCEPT)
        .and_then(|v| v.to_str().ok());
    debug!("Accept header: {:?}", accept_header);

    prepare_response(
        data.allow_json
            && (info.json.is_some()
                || accept_header.map_or(false, |v| v.contains("application/json"))),
        user_agent,
        Some("Hey there!"),
        None,
    )
    .await
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize the logger
    let log_level = match cli.debug {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    log_init(log_level, cli.use_json_logging, cli.log_file)?;

    info!("Starting up...");
    if log_level > LevelFilter::Info {
        info!("Debugging enabled to level {}", log_level);
    }

    info!("Starting server on {}:{}", cli.bind, cli.port);

    // Clone cli.json to move it into the closure
    let json_data = !cli.no_json;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                allow_json: json_data,
            }))
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Version", crate_version!()))
                    .add((header::SERVER, "rustwester"))
                    .add(("X-Powered-By", "actix-web")),
            )
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((cli.bind, cli.port))?
    .run()
    .await?;

    Ok(())
}
