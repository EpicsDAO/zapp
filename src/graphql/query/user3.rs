use async_graphql::{Context, Object, Result};
use entity::{async_graphql, user3};
use sea_orm::EntityTrait;
use crate::graphql::mutation::common::*;
use crate::db::Database;

#[derive(Default)]
pub struct User3Query;

#[Object]
impl User3Query {
    async fn get_user3s(&self, ctx: &Context<'_>) -> Result<Vec<user3::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(user3::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_user3_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<user3::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(user3::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}