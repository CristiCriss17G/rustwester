#[cfg(test)]
mod tests;
mod utils;

use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use clap::{crate_version, Parser};
use gethostname::gethostname;
use log::{info, LevelFilter};
use serde_json::json;
use utils::{logging::log_init, structs::WesterError};

#[derive(Parser, PartialEq)]
#[command(name = "rustwester", author, version, about, long_about = None)]
struct Cli {
    /// Host to listen to
    #[arg(short, long, env, global = true, default_value = "0.0.0.0")]
    bind: String,

    /// Service port
    #[arg(short, long, env, global = true, default_value = "9999")]
    port: u16,

    /// Json response
    #[arg(short, long, env, global = true)]
    json: bool,

    /// Turn debugging information on
    /// repetitive use increases verbosity, at most 2 times
    #[arg(short='v', long="verbose", global = true, action = clap::ArgAction::Count)]
    debug: u8,

    /// Show logging information as json
    #[arg(long, env, global = true)]
    use_json_logging: bool,
}

struct AppState {
    json: bool,
    html_hello: String,
}

#[get("/")]
async fn hello(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let user_agent = req
        .headers()
        .get(http::header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");

    if data.json {
        let json_response = json!({
            "hello": "world",
            "hostname": gethostname().to_string_lossy().to_string(),
            "user_agent": user_agent
        });
        HttpResponse::Ok().json(json_response)
    } else {
        let hostname = gethostname().to_string_lossy().to_string();
        let html_response = data
            .html_hello
            .replace("{{hostname}}", &hostname)
            .replace("{{user_agent}}", user_agent)
            .replace("{{echo}}", "<hr />");
        HttpResponse::Ok().body(html_response)
    }
}

#[post("/echo")]
async fn echo(
    req: HttpRequest,
    req_body: web::Json<serde_json::Value>,
    data: web::Data<AppState>,
) -> Result<impl Responder, WesterError> {
    let user_agent = req
        .headers()
        .get(http::header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");

    let parsed = req_body.into_inner();

    if data.json {
        // Response as JSON
        let json_response = json!({
            "request body": parsed,
            "hostname": gethostname().to_string_lossy().to_string(),
            "user_agent": user_agent
        });
        Ok(HttpResponse::Ok().json(json_response))
    } else {
        // Response as text
        let hostname = gethostname().to_string_lossy().to_string();
        let html_response = data
            .html_hello
            .replace("{{hostname}}", &hostname)
            .replace("{{user_agent}}", user_agent)
            .replace(
                "{{echo}}",
                format!("<pre>{}</pre>\n    <hr />", parsed).as_str(),
            );
        Ok(HttpResponse::Ok().body(html_response))
    }
}

async fn manual_hello(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let user_agent = req
        .headers()
        .get(http::header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");

    if data.json {
        HttpResponse::Ok().json(json!({
            "hey": "there",
            "hostname": gethostname().to_string_lossy().to_string(),
            "user_agent": user_agent
        }))
    } else {
        let hostname = gethostname().to_string_lossy().to_string();
        let html_response = data
            .html_hello
            .replace("Hello world", "Hey there")
            .replace("{{hostname}}", &hostname)
            .replace("{{user_agent}}", user_agent)
            .replace("{{echo}}", "<hr />");
        HttpResponse::Ok().body(html_response)
    }
}

#[tokio::main]
async fn main() -> Result<(), WesterError> {
    let cli = Cli::parse();

    // Initialize the logger
    let log_level = match cli.debug {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    log_init(log_level, cli.use_json_logging);

    info!("Starting up...");
    if log_level > LevelFilter::Info {
        info!("Debugging enabled to level {}", log_level);
    }

    info!("Starting server on {}:{}", cli.bind, cli.port);

    // Clone cli.json to move it into the closure
    let json_data = cli.json.clone();
    const HTML_HELLO: &str = include_str!("hello.html");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                json: json_data,
                html_hello: HTML_HELLO.to_string(),
            }))
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Version", crate_version!()))
                    .add((
                        header::CONTENT_TYPE,
                        format!(
                            "{}; charset=utf-8",
                            if json_data {
                                "application/json"
                            } else {
                                "text/html"
                            }
                        ),
                    ))
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
