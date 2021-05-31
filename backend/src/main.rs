



mod graphql;
mod todo;
mod web;

use std::env;
use dotenv::dotenv;
use sqlx::{PgPool, Row};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?)
        .await
        .expect("Failed to create pool.");

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS todo (id TEXT PRIMARY KEY NOT NULL, body TEXT NOT NULL, complete BOOLEAN NOT NULL) 
        "#
    )
            .execute(&pool)
            .await?;
    
    web::start(pool).await;
    Ok(())
}
