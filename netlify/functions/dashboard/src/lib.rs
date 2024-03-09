use chrono::{DateTime, Utc};
use mongodb::bson::{oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

pub mod handlers;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserLoginData {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DashboardMetadata {
    pub posts_count: u64,
    pub published_posts_count: u64,
    pub draft_posts_count: u64,
    pub featured_posts_count: u64,
    pub recent_posts: Vec<Document>,
}
