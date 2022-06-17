use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::user3;
use sea_orm::{ActiveModelTrait, Set};
use crate::graphql::mutation::common::*;
use crate::db::Database;


#[derive(InputObject)]
pub struct CreateUser3Input {
    pub id: i32
}

#[derive(Default)]
pub struct User3Mutation;

#[Object]
impl User3Mutation {
    pub async fn create_user3(
        &self,
        ctx: &Context<'_>,
        input: CreateUser3Input,
    ) -> Result<user3::Model> {
        let db = ctx.data::<Database>().unwrap();

        // Define schema here
        let user3 = user3::ActiveModel {
            id: Set(input.id),
            ..Default::default()
        };

        Ok(user3.insert(db.get_connection()).await?)
    }

    pub async fn delete_user3(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = user3::Entity::delete_by_id(id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            Ok(DeleteResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }
}