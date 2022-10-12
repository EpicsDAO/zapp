mod db;
mod graphql;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use graphql::schema::{build_schema, ZappSchema};
use std::env;

#[cfg(debug_assertions)]
use dotenv::dotenv;

async fn graphql_handler(schema: Extension<ZappSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();
    let port = env::var("PORT").expect("PORT must be set.");
    let schema = build_schema().await;

    let app = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .layer(Extension(schema));

    println!("Playground: http://localhost:{}/api/graphql", port);

    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
