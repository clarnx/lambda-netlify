use aws_lambda_events::{apigw::ApiGatewayProxyResponse, http::StatusCode};
use lambda_runtime::Error;
use mongodb::{bson::doc, Database};
use serde_json::json;
use shared_lib::{
    models::post::Post, traits::model_traits::ModelTraits, AppErrorResponse, AppSuccessResponse,
    DataInsertError,
};

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
