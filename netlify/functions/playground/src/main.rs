use aws_lambda_events::{apigw::ApiGatewayProxyResponse, encodings::Body, http};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use shared_lib::RequestPayload;

async fn handler(event: LambdaEvent<RequestPayload>) -> Result<ApiGatewayProxyResponse, Error> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        multi_value_headers: headers.clone(),
        is_base64_encoded: false,
        body: Some(Body::Text(
            serde_json::to_string(&event.payload.path).unwrap(),
        )),
        headers,
    };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}
