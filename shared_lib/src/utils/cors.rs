use std::env;

use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    encodings::Body,
    http::{HeaderMap, StatusCode},
};
use lambda_runtime::Error;

pub fn cors() -> Result<ApiGatewayProxyResponse, Error> {
    let frontend_base_url = env::var("FRONTEND_BASE_URL").unwrap_or_default();

    let status_as_i64: i64 = StatusCode::OK.as_u16() as i64;

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert(
        "Access-Control-Allow-Origin",
        frontend_base_url.parse().unwrap(),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        "POST, GET, OPTIONS".parse().unwrap(),
    );
    headers.insert("Access-Control-Allow-Headers", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Credentials", "true".parse().unwrap());

    // Preflight request. Reply successfully:
    Ok(ApiGatewayProxyResponse {
        status_code: status_as_i64,
        multi_value_headers: headers.clone(),
        headers,
        body: Some(Body::Text("".to_owned())),
        is_base64_encoded: false,
    })
}
