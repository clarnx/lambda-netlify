use aws_lambda_events::{apigw::ApiGatewayProxyResponse, http::HeaderMap};
use lambda_http::Body;
use lambda_runtime::Error;

use crate::{not_found, RequestPayload};

use super::playground_routes::playground_routes;

pub async fn api_routes(request: &RequestPayload) -> Result<ApiGatewayProxyResponse, Error> {
    match request.path.as_ref().unwrap().as_str() {
        "/api/playground" => playground_routes(request).await,
        _ => not_found(),
    }
}
