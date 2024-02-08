use std::env;

use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    http::{Method, StatusCode},
};
use dotenvy::dotenv;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Value};
use shared_lib::{utils::cors::cors, AppErrorResponse, AppSuccessResponse, RequestPayload};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestBody {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RustCodeExecuteRequestData {
    version: String,
    optimize: String,
    pub code: String,
    edition: String,
}

impl RustCodeExecuteRequestData {
    fn new(code: String) -> Self {
        Self {
            version: "stable".to_owned(),
            optimize: "0".to_owned(),
            code,
            edition: "2021".to_owned(),
        }
    }
}

async fn handler(event: LambdaEvent<RequestPayload>) -> Result<ApiGatewayProxyResponse, Error> {
    let http_method = event.payload.http_method.unwrap_or_default().to_uppercase();
    let path = event.payload.path.unwrap_or_default();

    if path != "/api/playground/execute-code" {
        return AppErrorResponse::new(StatusCode::NOT_FOUND, Some("Not found".to_owned()), None);
    }

    if http_method == Method::OPTIONS.to_string() {
        return cors();
    }

    let rust_code_execution_url = env::var("RUST_CODE_EXECUTION_URL").unwrap_or_default();

    let rust_code_from_request = if let Some(json_body) = event.payload.body {
        let body_from_json: RequestBody =
            serde_json::from_str(json_body.as_str()).unwrap_or_default();
        let code = body_from_json.code;
        code
    } else {
        return AppErrorResponse::new(
            StatusCode::BAD_REQUEST,
            Some("Provide rust code".to_owned()),
            None,
        );
    };

    let rust_code_execute_request_data =
        RustCodeExecuteRequestData::new(rust_code_from_request.to_owned());

    let client = reqwest::Client::new();
    let response = client
        .post(rust_code_execution_url)
        .json::<RustCodeExecuteRequestData>(&rust_code_execute_request_data)
        .send()
        .await?;

    let data: Value = response.json().await.unwrap();
    let http_method_to_enum = Method::from_bytes(http_method.as_bytes()).unwrap_or_default();

    match http_method_to_enum {
        Method::POST => AppSuccessResponse::new(
            StatusCode::OK,
            Some("Request successful".to_owned()),
            Some(data),
        ),
        _ => AppErrorResponse::new(
            StatusCode::NOT_ACCEPTABLE,
            Some("Not acceptable".to_owned()),
            None,
        ),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().unwrap_or_default();
    lambda_runtime::run(service_fn(handler)).await
}
