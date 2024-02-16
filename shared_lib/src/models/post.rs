use chrono::{DateTime, Utc};
use inflector::Inflector;
use mongodb::{
    bson::{doc, document, oid::ObjectId, to_document, Document},
    options::{FindOptions, IndexOptions},
    Cursor, Database, IndexModel,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use validator::Validate;

use crate::{traits::model_traits::ModelTraits, DataInsertError};
use futures_util::stream::StreamExt;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct Post {
    #[validate(
        required(message = "Title is required"),
        length(min = 10, message = "Username cannot be less than 10 characters"),
        length(max = 70, message = "Username cannot be more than 70 characters")
    )]
    pub title: Option<String>,
    #[validate(required)]
    pub slug: Option<String>,
    pub rust_code_snippet: Option<String>,
    #[validate(required(message = "Post content is required"))]
    pub content: Option<String>,
    #[validate(required(message = "Post author is required"))]
    pub published_by: Option<Vec<ObjectId>>,
    #[validate(length(max = 5, message = "Tags exceed the limit of 5"))]
    pub tags: Option<Vec<String>>,
    pub code_snippet_enabled: Option<bool>,
    pub playground_enabled: Option<bool>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    #[validate(required)]
    pub created_at: Option<DateTime<Utc>>,
    #[validate(required)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UniquePostFields {
    title: bool,
    slug: bool,
}

impl ModelTraits for Post {
    fn get_struct_name_as_plural_string() -> String {
        stringify!(Post).to_lowercase().to_plural()
    }

    async fn set_unique_fields(database: &Database) -> Result<(), DataInsertError> {
        let collection_name = Self::get_struct_name_as_plural_string();
        let unique_fields = UniquePostFields {
            title: true,
            slug: true,
        };

        let bson_doc = to_document(&unique_fields).unwrap();

        for (key, _) in bson_doc.iter() {
            let options = IndexOptions::builder().unique(true).build();
            let model = IndexModel::builder()
                .keys(doc! {key: 1})
                .options(Some(options))
                .build();

            database
                .collection::<Self>(&collection_name)
                .create_index(model, None)
                .await?;
        }
        Ok(())
    }

    async fn save(
        &self,
        database: &Database,
    ) -> Result<mongodb::results::InsertOneResult, DataInsertError> {
        self.validate()?;
        Self::set_unique_fields(database).await?;

        let collection_name = Self::get_struct_name_as_plural_string();

        let database_insert_response = database
            .collection::<Self>(&collection_name)
            .insert_one(self, None)
            .await?;

        Ok(database_insert_response)
    }

    async fn find(
        database: &Database,
        filter: document::Document,
        projection: Option<document::Document>,
        sort: Option<document::Document>,
        limit: i64,
    ) -> mongodb::error::Result<Vec<Document>> {
        let collection_name = Self::get_struct_name_as_plural_string();

        let find_options = FindOptions::builder()
            .projection(projection)
            .sort(sort)
            .limit(limit)
            .build();

        let mut database_find_cursor = database
            .collection(&collection_name)
            .find(filter, find_options)
            .await?;

        let mut documents = Vec::new();

        while let Some(result) = database_find_cursor.next().await {
            match result {
                Ok(document) => documents.push(document),
                Err(_) => (),
            }
        }

        Ok(documents)
    }
}

impl Default for Post {
    fn default() -> Self {
        Self {
            title: None,
            slug: None,
            rust_code_snippet: None,
            content: None,
            published_by: None,
            tags: None,
            code_snippet_enabled: Some(false),
            playground_enabled: Some(false),
            is_published: Some(false),
            is_featured: Some(false),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }
}
