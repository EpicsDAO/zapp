use async_graphql::{Context, Object, Result};
use entity::{async_graphql, user2};
use sea_orm::EntityTrait;
use crate::graphql::mutation::common::*;
use crate::db::Database;

#[derive(Default)]
pub struct User2Query;

#[Object]
impl User2Query {
    async fn get_user2s(&self, ctx: &Context<'_>) -> Result<Vec<user2::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(user2::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_user2_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<user2::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(user2::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}