// use lambda_http::Response;
// use lambda_runtime::{service_fn, Error, LambdaEvent};
// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use serde_json::Value;

// #[tokio::main]
// async fn main() -> Result<Value, Error> {
//     let func = service_fn(func);
//     lambda_runtime::run(func).await?;
//     Ok(())
// }

// // struct Request {
// //     #[serde("ren")]
// // multiValueQueryStringParameters: Option<Value>,
// // "path": "/.netlify/functions/hello-world",
// // "queryStringParameters": {},
// // "rawQuery": "",
// // "rawUrl": "https://bespoke-sable-36b18d.netlify.app/.netlify/functions/hello-world"
// // }

// async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
//     let (parts, body) = event.into_parts();

//     let response = Response::builder().status(200).body("body").unwrap();

//     Ok(serde_json::to_string(response))
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

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     // If you use API Gateway stages, the Rust Runtime will include the stage name
//     // as part of the path that your application receives.
//     // Setting the following environment variable, you can remove the stage from the path.
//     // This variable only applies to API Gateway stages,
//     // you can remove it if you don't use them.
//     // i.e with: `GET /test-stage/todo/id/123` without: `GET /todo/id/123`
//     set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

//     let app = Router::new()
//         .route("/", get(root))
//         .route("/foo", get(get_foo).post(post_foo))
//         .route("/foo/:name", post(post_foo_name))
//         .route("/parameters", get(get_parameters))
//         .route("/health/", get(health_check));

//     run(app).await
// }

// use lambda_runtime::{service_fn, Error, LambdaEvent};
// use serde_json::{json, Value};

// async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
//     let payload = event.payload;
//     let first_name = payload["firstName"].as_str().unwrap_or("world");
//     Ok(
//         json!({ "message": format!("Hello, {first_name}!"), "statusCode": 200,  "headers": {
//             "Content-Type": "application/json"
//         }, }),
//     )
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     lambda_runtime::run(service_fn(handler)).await
// }

use lambda_runtime::Error;

use rust_digest::init_server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_server().await?;
    Ok(())
}
