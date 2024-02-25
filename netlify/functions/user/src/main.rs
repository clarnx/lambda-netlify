use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    http::{Method, StatusCode},
};
use bcrypt::{hash, DEFAULT_COST};
use dotenvy::dotenv;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};

use serde_json::json;
use shared_lib::{
    database::client::connect_db,
    models::user::{User, UserRole},
    utils::cors::cors,
    AppErrorResponse, AppSuccessResponse, RequestPayload,
};
use user::handlers::user_handler::add_user;

async fn handler(event: LambdaEvent<RequestPayload>) -> Result<ApiGatewayProxyResponse, Error> {
    let database = connect_db().await?;

    let http_method = event.payload.http_method.unwrap_or_default().to_uppercase();
    let path = event.payload.path.unwrap_or_default();

    if path.contains("/api/user") == false {
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
        Method::POST => {
            let user_data_json = event.payload.body.unwrap_or_default();
            let user_data: User = serde_json::from_str::<User>(&user_data_json).unwrap_or_default();

            const CUSTOM_DEFAULT_COST: u32 = 14;

            let hashed_password =
                hash(user_data.password.unwrap_or_default(), CUSTOM_DEFAULT_COST)?;

            let new_user_data = User {
                username: user_data.username,
                email: user_data.email,
                password: Some(hashed_password),
                role: user_data.role,
                ..Default::default()
            };

            let response = add_user(&database, new_user_data).await;

            return response;
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
