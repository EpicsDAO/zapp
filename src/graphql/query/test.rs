use async_graphql::{Context, Object, Result};
use entity::{async_graphql, test};
use sea_orm::EntityTrait;

use crate::db::Database;

#[derive(Default)]
pub struct TestQuery;

#[Object]
impl TestQuery {
    async fn get_tests(&self, ctx: &Context<'_>) -> Result<Vec<test::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(test::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_test_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<test::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(test::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}