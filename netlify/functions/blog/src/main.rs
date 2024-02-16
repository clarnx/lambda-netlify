use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    http::{Method, StatusCode},
};
use blog::handlers::post_handler::{get_featured_posts, get_posts};
use dotenvy::dotenv;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use mongodb::bson::oid::ObjectId;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize};

use serde_json::Value;
use shared_lib::{
    database::client::connect_db, models::post::Post, utils::cors::cors, AppErrorResponse,
    AppSuccessResponse, RequestPayload,
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

#[derive(Debug, Serialize, Deserialize, Default)]
struct RequestPostsQueryParams {
    #[serde(default, deserialize_with = "from_str_to_bool")]
    featured: bool,
    #[serde(default, deserialize_with = "from_str_to_i64")]
    current_page: i64,
}

async fn handler(event: LambdaEvent<RequestPayload>) -> Result<ApiGatewayProxyResponse, Error> {
    let database = connect_db().await?;

    let http_method = event.payload.http_method.unwrap_or_default().to_uppercase();
    let path = event.payload.path.unwrap_or_default();

    if path != "/api/blog/posts" {
        return AppErrorResponse::new(StatusCode::NOT_FOUND, Some("Not found".to_owned()), None);
    }

    if http_method == Method::OPTIONS.to_string() {
        return cors();
    }
    // dbg!(event.payload.query_string_parameters.clone());
    let request_post_query_params =
        if let Some(query_params) = event.payload.query_string_parameters {
            serde_json::from_value::<RequestPostsQueryParams>(query_params).unwrap_or_default()
        } else {
            RequestPostsQueryParams {
                featured: false,
                ..Default::default()
            }
        };

    // Get Posts
    let new_post = Post {
        title: Some("Understanding the ? Operator in Rust-3".to_owned()),
        slug: Some("understanding-the-question-mark-operator-in-rust-3".to_owned()),
        rust_code_snippet: Some("fn main() {\n    println!(\"Hello World\");\n}".to_owned()),
        content: Some("The From and Into traits are inherently linked, and this is actually part of its implementation. If you are able to convert type A from type B, then it should be easy to believe that we should be able to convert type B to type A.\n\nFrom\nThe From trait allows for a type to define how to create itself from another type, hence providing a very simple mechanism for converting between several types. There are numerous implementations of this trait within the standard library for conversion of primitive and common types.\n\nFor example we can easily convert a str into a String\n\n\nlet my_str = \"hello\";\nlet my_string = String::from(my_str);".to_owned()),
        published_by: Some(vec![ObjectId::new()]),
        tags: Some(vec!["rust".to_owned()]),
        code_snippet_enabled: Some(true),
        playground_enabled: Some(false),
        is_published: Some(true),
        is_featured: Some(false),
        ..Default::default()
    };

    let http_method_to_enum = Method::from_bytes(http_method.as_bytes()).unwrap_or_default();

    match http_method_to_enum {
        Method::GET => {
            if request_post_query_params.featured == true {
                return get_featured_posts(&database).await;
            }
            // post_handler::add_post(&database, new_post).await
            return get_posts(&database, Some(request_post_query_params.current_page)).await;
            // dbg!(request_post_query_params.current_page);
            // AppSuccessResponse::new(StatusCode::OK, Some("Request successful".to_owned()), None)
        }
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
