use std::ops::Deref;

use aws_lambda_events::{apigw::ApiGatewayProxyResponse, http::HeaderMap};
use lambda_http::{request, Body};
use lambda_runtime::Error;
use serde::{Deserialize, Serialize};

use crate::RequestPayload;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RequestBody {
    pub code: String,
    pub edition: String,
    pub optimize: String,
    pub version: String,
}

pub async fn execute_code(request: &RequestPayload) -> Result<ApiGatewayProxyResponse, Error> {
    let request_body: RequestBody = if let Some(request_body) = request.body.as_ref() {
        let request_body_as_string = request_body.as_str().unwrap_or_default();
        serde_json::from_str(request_body_as_string).unwrap_or_default()
    } else {
        RequestBody {
            ..Default::default()
        }
    };

    let client = reqwest::Client::new();
    let response = client
        .post("")
        .json::<RequestBody>(&request_body)
        .send()
        .await?;

    let data = response.text().await.unwrap();

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(data)),
        is_base64_encoded: false,
    };

    Ok(resp)
}
