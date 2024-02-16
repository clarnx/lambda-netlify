use std::env;

use mongodb::{options::ClientOptions, Client, Database};

pub async fn connect_db() -> mongodb::error::Result<Database> {
    let database_uri = env::var("DATABASE_URI").unwrap_or_default();
    let database_name = env::var("DATABASE_NAME").unwrap_or_default();

    let client_options = ClientOptions::parse(database_uri).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(&database_name);

    Ok(database)
}
