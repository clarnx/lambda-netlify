use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, IntoResponse, Request, RequestExt, RequestPayloadExt,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

pub async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let (parts, body) = event.into_parts();

    let http_path = parts.raw_http_path();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "Hello World",
              "payload": http_path,
              "body": body
            })
            .to_string(),
        )
        .map_err(Box::new)?;

    Ok(response)
}
