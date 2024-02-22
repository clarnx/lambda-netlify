use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    http::{Method, StatusCode},
};
use dotenvy::dotenv;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize};

use serde_json::json;
use shared_lib::{
    database::client::connect_db, utils::cors::cors, AppErrorResponse, AppSuccessResponse,
    RequestPayload,
};

fn from_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Result<String, _> = Deserialize::deserialize(deserializer);
    match s {
        Ok(s) => s.parse::<bool>().map_err(SerdeError::custom),
        Err(_) => Ok(false), // default value
    }
}

fn from_str_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Result<String, _> = Deserialize::deserialize(deserializer);
    match s {
        Ok(s) => s.parse::<i64>().map_err(SerdeError::custom),
        Err(_) => Ok(1), // default value
    }
}

// #[derive(Debug, Serialize, Deserialize, Default)]
// struct RequestPostsQueryParams {
//     #[serde(default, deserialize_with = "from_str_to_bool")]
//     featured: bool,
//     #[serde(default, deserialize_with = "from_str_to_i64")]
//     current_page: i64,
//     #[serde(default)]
//     slug: String,
// }

async fn handler(event: LambdaEvent<RequestPayload>) -> Result<ApiGatewayProxyResponse, Error> {
    let database = connect_db().await?;

    let http_method = event.payload.http_method.unwrap_or_default().to_uppercase();
    let path = event.payload.path.unwrap_or_default();
    // let raw_query = event.payload.raw_query.unwrap_or_default();

    if path.contains("/api/admin") == false {
        return AppErrorResponse::new(StatusCode::NOT_FOUND, Some("Not found".to_owned()), None);
    }

    if http_method == Method::OPTIONS.to_string() {
        return cors();
    }

    // let request_post_query_params =
    //     if let Some(query_params) = event.payload.query_string_parameters {
    //         serde_json::from_value::<RequestPostsQueryParams>(query_params).unwrap_or_default()
    //     } else {
    //         RequestPostsQueryParams {
    //             featured: false,
    //             ..Default::default()
    //         }
    //     };

    let http_method_to_enum = Method::from_bytes(http_method.as_bytes()).unwrap_or_default();
    // dbg!(&request_post_query_params);
    match http_method_to_enum {
        // Method::GET => {
        //     if request_post_query_params.featured == true {
        //         return get_featured_posts(&database).await;
        //     }

        //     if request_post_query_params.featured == false && raw_query.contains("slug=") {
        //         return get_post_by_slug(&database, request_post_query_params.slug).await;
        //     }

        //     return get_posts(&database, Some(request_post_query_params.current_page)).await;
        // },
        Method::POST => AppSuccessResponse::new(
            StatusCode::OK,
            None,
            Some(json!(event.payload.body.clone())),
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
