use std::{default, error::Error};

use chrono::{DateTime, Utc};
use inflector::Inflector;
use mongodb::{
    bson::{doc, document, to_document, Document},
    options::{FindOptions, IndexOptions},
    Cursor, Database, IndexModel,
};
use serde::{Deserialize, Serialize};
use validator::{HasLen, Validate, ValidationError, ValidationErrors};

use crate::{
    traits::model_traits::ModelTraits, DataInsertError, PaginatedData, PaginationMetadata,
};
use futures_util::stream::StreamExt;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub enum UserRole {
    #[serde(rename = "super_admin")]
    SuperAdmin,
    #[serde(rename = "admin")]
    Admin,
    #[default]
    #[serde(rename = "user")]
    User,
}

// impl Default for UserRole {
//     fn default() -> Self {
//         UserRole::Admin
//     }
// }

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

        let find_options = FindOptions::builder()
            .projection(projection)
            .sort(sort)
            .limit(Some(limit))
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

    async fn find_paginated(
        database: &Database,
        filter: document::Document,
        projection: Option<document::Document>,
        sort: Option<document::Document>,
        current_page: Option<i64>,
        items_per_page: Option<i64>,
    ) -> mongodb::error::Result<PaginatedData> {
        let collection_name = Self::get_struct_name_as_plural_string();

        let current_page = if let Some(page_no) = current_page {
            if page_no < 1 {
                1
            } else {
                page_no
            }
        } else {
            1
        };

        let items_per_page = if let Some(items_per_page_no) = items_per_page {
            if items_per_page_no < 1 {
                1
            } else {
                items_per_page_no
            }
        } else {
            10
        };

        let total_items = database
            .collection::<Self>(&collection_name)
            .count_documents(filter.clone(), None)
            .await?;

        let total_pages = (total_items as f64 / items_per_page as f64).ceil() as u64;

        let find_options = FindOptions::builder()
            .projection(projection)
            .sort(sort)
            .limit(Some(items_per_page))
            .skip(Some((current_page as u64 - 1) * items_per_page as u64))
            .build();

        let mut database_find_cursor = database
            .collection(&collection_name)
            .find(filter, find_options)
            .await?;

        let mut paginated_users_data = PaginatedData {
            documents: Vec::new(),
            metadata: PaginationMetadata {
                ..Default::default()
            },
        };

        while let Some(result) = database_find_cursor.next().await {
            match result {
                Ok(document) => paginated_users_data.documents.push(document),
                Err(_) => (),
            }
        }

        if paginated_users_data.documents.length() < 1 {
            return Ok(paginated_users_data);
        }
        paginated_users_data.metadata = PaginationMetadata {
            current_page: Some(current_page as u64),
            total_pages: Some(total_pages),
            total_items: Some(total_items),
            items_per_page: Some(items_per_page as u64),
        };
        Ok(paginated_users_data)
    }

    async fn count_documents(
        database: &Database,
        filter: document::Document,
    ) -> mongodb::error::Result<u64> {
        let collection_name = Self::get_struct_name_as_plural_string();

        let total_items = database
            .collection::<Self>(&collection_name)
            .count_documents(filter.clone(), None)
            .await?;

        Ok(total_items)
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
