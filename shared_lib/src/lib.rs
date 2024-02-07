use std::collections::HashMap;

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
