use aws_lambda_events::{apigw::ApiGatewayProxyResponse, http::StatusCode};

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::json;
use shared_lib::{AppSuccessResponse, RequestPayload};

async fn handler(event: LambdaEvent<RequestPayload>) -> Result<ApiGatewayProxyResponse, Error> {
    AppSuccessResponse::new(
        StatusCode::OK,
        Some("Request successful".to_owned()),
        Some(json!({"path": event.payload.path})),
    )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}
