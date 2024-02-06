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

use std::collections::HashMap;

use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::http::{self, HeaderMap};
use lambda_runtime::{service_fn, Context, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct RequestPayload {
    body: Option<Value>,
    path: Option<String>,
    headers: Option<HashMap<String, Value>>,
    #[serde(rename = "httpMethod")]
    http_method: Option<String>,
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "multiValueHeaders")]
    multi_value_headers: Option<HashMap<String, Value>>,
    #[serde(rename = "queryStringParameters")]
    query_string_parameters: Option<Value>,
    #[serde(rename = "rawQuery")]
    raw_query: Option<String>,
    #[serde(rename = "rawUrl")]
    raw_url: Option<String>,
    #[serde(rename = "requestContext")]
    request_context: Option<HashMap<String, Value>>,
    resource: Option<String>,
}

async fn my_handler(event: LambdaEvent<RequestPayload>) -> Result<ApiGatewayProxyResponse, Error> {
    let request = serde_json::to_string(&event.payload).unwrap();

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(request)),
        is_base64_encoded: false,
    };

    Ok(resp)
}