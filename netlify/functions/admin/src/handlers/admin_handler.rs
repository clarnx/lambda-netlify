use std::env;

use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    http::{HeaderValue, StatusCode},
};
use bcrypt::verify;
use cookie::{time::Duration, Cookie, CookieJar, Key};
use lambda_runtime::Error;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId},
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
use validator::HasLen;

use crate::UserLoginData;

pub async fn login_admin(
    database: &Database,
    user_login_data: UserLoginData,
) -> Result<ApiGatewayProxyResponse, Error> {
    let cookie_secret = env::var("COOKIE_SECRET").unwrap_or_default();

    let username = user_login_data.username.clone().unwrap_or_default();
    let password = user_login_data.password.clone().unwrap_or_default();

    let user_from_db_result = User::find(
        database,
        doc! {"username": username.clone()},
        Some(doc! {"created_at": false, "updated_at": false}),
        None,
        1,
    )
    .await;

    match user_from_db_result {
        Ok(data_from_db) => {
            if data_from_db.length() == 0 {
                return AppErrorResponse::new(
                    StatusCode::NOT_FOUND,
                    Some("User not found. Make sure username or password is correct".to_string()),
                    None,
                );
            }

            let db_user: User = from_document::<User>(data_from_db[0].clone()).unwrap_or_default();

            if db_user.role.clone().unwrap_or(UserRole::User) != UserRole::Admin {
                return AppErrorResponse::new(
                    StatusCode::UNAUTHORIZED,
                    Some("Unauthorized login request".to_string()),
                    None,
                );
            }

            let hashed_password_from_db = db_user.password.clone().unwrap_or_default();
            let password_is_valid = verify(password, &hashed_password_from_db).unwrap_or_default();

            if !password_is_valid {
                return AppErrorResponse::new(
                    StatusCode::NOT_FOUND,
                    Some("User not found. Make sure username or password is correct".to_string()),
                    None,
                );
            }

            let key = Key::from(cookie_secret.as_bytes());
            // Add a private (signed + encrypted) cookie.
            let mut jar = CookieJar::new();
            let mut cookie =
                Cookie::new("sessionToken", db_user.username.clone().unwrap_or_default());
            cookie.set_http_only(true);
            cookie.set_max_age(Duration::days(30));
            jar.private_mut(&key).add(cookie);

            // The cookie's contents are encrypted.
            let cookie_value = jar.get("sessionToken").unwrap().to_string();

            let mut response = AppSuccessResponse::new(
                StatusCode::OK,
                Some("Login successful".to_string()),
                Some(json!({"user": db_user.username})),
            )
            .unwrap_or_default();

            response
                .headers
                .insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

            return Ok(response);
        }
        Err(_) => {
            return AppErrorResponse::new(
                StatusCode::NOT_FOUND,
                Some("User not found. Make sure username or password is correct".to_string()),
                None,
            );
        }
    };
}

pub async fn add_post(
    database: &Database,
    new_post_data: Post,
) -> Result<ApiGatewayProxyResponse, Error> {
    let result = new_post_data.save(&database).await;

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

pub async fn get_posts(
    database: &Database,
    current_page: Option<i64>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let post_response = Post::find_paginated(
        &database,
        doc! {"is_published": true},
        Some(doc! {"title": true, "slug": true, "tags": true, "created_at": true, "_id": false}),
        Some(doc! { "created_at": -1 }),
        current_page,
        Some(4),
    )
    .await;

    match post_response {
        Ok(paginated_posts_data) => {
            let posts = paginated_posts_data.documents;
            let pagination_metadata = paginated_posts_data.metadata;

            AppSuccessResponse::new(
                StatusCode::OK,
                Some("Request successful".to_string()),
                Some(json!({
                    "posts": posts,
                    "metadata": {
                        "pagination": pagination_metadata
                    }
                })),
            )
        }

        Err(_) => AppSuccessResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("An error occured fetching data".to_string()),
            None,
        ),
    }
}

pub async fn get_post_by_slug(
    database: &Database,
    slug: String,
) -> Result<ApiGatewayProxyResponse, Error> {
    let post_response = Post::find_one(
        &database,
        doc! {"slug": slug, "is_published": true},
        Some(doc! {"_id": false}),
        1,
    )
    .await;

    match post_response {
        Ok(document) => AppSuccessResponse::new(
            StatusCode::OK,
            Some("Request successful".to_string()),
            Some(json!({
                "post": document
            })),
        ),

        Err(_) => AppSuccessResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("An error occured fetching data".to_string()),
            None,
        ),
    }
}

pub async fn get_featured_posts(database: &Database) -> Result<ApiGatewayProxyResponse, Error> {
    let featured_post_response = Post::find(
        &database,
        doc! {"is_featured": true, "is_published": true},
        Some(doc! {"title": true, "slug": true, "tags": true, "updated_at": true, "_id": false}),
        Some(doc! { "updated_at": -1 }),
        3,
    )
    .await;

    match featured_post_response {
        Ok(documents) => AppSuccessResponse::new(
            StatusCode::OK,
            Some("Request successful".to_string()),
            // Some(
            //     serde_json::to_value(
            //         documents
            //             .iter()
            //             .map(|doc| {
            //                 from_document_with_options::<FeaturedPostResponseData>(
            //                     doc.clone(),
            //                     DeserializerOptions::builder().human_readable(false).build(),
            //                 )
            //                 .unwrap()
            //             })
            //             .collect::<Vec<FeaturedPostResponseData>>(),
            //     )
            //     .unwrap(),
            // ),
            Some(serde_json::to_value(documents).unwrap_or_default()),
        ),

        Err(_) => AppSuccessResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("An error occured fetching data".to_string()),
            None,
        ),
    }
}
