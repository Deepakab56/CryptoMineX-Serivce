use mongodb::{ Client, Database };
use std::env;

pub async fn connect() -> Result<Database, mongodb::error::Error> {
    let uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "axum_db".to_string());

    println!("{}", uri);

    let client = Client::with_uri_str(&uri).await?;
    let db = client.database(&db_name);

    db.run_command(mongodb::bson::doc! { "ping": 1 }, None).await?;

    Ok(db)
}
