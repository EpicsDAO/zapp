use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::user2;
use sea_orm::{ActiveModelTrait, Set};
use crate::graphql::mutation::common::*;
use crate::db::Database;


#[derive(InputObject)]
pub struct CreateUser2Input {
    pub id: i32
}

#[derive(Default)]
pub struct User2Mutation;

#[Object]
impl User2Mutation {
    pub async fn create_user2(
        &self,
        ctx: &Context<'_>,
        input: CreateUser2Input,
    ) -> Result<user2::Model> {
        let db = ctx.data::<Database>().unwrap();

        // Define schema here
        let user2 = user2::ActiveModel {
            id: Set(input.id),
            ..Default::default()
        };

        Ok(user2.insert(db.get_connection()).await?)
    }

    pub async fn delete_user2(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = user2::Entity::delete_by_id(id)
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