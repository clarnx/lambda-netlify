// use aws_lambda_events::encodings::Body;
// use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
// use aws_lambda_events::http::{self, HeaderMap};
// use lambda_runtime::{service_fn, Context, Error, LambdaEvent};
// use routes::api_routes::api_routes;
// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use std::collections::HashMap;

// mod handlers;
// mod routes;

// #[derive(Debug, Serialize, Deserialize, Clone, Default)]
// pub struct RequestPayload {
//     pub body: Option<Value>,
//     pub path: Option<String>,
//     pub headers: Option<HashMap<String, Value>>,
//     #[serde(rename = "httpMethod")]
//     pub http_method: Option<String>,
//     #[serde(rename = "isBase64Encoded")]
//     pub is_base64_encoded: bool,
//     #[serde(rename = "multiValueHeaders")]
//     pub multi_value_headers: Option<HashMap<String, Value>>,
//     #[serde(rename = "queryStringParameters")]
//     pub query_string_parameters: Option<Value>,
//     #[serde(rename = "rawQuery")]
//     pub raw_query: Option<String>,
//     #[serde(rename = "rawUrl")]
//     pub raw_url: Option<String>,
//     #[serde(rename = "requestContext")]
//     pub request_context: Option<HashMap<String, Value>>,
//     pub resource: Option<String>,
// }

// fn not_found() -> Result<ApiGatewayProxyResponse, Error> {
//     let resp = ApiGatewayProxyResponse {
//         status_code: 404,
//         headers: HeaderMap::new(),
//         multi_value_headers: HeaderMap::new(),
//         body: Some(Body::Text("Not found".to_owned())),
//         is_base64_encoded: false,
//     };

//     Ok(resp)
// }

// async fn init_handler(
//     event: LambdaEvent<RequestPayload>,
// ) -> Result<ApiGatewayProxyResponse, Error> {
//     let request_contains_api_as_base_path = event
//         .payload
//         .clone()
//         .path
//         .unwrap_or_default()
//         .contains("/api");
//     let request_payload = &event.payload.clone();

//     match request_contains_api_as_base_path {
//         true => api_routes(request_payload).await,
//         _ => not_found(),
//     }
// }

// pub async fn init_server() -> Result<(), Error> {
//     let func = service_fn(init_handler);
//     lambda_runtime::run(func).await?;
//     Ok(())
// }
