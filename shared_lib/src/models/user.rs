use std::error::Error;

use chrono::{DateTime, Utc};
use inflector::Inflector;
use mongodb::{
    bson::{doc, document, to_document, Document},
    options::{FindOptions, IndexOptions},
    Cursor, Database, IndexModel,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{traits::model_traits::ModelTraits, DataInsertError};
use futures_util::stream::StreamExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserRole {
    #[serde(rename = "super_admin")]
    SuperAdmin,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "user")]
    User,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct User {
    #[validate(
        required,
        length(min = 3, message = "Username cannot be less than 3 characters"),
        length(max = 25, message = "Username cannot be more than 25 characters")
    )]
    pub username: Option<String>,
    #[validate(required, email(message = "Enter a valid email address."))]
    pub email: Option<String>,
    #[validate(required(message = "Password is required"))]
    pub password: Option<String>,
    #[validate(required(message = "Role is required"))]
    pub role: Option<UserRole>,
    pub profile_image: Option<String>,
    #[validate(required)]
    pub created_at: Option<DateTime<Utc>>,
    #[validate(required)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UniqueUserFields {
    email: bool,
    username: bool,
}

impl ModelTraits for User {
    fn get_struct_name_as_plural_string() -> String {
        stringify!(User).to_lowercase().to_plural()
    }

    async fn set_unique_fields(database: &Database) -> Result<(), DataInsertError> {
        let collection_name = Self::get_struct_name_as_plural_string();
        let unique_fields = UniqueUserFields {
            email: true,
            username: true,
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

        let find_options = FindOptions::builder().build();

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

impl Default for User {
    fn default() -> Self {
        Self {
            username: None,
            email: None,
            password: None,
            role: Some(UserRole::User),
            profile_image: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }
}