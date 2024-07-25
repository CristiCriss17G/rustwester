use super::super::*;
use actix_web::{test, App};
use gethostname::gethostname;
use serde_json::json;

#[actix_web::test]
async fn test_hello_json() {
    let app_state = web::Data::new(AppState {
        json: true,
        html_hello: String::new(),
    });

    let mut app = test::init_service(App::new().app_data(app_state.clone()).service(hello)).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let result: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(result["hello"], "world");
    assert!(result["hostname"].is_string());
    assert!(result["user_agent"].is_string());
}

#[actix_web::test]
async fn test_hello_html() {
    let app_state = web::Data::new(AppState {
        json: false,
        html_hello: String::from("<html><body>{{hostname}} {{user_agent}} {{echo}}</body></html>"),
    });

    let mut app = test::init_service(App::new().app_data(app_state.clone()).service(hello)).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains(format!("{}", gethostname().to_string_lossy().to_string()).as_str()));
    assert!(body_str.contains("Unknown"));
    assert!(body_str.contains("<hr />"));
}

#[actix_web::test]
async fn test_echo_json() {
    let app_state = web::Data::new(AppState {
        json: true,
        html_hello: String::new(),
    });

    let mut app = test::init_service(App::new().app_data(app_state.clone()).service(echo)).await;

    let req_body = json!({"key": "value"});
    let req = test::TestRequest::post()
        .uri("/echo")
        .set_json(&req_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let result: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(result["request body"], req_body);
    assert!(result["hostname"].is_string());
    assert!(result["user_agent"].is_string());
}

#[actix_web::test]
async fn test_echo_html() {
    let app_state = web::Data::new(AppState {
        json: false,
        html_hello: String::from("<html><body>{{hostname}} {{user_agent}} {{echo}}</body></html>"),
    });

    let mut app = test::init_service(App::new().app_data(app_state.clone()).service(echo)).await;

    let req_body = json!({"key": "value"});
    let req = test::TestRequest::post()
        .uri("/echo")
        .set_json(&req_body)
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains(format!("{}", gethostname().to_string_lossy().to_string()).as_str()));
    assert!(body_str.contains("Unknown"));
    assert!(body_str.contains("<pre>{\"key\":\"value\"}</pre>"));
}

#[actix_web::test]
async fn test_manual_hello_json() {
    let app_state = web::Data::new(AppState {
        json: true,
        html_hello: String::new(),
    });

    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/hey", web::get().to(manual_hello)),
    )
    .await;

    let req = test::TestRequest::get().uri("/hey").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let json_body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json_body["hey"], "there");
    assert!(json_body["hostname"].is_string());
    assert!(json_body["user_agent"].is_string());
}

#[actix_web::test]
async fn test_manual_hello_html() {
    let app_state = web::Data::new(AppState {
        json: false,
        html_hello: String::from(
            "<html><body>Hello world {{hostname}} {{user_agent}} {{echo}}</body></html>",
        ),
    });

    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/hey", web::get().to(manual_hello)),
    )
    .await;

    let req = test::TestRequest::get().uri("/hey").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();

    println!("{}", body_str);
    assert!(body_str.contains("Hey there"));
    assert!(body_str.contains(format!("{}", gethostname().to_string_lossy().to_string()).as_str()));
    assert!(body_str.contains("Unknown"));
    assert!(body_str.contains("<hr />"));
}
