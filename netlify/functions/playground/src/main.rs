use std::collections::HashMap;

use aws_lambda_events::{apigw::ApiGatewayProxyResponse, encodings::Body, http};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RequestPayload {
    pub body: Option<Value>,
    pub path: Option<String>,
    pub headers: Option<HashMap<String, Value>>,
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: bool,
    #[serde(rename = "multiValueHeaders")]
    pub multi_value_headers: Option<HashMap<String, Value>>,
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: Option<Value>,
    #[serde(rename = "rawQuery")]
    pub raw_query: Option<String>,
    #[serde(rename = "rawUrl")]
    pub raw_url: Option<String>,
    #[serde(rename = "requestContext")]
    pub request_context: Option<HashMap<String, Value>>,
    pub resource: Option<String>,
}

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
