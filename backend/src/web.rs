use crate::graphql::{MutationRoot, QueryRoot, SubscriptionRoot};
use async_graphql::http::*;
use async_graphql::{QueryBuilder, Schema};
use async_graphql_warp::graphql_subscription;
use sqlx::{
    postgres::{PgPoolOptions},
    Connection, PgConnection, PgPool,
};
use std::convert::Infallible;
use warp::{http::Response, Filter, Reply};

pub async fn connect_pg(database_url:&str)-> Result<(PgConnection, PgPool), sqlx::Error>{
    let conn = PgConnection::connect(database_url).await?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
        Ok((conn, pool))
}
pub async fn start(pool: PgPool) {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(pool)
        .finish();

    println!("Playground: http://localhost:8080");

    let graphql_post = async_graphql_warp::graphql(schema.clone()).and_then(
        |(schema, builder): (_, QueryBuilder)| async move {
            let resp = builder.execute(&schema).await;
            Ok::<_, Infallible>(warp::reply::json(&GQLResponse(resp)).into_response())
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("http://localhost:8080")))
    });

    let routes = graphql_post
        .or(graphql_subscription(schema))
        .or(graphql_playground);
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
