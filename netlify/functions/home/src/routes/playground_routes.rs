use std::ops::Deref;

use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    http::{HeaderMap, Method},
};
use lambda_http::Body;
use lambda_runtime::Error;

use crate::{
    handlers::playground_handler::{self, execute_code},
    not_found, RequestPayload,
};

pub async fn playground_routes(request: &RequestPayload) -> Result<ApiGatewayProxyResponse, Error> {
    let request_path = request.path.as_deref().unwrap_or_default();
    let request_method = request
        .http_method
        .as_deref()
        .unwrap_or_default()
        .to_uppercase();

    match request_method.as_str() {
        "POST" => execute_code(request).await,
        _ => not_found(),
    }
}
