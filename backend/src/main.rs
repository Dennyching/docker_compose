



mod graphql;
mod todo;
mod web;

use std::env;
use dotenv::dotenv;
use sqlx::{
    postgres::{PgPoolOptions},
    Connection, PgConnection, PgPool,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?)
        .await
        .expect("Failed to create pool.");


    
    web::start(pool).await;
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use crate::graphql::*;
    use crate::todo::*;
    use crate::web::*;

    use anyhow::Result;
    use std::env;
    use dotenv::dotenv;
    use sqlx::{
        postgres::{PgPoolOptions},
        Connection, PgConnection, PgPool,
    };
    use tokio::task;
    use tokio::runtime::Runtime;
     #[test]
    fn test_list() ->anyhow::Result<()> {
        dotenv().ok();
        let mut rt = Runtime::new()?;
        rt.block_on( async{
            let str_url=&env::var("DATABASE_URL").unwrap();
            let  pg_result= connect_pg(str_url).await;
            let (mut conn, pg_pool) = pg_result.unwrap();
            let todo1 = Todo{id:"123".to_string(),body:"test1".to_string(),complete:false};
            let todo2 = Todo{id:"123".to_string(),body:"test2".to_string(),complete:false};

            let list_r=Todo::list(&pg_pool);
            for val in list_r.await.iter() {
                println!("{:?}",val);
            }
            //assert_eq!(Todo::list(&pg_pool),Todo::list(&pg_pool));
            let todo = Todo{id:"ad".to_string(),body:"aw".to_string(),complete:false};
        });
        Ok(())
    }
    #[test]
    fn test_insert() ->anyhow::Result<()> {
        dotenv().ok();
        let mut rt = Runtime::new()?;
        rt.block_on( async{
            let str_url=&env::var("DATABASE_URL").unwrap();
            let  pg_result= connect_pg(str_url).await;
            let (mut conn, pg_pool) = pg_result.unwrap();
            let todo1 = Todo{id:"123".to_string(),body:"test1".to_string(),complete:false};
            let todo2 = Todo{id:"123".to_string(),body:"test2".to_string(),complete:false};

            let insert=Todo::insert(&pg_pool,&"test_str".to_string()).await;
            let insert=&insert.unwrap();
            let id = &insert.id;
            let select_todo = Todo::select_id(&pg_pool,&id).await;
            let select_todo = select_todo.unwrap().unwrap();
            assert_eq!(insert.id,select_todo.id);
            assert_eq!(insert.body,select_todo.body);
            assert_eq!(insert.complete,select_todo.complete);
            let todo = Todo{id:"ad".to_string(),body:"aw".to_string(),complete:false};
        });
        Ok(())
    }
}




