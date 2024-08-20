use super::super::*;
use actix_web::{test, App};
use gethostname::gethostname;
use serde_json::json;

#[actix_web::test]
async fn test_hello_json_header() {
    let app_state = web::Data::new(AppState {
        allow_json: true,
        html_hello: String::new(),
    });

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
    let app_state = web::Data::new(AppState {
        allow_json: true,
        html_hello: String::new(),
    });

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
    let app_state = web::Data::new(AppState {
        allow_json: false,
        html_hello: String::from("<html><body>{{hostname}} {{user_agent}} {{echo}}</body></html>"),
    });

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
    assert!(body_str.contains(format!("{}", gethostname().to_string_lossy().to_string()).as_str()));
    assert!(body_str.contains("Unknown"));
    assert!(body_str.contains("<hr />"));
}

#[actix_web::test]
async fn test_echo_json_header() {
    let app_state = web::Data::new(AppState {
        allow_json: true,
        html_hello: String::new(),
    });

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
    let app_state = web::Data::new(AppState {
        allow_json: true,
        html_hello: String::new(),
    });

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
    let app_state = web::Data::new(AppState {
        allow_json: false,
        html_hello: String::from("<html><body>{{hostname}} {{user_agent}} {{echo}}</body></html>"),
    });

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
    assert!(body_str.contains(format!("{}", gethostname().to_string_lossy().to_string()).as_str()));
    assert!(body_str.contains("Unknown"));
    assert!(body_str.contains("<pre>{\"key\":\"value\"}</pre>"));
}

#[actix_web::test]
async fn test_manual_hello_json_header() {
    let app_state = web::Data::new(AppState {
        allow_json: true,
        html_hello: String::new(),
    });

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
    let app_state = web::Data::new(AppState {
        allow_json: true,
        html_hello: String::new(),
    });

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
    let app_state = web::Data::new(AppState {
        allow_json: false,
        html_hello: String::from(
            "<html><body>Hello world {{hostname}} {{user_agent}} {{echo}}</body></html>",
        ),
    });

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
    assert!(body_str.contains(format!("{}", gethostname().to_string_lossy().to_string()).as_str()));
    assert!(body_str.contains("Unknown"));
    assert!(body_str.contains("<hr />"));
}

#[actix_web::test]
async fn test_hello_json_header_no_json() {
    let app_state = web::Data::new(AppState {
        allow_json: false,
        html_hello: String::new(),
    });

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
    let app_state = web::Data::new(AppState {
        allow_json: false,
        html_hello: String::new(),
    });

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
    let app_state = web::Data::new(AppState {
        allow_json: false,
        html_hello: String::new(),
    });

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
    let app_state = web::Data::new(AppState {
        allow_json: false,
        html_hello: String::new(),
    });

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
    let app_state = web::Data::new(AppState {
        allow_json: false,
        html_hello: String::new(),
    });

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
    let app_state = web::Data::new(AppState {
        allow_json: false,
        html_hello: String::new(),
    });

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
