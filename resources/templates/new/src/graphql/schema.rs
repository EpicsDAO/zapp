use async_graphql::{EmptySubscription, Schema};
use migration::{Migrator, MigratorTrait};

use crate::{
    db::Database,
    graphql::{mutation::Mutation, query::Query},
};

pub type ZappSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema() -> ZappSchema {
    let db = Database::new().await;

    Migrator::up(db.get_connection(), None).await.unwrap();

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
