use aws_lambda_events::{apigw::ApiGatewayProxyResponse, http::HeaderMap};
use lambda_http::Body;
use lambda_runtime::Error;

pub async fn execute_code() -> Result<ApiGatewayProxyResponse, Error> {
    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(serde_json::to_string("playground").unwrap())),
        is_base64_encoded: false,
    };

    Ok(resp)
}
