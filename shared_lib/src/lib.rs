pub mod database;
pub mod models;
pub mod traits;
pub mod utils;

use std::{clone, collections::HashMap, env, ops::Deref};

use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    encodings::Body,
    http::{HeaderMap, StatusCode},
};
use lambda_runtime::{tower::util::error, Error};
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RequestPayload {
    pub body: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PaginationMetadata {
    current_page: Option<u64>,
    total_pages: Option<u64>,
    total_items: Option<u64>,
    items_per_page: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PaginatedData {
    pub documents: Vec<Document>,
    pub metadata: PaginationMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
    status: ResponseStatus,
    message: Option<String>,
    data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppSuccessResponse {}

impl AppSuccessResponse {
    pub fn new(
        status_code: StatusCode,
        message: Option<String>,
        data: Option<Value>,
    ) -> Result<ApiGatewayProxyResponse, Error> {
        let frontend_base_url = env::var("FRONTEND_BASE_URL").unwrap_or_default();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Access-Control-Allow-Origin",
            frontend_base_url.parse().unwrap(),
        );
        headers.insert("Access-Control-Allow-Credentials", "true".parse().unwrap());

        let status_as_i64: i64 = status_code.as_u16() as i64;

        let response_body = ResponseBody {
            status: ResponseStatus::Success,
            message,
            data,
        };

        let response_body_json = serde_json::to_string(&response_body).unwrap_or_default();

        Ok(ApiGatewayProxyResponse {
            status_code: status_as_i64,
            multi_value_headers: headers.clone(),
            headers,
            body: Some(Body::Text(response_body_json)),
            is_base64_encoded: false,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppErrorResponse {}

impl AppErrorResponse {
    pub fn new(
        status_code: StatusCode,
        message: Option<String>,
        data: Option<Value>,
    ) -> Result<ApiGatewayProxyResponse, Error> {
        let frontend_base_url = env::var("FRONTEND_BASE_URL").unwrap_or_default();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Access-Control-Allow-Origin",
            frontend_base_url.parse().unwrap(),
        );
        headers.insert("Access-Control-Allow-Credentials", "true".parse().unwrap());

        let status_as_i64: i64 = status_code.as_u16() as i64;

        let response_body = ResponseBody {
            status: ResponseStatus::Error,
            message,
            data,
        };

        let response_body_json = serde_json::to_string(&response_body).unwrap_or_default();

        Ok(ApiGatewayProxyResponse {
            status_code: status_as_i64,
            multi_value_headers: headers.clone(),
            headers,
            body: Some(Body::Text(response_body_json)),
            is_base64_encoded: false,
        })
    }
}

#[derive(Debug)]
pub enum DataInsertError {
    FieldValidationError(validator::ValidationErrors),
    MongoDuplicateError(mongodb::error::WriteError),
    MongoWriteError(mongodb::error::WriteError),
    OtherMongoError(mongodb::error::Error),
}

impl From<validator::ValidationErrors> for DataInsertError {
    fn from(error: validator::ValidationErrors) -> Self {
        Self::FieldValidationError(error)
    }
}

impl From<mongodb::error::Error> for DataInsertError {
    fn from(error: mongodb::error::Error) -> Self {
        match error.kind.as_ref() {
            mongodb::error::ErrorKind::Write(mongodb::error::WriteFailure::WriteError(error)) => {
                if error.code == 11000 {
                    Self::MongoDuplicateError(error.clone())
                } else {
                    Self::MongoWriteError(error.clone())
                }
            }

            _ => Self::OtherMongoError(error),
        }
    }
}

// impl From<mongodb::error::Error> for DataInsertError {
//     fn from(error: mongodb::error::Error) -> Self {
//         match error.kind.as_ref() {
//             mongodb::error::ErrorKind::Write(mongodb::error::WriteFailure::WriteError(
//                 write_error,
//             )) if write_error.code == 11000 => Self::MongoDuplicateError(error),
//             _ => Self::OtherMongoErrors(error),
//         }
//     }
// }
