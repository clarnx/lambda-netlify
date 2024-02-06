use aws_lambda_events::{apigw::ApiGatewayProxyResponse, http::{HeaderMap}};
use lambda_http::Body;
use lambda_runtime::Error;

use crate::{handlers::playground_handler::{self, execute_code}, RequestPayload};

pub async fn playground_routes(request: &RequestPayload) -> Result<ApiGatewayProxyResponse, Error> {
    let request_path = request.path.as_ref().unwrap();

    execute_code().await
    
}