use aws_lambda_events::{apigw::ApiGatewayProxyResponse, http::StatusCode};
use lambda_runtime::Error;
use mongodb::{
    bson::{doc, from_document},
    Database,
};
use serde_json::json;
use shared_lib::{
    models::{
        post::Post,
        user::{User, UserRole},
    },
    traits::model_traits::ModelTraits,
    AppErrorResponse, AppSuccessResponse, DataInsertError,
};

use crate::UserLoginData;

pub async fn add_user(
    database: &Database,
    new_user_data: User,
) -> Result<ApiGatewayProxyResponse, Error> {
    let result = new_user_data.save(&database).await;

    let _ = match result {
        Ok(_insert_value) => {
            return AppSuccessResponse::new(
                StatusCode::OK,
                Some("Data added successfully".to_string()),
                None,
            )
        }
        Err(error) => match error {
            DataInsertError::FieldValidationError(error) => {
                return AppErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    Some("An error occured".to_string()),
                    Some(json!({
                        "errors": error
                    })),
                )
            }
            DataInsertError::MongoDuplicateError(mut error) if error.code == 11000 => {
                let duplicate_field_data =
                    error.message.split("key: ").collect::<Vec<&str>>()[1].to_owned();

                let duplicate_field_value =
                    duplicate_field_data.split(" ").collect::<Vec<&str>>()[2].to_owned();

                error.message =
                    format!("An error occured. {} already exists", duplicate_field_value);

                return AppErrorResponse::new(StatusCode::BAD_REQUEST, Some(error.message), None);
            }
            _ => {
                return AppErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some("An error occured".to_string()),
                    None,
                )
            }
        },
    };
}
