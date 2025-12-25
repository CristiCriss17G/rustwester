use super::super::*;
use actix_web::{test, App};
use gethostname::gethostname;
use serde_json::json;

#[actix_web::test]
async fn test_hello_json_header() {
    let app_state = web::Data::new(AppState { allow_json: true });

    let app = test::init_service(App::new().app_data(app_state.clone()).service(hello)).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(header::Accept::json())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "application/json"
    );

    let result: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(result["response"], "Hello world");
    assert!(result["hostname"].is_string());
    assert_eq!(result["user_agent"], "Unknown");
}

#[actix_web::test]
async fn test_hello_json_query_param() {
    let app_state = web::Data::new(AppState { allow_json: true });

    let app = test::init_service(App::new().app_data(app_state.clone()).service(hello)).await;

    let req = test::TestRequest::get().uri("/?json").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "application/json"
    );

    let result: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(result["response"], "Hello world");
    assert!(result["hostname"].is_string());
    assert_eq!(result["user_agent"], "Unknown");
}

#[actix_web::test]
async fn test_hello_html_header() {
    let app_state = web::Data::new(AppState { allow_json: false });

    let app = test::init_service(App::new().app_data(app_state.clone()).service(hello)).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(header::Accept::html())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains(gethostname().to_string_lossy().to_string().as_str()));
    assert!(body_str.contains("Unknown"));
    assert!(body_str.contains("<hr>"));
}

#[actix_web::test]
async fn test_echo_json_header() {
    let app_state = web::Data::new(AppState { allow_json: true });

    let app = test::init_service(App::new().app_data(app_state.clone()).service(echo)).await;

    let req_body = json!({"key": "value"});
    let req = test::TestRequest::post()
        .uri("/echo")
        .insert_header(header::Accept::json())
        .set_json(&req_body)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "application/json"
    );

    let result: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(result["response"], req_body);
    assert!(result["hostname"].is_string());
    assert_eq!(result["user_agent"], "Unknown");
}

#[actix_web::test]
async fn test_echo_json_query_param() {
    let app_state = web::Data::new(AppState { allow_json: true });

    let app = test::init_service(App::new().app_data(app_state.clone()).service(echo)).await;

    let req_body = json!({"key": "value"});
    let req = test::TestRequest::post()
        .uri("/echo?json")
        .set_json(&req_body)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "application/json"
    );

    let result: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(result["response"], req_body);
    assert!(result["hostname"].is_string());
    assert_eq!(result["user_agent"], "Unknown");
}

#[actix_web::test]
async fn test_echo_html() {
    let app_state = web::Data::new(AppState { allow_json: false });

    let app = test::init_service(App::new().app_data(app_state.clone()).service(echo)).await;

    let req_body = json!({"key": "value"});
    let req = test::TestRequest::post()
        .uri("/echo")
        .set_json(&req_body)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains(gethostname().to_string_lossy().to_string().as_str()));
    assert!(body_str.contains("Unknown"));
    assert!(body_str.contains("<pre>{\n  &quot;key&quot;: &quot;value&quot;\n}</pre>"));
}

#[actix_web::test]
async fn test_manual_hello_json_header() {
    let app_state = web::Data::new(AppState { allow_json: true });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/hey", web::get().to(manual_hello)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/hey")
        .insert_header(header::Accept::json())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "application/json"
    );

    let result: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(result["response"], "Hey there!");
    assert!(result["hostname"].is_string());
    assert_eq!(result["user_agent"], "Unknown");
}

#[actix_web::test]
async fn test_manual_hello_json_query_param() {
    let app_state = web::Data::new(AppState { allow_json: true });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/hey", web::get().to(manual_hello)),
    )
    .await;

    let req = test::TestRequest::get().uri("/hey?json").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "application/json"
    );

    let result: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(result["response"], "Hey there!");
    assert!(result["hostname"].is_string());
    assert_eq!(result["user_agent"], "Unknown");
}

#[actix_web::test]
async fn test_manual_hello_html() {
    let app_state = web::Data::new(AppState { allow_json: false });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/hey", web::get().to(manual_hello)),
    )
    .await;

    let req = test::TestRequest::get().uri("/hey").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();

    // println!("{}", body_str);
    assert!(body_str.contains("Hey there"));
    assert!(body_str.contains(gethostname().to_string_lossy().to_string().as_str()));
    assert!(body_str.contains("Unknown"));
    assert!(body_str.contains("<hr>"));
}

#[actix_web::test]
async fn test_hello_json_header_no_json() {
    let app_state = web::Data::new(AppState { allow_json: false });

    let app = test::init_service(App::new().app_data(app_state.clone()).service(hello)).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header(header::Accept::json())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );
}

#[actix_web::test]
async fn test_hello_json_query_param_no_json() {
    let app_state = web::Data::new(AppState { allow_json: false });

    let app = test::init_service(App::new().app_data(app_state.clone()).service(hello)).await;

    let req = test::TestRequest::get().uri("/?json").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );
}

#[actix_web::test]
async fn test_echo_json_header_no_json() {
    let app_state = web::Data::new(AppState { allow_json: false });

    let app = test::init_service(App::new().app_data(app_state.clone()).service(echo)).await;

    let req_body = json!({"key": "value"});
    let req = test::TestRequest::post()
        .uri("/echo")
        .insert_header(header::Accept::json())
        .set_json(&req_body)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );
}

#[actix_web::test]
async fn test_echo_json_query_param_no_json() {
    let app_state = web::Data::new(AppState { allow_json: false });

    let app = test::init_service(App::new().app_data(app_state.clone()).service(echo)).await;

    let req_body = json!({"key": "value"});
    let req = test::TestRequest::post()
        .uri("/echo?json")
        .set_json(&req_body)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );
}

#[actix_web::test]
async fn test_manual_hello_json_header_no_json() {
    let app_state = web::Data::new(AppState { allow_json: false });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/hey", web::get().to(manual_hello)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/hey")
        .insert_header(header::Accept::json())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );
}

#[actix_web::test]
async fn test_manual_hello_json_query_param_no_json() {
    let app_state = web::Data::new(AppState { allow_json: false });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/hey", web::get().to(manual_hello)),
    )
    .await;

    let req = test::TestRequest::get().uri("/hey?json").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );
}

#[actix_web::test]
async fn test_echo_form_get() {
    let app = test::init_service(App::new().service(echo_form)).await;

    let req = test::TestRequest::get().uri("/echo").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers = resp.headers();
    assert_eq!(
        headers.get(header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();

    // Verify the form structure exists
    assert!(body_str.contains("<form id=\"echo-form\""));
    assert!(body_str.contains("<textarea id=\"payload\""));
    assert!(body_str.contains("<button type=\"submit\""));
    assert!(body_str.contains("Echo"));
    assert!(body_str.contains("JSON payload"));
    assert!(body_str.contains("Hello from form"));
}

#[actix_web::test]
async fn test_echo_form_contains_script() {
    let app = test::init_service(App::new().service(echo_form)).await;

    let req = test::TestRequest::get().uri("/echo").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();

    // Verify JavaScript code is present
    assert!(body_str.contains("fetch('/echo'"));
    assert!(body_str.contains("method: 'POST'"));
    assert!(body_str.contains("'Accept': 'application/json'"));
}

#[actix_web::test]
async fn test_render_markup_with_hello() {
    let markup = super::super::render_markup("test-host", "Mozilla", Some("Hi"), None).await;
    let html_str = markup.into_string();

    assert!(html_str.contains("Hi from test-host"));
    assert!(html_str.contains("Your User agent is: Mozilla"));
    assert!(html_str.contains("<hr>"));
    assert!(!html_str.contains("<pre>"));
}

#[actix_web::test]
async fn test_render_markup_with_echo() {
    let echo_value = json!({"key": "value"});
    let markup = super::super::render_markup("my-host", "Chrome", None, Some(echo_value)).await;
    let html_str = markup.into_string();

    assert!(html_str.contains("Hello world from my-host"));
    assert!(html_str.contains("Your User agent is: Chrome"));
    assert!(html_str.contains("<pre>"));
    assert!(html_str.contains("key"));
    assert!(html_str.contains("value"));
    assert!(!html_str.contains("<hr>"));
}

#[actix_web::test]
async fn test_render_markup_with_both() {
    let echo_value = json!({"test": 123});
    let markup =
        super::super::render_markup("prod-server", "Safari", Some("Welcome"), Some(echo_value))
            .await;
    let html_str = markup.into_string();

    assert!(html_str.contains("Welcome from prod-server"));
    assert!(html_str.contains("Your User agent is: Safari"));
    assert!(html_str.contains("<pre>"));
    assert!(html_str.contains("test"));
    assert!(html_str.contains("123"));
}

#[actix_web::test]
async fn test_render_markup_html_structure() {
    let markup = super::super::render_markup("localhost", "Test", None, None).await;
    let html_str = markup.into_string();

    // Check for proper HTML structure
    assert!(html_str.contains("<!DOCTYPE"));
    assert!(html_str.contains("<head>"));
    assert!(html_str.contains("<body>"));
    assert!(html_str.contains("<meta charset=\"utf-8\""));
    assert!(html_str.contains("<meta name=\"viewport\""));
    assert!(html_str.contains("<title>"));
}

#[actix_web::test]
async fn test_get_hostname() {
    let hostname = super::super::get_hostname().await;

    // Hostname should not be empty
    assert!(!hostname.is_empty());

    // Should match actual hostname from gethostname
    let expected = gethostname().to_string_lossy().to_string();
    assert_eq!(hostname, expected);
}

#[actix_web::test]
async fn test_get_hostname_cached() {
    let hostname1 = super::super::get_hostname().await;
    let hostname2 = super::super::get_hostname().await;

    // Both calls should return the same value (testing the OnceCell caching)
    assert_eq!(hostname1, hostname2);
}
