#[cfg(test)]
mod tests;
mod utils;

use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use clap::{crate_version, Parser};
use gethostname::gethostname;
use log::{debug, info, LevelFilter};
use serde_json::json;
use std::path::PathBuf;
use utils::logging::log_init;
use utils::structs::Result;

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
    html_hello: String,
}

#[get("/")]
async fn hello(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
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

    if data.allow_json && accept_header.map_or(false, |v| v.contains("application/json")) {
        debug!("Returning JSON response");
        let json_response = json!({
            "hello": "world",
            "hostname": gethostname().to_string_lossy().to_string(),
            "user_agent": user_agent
        });
        HttpResponse::Ok().json(json_response)
    } else {
        debug!("Returning HTML response");
        let hostname = gethostname().to_string_lossy().to_string();
        let html_response = data
            .html_hello
            .replace("{{hostname}}", &hostname)
            .replace("{{user_agent}}", user_agent)
            .replace("{{echo}}", "<hr />");
        HttpResponse::Ok()
            .append_header((header::CONTENT_TYPE, mime::TEXT_HTML_UTF_8))
            .body(html_response)
    }
}

#[post("/echo")]
async fn echo(
    req: HttpRequest,
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

    if data.allow_json && accept_header.map_or(false, |v| v.contains("application/json")) {
        debug!("Returning JSON response");
        // Response as JSON
        let json_response = json!({
            "request body": parsed,
            "hostname": gethostname().to_string_lossy().to_string(),
            "user_agent": user_agent
        });
        Ok(HttpResponse::Ok().json(json_response))
    } else {
        debug!("Returning HTML response");
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
        Ok(HttpResponse::Ok()
            .append_header((header::CONTENT_TYPE, mime::TEXT_HTML_UTF_8))
            .body(html_response))
    }
}

async fn manual_hello(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
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

    if data.allow_json && accept_header.map_or(false, |v| v.contains("application/json")) {
        debug!("Returning JSON response");
        HttpResponse::Ok().json(json!({
            "hey": "there",
            "hostname": gethostname().to_string_lossy().to_string(),
            "user_agent": user_agent
        }))
    } else {
        debug!("Returning HTML response");
        let hostname = gethostname().to_string_lossy().to_string();
        let html_response = data
            .html_hello
            .replace("Hello world", "Hey there")
            .replace("{{hostname}}", &hostname)
            .replace("{{user_agent}}", user_agent)
            .replace("{{echo}}", "<hr />");
        HttpResponse::Ok()
            .append_header((header::CONTENT_TYPE, mime::TEXT_HTML_UTF_8))
            .body(html_response)
    }
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
    const HTML_HELLO: &str = include_str!("hello.html");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                allow_json: json_data,
                html_hello: HTML_HELLO.to_string(),
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
