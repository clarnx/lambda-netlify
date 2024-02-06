// use lambda_runtime::{service_fn, Error, LambdaEvent};
// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use serde_json::Value;

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let func = service_fn(func);
//     lambda_runtime::run(func).await?;
//     Ok(())
// }

// struct Request {
//     #[serde("ren")]
// multiValueQueryStringParameters: Option<Value>,
// "path": "/.netlify/functions/hello-world",
// "queryStringParameters": {},
// "rawQuery": "",
// "rawUrl": "https://bespoke-sable-36b18d.netlify.app/.netlify/functions/hello-world"
// }

// async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
//     let (parts, body) = event.into_parts();

//     let response = json!({
//         "statusCode": 200,
//         "headers": {
//             "Content-Type": "application/json"
//         },
//         "body": serde_json::to_string(&json!({"message": "success", "parts": parts, "body": body})).unwrap(),
//         "isBase64Encoded": false
//     });

//     Ok(response)
// }
// use lambda_runtime::{service_fn, LambdaEvent, Error};
// use serde_json::{json, Value};

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let func = service_fn(func);
//     lambda_runtime::run(func).await?;
//     Ok(())
// }

// async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
//     let (event, _context) = event.into_parts();
//     let first_name = event["firstName"].as_str().unwrap_or("world");

//     Ok(json!({ "message": format!("Hello, {}!", first_name) }))
// }


// use lambda_http::{http::{StatusCode, Response}, run, service_fn, Error, IntoResponse, Request, RequestExt, RequestPayloadExt};
// use serde::{Deserialize, Serialize};
// use serde_json::json;

// #[tokio::main]
// async fn main() -> Result<(), Error> {
    
//     run(service_fn(function_handler)).await
// }

// pub async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
//    let name = event.uri().path_and_query().unwrap().as_str();

// //    dbg!(name);

//     let response = Response::builder()
//         .status(StatusCode::OK)
//         .header("Content-Type", "application/json")
//         .body(json!({
//             "message": "Hello World",
//             "payload": name,
//           }).to_string())
//         .map_err(Box::new)?;

//     Ok(response)
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct MyPayload {
//     pub prop1: String,
//     pub prop2: String,
// }

//! This is an example function that leverages the Lambda Rust runtime HTTP support
//! and the [axum](https://docs.rs/axum/latest/axum/index.html) web framework.  The
//! runtime HTTP support is backed by the [tower::Service](https://docs.rs/tower-service/0.3.2/tower_service/trait.Service.html)
//! trait.  Axum's applications are also backed by the `tower::Service` trait.  That means
//! that it is fairly easy to build an Axum application and pass the resulting `Service`
//! implementation to the Lambda runtime to run as a Lambda function.  By using Axum instead
//! of a basic `tower::Service` you get web framework niceties like routing, request component
//! extraction, validation, etc.
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env::set_var;

#[derive(Deserialize, Serialize)]
struct Params {
    first: Option<String>,
    second: Option<String>,
}

async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /" }))
}

async fn get_foo() -> Json<Value> {
    Json(json!({ "msg": "I am GET /foo" }))
}

async fn post_foo() -> Json<Value> {
    Json(json!({ "msg": "I am POST /foo" }))
}

async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("I am POST /foo/:name, name={name}") }))
}

async fn get_parameters(Query(params): Query<Params>) -> Json<Value> {
    Json(json!({ "request parameters": params }))
}

/// Example on how to return status codes and data from an Axum function
async fn health_check() -> (StatusCode, String) {
    let health = true;
    match health {
        true => (StatusCode::OK, "Healthy!".to_string()),
        false => (StatusCode::INTERNAL_SERVER_ERROR, "Not healthy!".to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // If you use API Gateway stages, the Rust Runtime will include the stage name
    // as part of the path that your application receives.
    // Setting the following environment variable, you can remove the stage from the path.
    // This variable only applies to API Gateway stages,
    // you can remove it if you don't use them.
    // i.e with: `GET /test-stage/todo/id/123` without: `GET /todo/id/123`
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");


    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/:name", post(post_foo_name))
        .route("/parameters", get(get_parameters))
        .route("/health/", get(health_check));

    run(app).await
}