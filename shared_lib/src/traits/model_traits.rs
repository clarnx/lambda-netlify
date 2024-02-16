use std::error::Error;

use mongodb::{
    bson::{document, Document},
    results::InsertOneResult,
    Cursor, Database,
};
use serde_json::Value;

use crate::DataInsertError;

pub trait ModelTraits {
    fn save(
        &self,
        database: &Database,
    ) -> impl std::future::Future<Output = Result<InsertOneResult, DataInsertError>> + Send;

    fn find(
        database: &Database,
        filter: document::Document,
        projection: Option<document::Document>,
        sort: Option<document::Document>,
        limit: i64,
    ) -> impl std::future::Future<Output = mongodb::error::Result<Vec<Document>>> + Send;

    fn set_unique_fields(
        database: &Database,
    ) -> impl std::future::Future<Output = Result<(), DataInsertError>> + Send;

    fn get_struct_name_as_plural_string() -> String;
}
