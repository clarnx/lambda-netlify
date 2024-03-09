use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    http::{Method, StatusCode},
};
use dashboard::handlers::dashboard_handler::{find_admin_user, get_metadata};
use dotenvy::dotenv;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use mongodb::bson::from_document;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};

use serde_json::{json, Value};
use shared_lib::{
    database::client::connect_db,
    models::user::{User, UserRole},
    traits::model_traits::ModelTraits,
    utils::{cookie::parse_cookie, cors::cors},
    AppErrorResponse, AppSuccessResponse, RequestPayload,
};
use validator::HasLen;

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

async fn handler(event: LambdaEvent<RequestPayload>) -> Result<ApiGatewayProxyResponse, Error> {
    let cookie_token = parse_cookie(&event);

    let username_from_token = match cookie_token.as_ref() {
        Some(token) => token,
        None => return AppErrorResponse::new(StatusCode::UNAUTHORIZED, None, None),
    };

    let database = connect_db().await?;

    match find_admin_user(&database, username_from_token.to_string()).await {
        Some(user) => {
            let user: User = from_document(user).unwrap_or_default();
            let user_role = user.role.unwrap_or_default();

            if user_role != UserRole::Admin || user_role != UserRole::SuperAdmin {
                return AppErrorResponse::new(StatusCode::UNAUTHORIZED, None, None);
            }
        }
        None => return AppErrorResponse::new(StatusCode::UNAUTHORIZED, None, None),
    };

    let http_method = event.payload.http_method.unwrap_or_default().to_uppercase();
    let path = event.payload.path.unwrap_or_default();

    if path.contains("/api/dashboard") == false {
        return AppErrorResponse::new(StatusCode::NOT_FOUND, Some("Not found".to_owned()), None);
    }

    if http_method == Method::OPTIONS.to_string() {
        return cors();
    }

    let http_method_to_enum = Method::from_bytes(http_method.as_bytes()).unwrap_or_default();

    match http_method_to_enum {
        Method::GET => match path.as_str() {
            "/api/dashboard/metadata" => get_metadata(&database).await,
            _ => AppErrorResponse::new(
                StatusCode::NOT_ACCEPTABLE,
                Some("Not acceptable".to_owned()),
                None,
            ),
        },
        // Method::POST => {
        // let user_login_data_json = event.payload.body.unwrap_or_default();
        // let user_login_data: UserLoginData =
        //     serde_json::from_str::<UserLoginData>(&user_login_data_json).unwrap_or_default();

        // if user_login_data.username.is_none()
        //     && user_login_data.password.is_none()
        //     && cookie_token.is_none()
        // {
        //     return AppErrorResponse::new(StatusCode::UNAUTHORIZED, None, None);
        // };

        // if let None = user_login_data.username {
        //     return AppErrorResponse::new(
        //         StatusCode::BAD_REQUEST,
        //         Some("Username is required".to_owned()),
        //         None,
        //     );
        // };

        // if let None = user_login_data.password {
        //     return AppErrorResponse::new(
        //         StatusCode::BAD_REQUEST,
        //         Some("Password is required".to_owned()),
        //         None,
        //     );
        // };

        // login_admin(&database, user_login_data).await
        // }
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
