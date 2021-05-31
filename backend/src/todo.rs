

use sqlx::{
    postgres::{ PgRow},
    Row,
    PgPool,
};
use uuid::Uuid;
use std::option::Option;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub body: String,
    pub complete: bool,
}

impl Todo {
    pub async fn insert(pool: &PgPool, body: &str) -> anyhow::Result<Todo> {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"
                INSERT INTO todo VALUES ($1, $2, $3)
            "#,)
            .bind(&id)
            .bind(body)
            .bind(false)
            .execute(pool)
            .await?;

        Ok(Todo {
            id,
            body: body.to_string(),
            complete: false,
        })
    }

    pub async fn list(pool: &PgPool) -> Result<Vec<Todo>,sqlx::Error> {
        let recs =sqlx::query(
            r#"
            SELECT * FROM todo
            "#,)
            .map(|row:PgRow| Todo{
                id:row.get("id"),
                body:row.get("body"),
                complete:row.get("complete")
            })
            .fetch_all(pool)
            .await?;
        Ok(recs)
    }

    pub async fn update(
        pool: &PgPool,
        id: &str,
        body: &str,
        complete: bool,
    ) -> Result<Option<Todo>,sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE todo SET body=$1, complete=$2 WHERE id=$3"#,)
        .bind(body)
        .bind(complete)
        .bind(id)
        .execute(pool)
        .await?;

        let todo = sqlx::query(
            r#"
            SELECT * FROM todo WHERE id=$1"#,) 
            .bind(id)
            .map(|row:PgRow| Todo{
                id:row.get("id"),
                body:row.get("body"),
                complete:row.get("complete")
            })
            .fetch_optional(pool)
            .await?;
        Ok(todo)
    }

    pub async fn toggle_complete(pool: &PgPool, id: &str) -> Result<Option<Todo>,sqlx::Error>{
        sqlx::query(r#"
            UPDATE todo SET complete=NOT complete WHERE id=$1
            "#,)
            .bind(id)
            .execute(pool)
            .await?;
        let todo = sqlx::query(
            r#"
            SELECT * FROM todo WHERE id=$1"#,) 
            .bind(id)
            .map(|row:PgRow| Todo{
                id:row.get("id"),
                body:row.get("body"),
                complete:row.get("complete")
            })
            .fetch_optional(pool)
            .await?;
            Ok(todo)
    }

    pub async fn delete(pool: &PgPool, id: &str) ->anyhow::Result<()>{
        sqlx::query(
            r#"
                DELETE FROM todo WHERE id=$1
            "#,) 
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
