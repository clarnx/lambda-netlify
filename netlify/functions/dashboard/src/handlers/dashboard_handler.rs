use std::result;

use aws_lambda_events::{apigw::ApiGatewayProxyResponse, http::StatusCode};
use lambda_runtime::Error;
use mongodb::{
    bson::{doc, Document},
    Database,
};
use serde_json::json;
use shared_lib::{
    models::{post::Post, user::User},
    traits::model_traits::ModelTraits,
    AppSuccessResponse,
};
use validator::HasLen;

use crate::DashboardMetadata;

pub async fn get_metadata(database: &Database) -> Result<ApiGatewayProxyResponse, Error> {
    let recent_posts = match Post::find(
        &database,
        doc! {},
        Some(doc! {"title": true, "is_published": true, "created_at": true, "_id": true}),
        Some(doc! { "created_at": -1 }),
        5,
    )
    .await
    {
        Ok(posts) => posts,
        Err(_) => {
            return AppSuccessResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some("An error occured fetching data".to_string()),
                None,
            )
        }
    };

    let posts_count = match Post::count_documents(&database, doc! {}).await {
        Ok(count) => count,
        Err(_) => {
            return AppSuccessResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some("An error occured fetching data".to_string()),
                None,
            )
        }
    };

    let published_posts_count =
        match Post::count_documents(&database, doc! {"is_published": true}).await {
            Ok(count) => count,
            Err(_) => {
                return AppSuccessResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some("An error occured fetching data".to_string()),
                    None,
                )
            }
        };

    let draft_posts_count =
        match Post::count_documents(&database, doc! {"is_published": false}).await {
            Ok(count) => count,
            Err(_) => {
                return AppSuccessResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some("An error occured fetching data".to_string()),
                    None,
                )
            }
        };

    let featured_posts_count =
        match Post::count_documents(&database, doc! {"is_featured": true}).await {
            Ok(count) => count,
            Err(_) => {
                return AppSuccessResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some("An error occured fetching data".to_string()),
                    None,
                )
            }
        };

    let dashboard_metadata = DashboardMetadata {
        posts_count,
        published_posts_count,
        draft_posts_count,
        featured_posts_count,
        recent_posts,
    };

    AppSuccessResponse::new(
        StatusCode::OK,
        Some("Request successful".to_string()),
        Some(json!({
            "dashboard_metadata": dashboard_metadata
        })),
    )
}

pub async fn find_admin_user(database: &Database, username: String) -> Option<Document> {
    let _ = match User::find(
        &database,
        doc! {"username": username, "role": "admin"},
        None,
        None,
        1,
    )
    .await
    {
        Ok(user_data) => {
            if user_data.length() != 0 {
                if let Some(user) = user_data.get(0) {
                    return Some(user.to_owned());
                };
            }
            return None;
        }
        Err(_) => return None,
    };
}
