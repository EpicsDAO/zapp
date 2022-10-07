use async_graphql::{Context, Object, Result, Error};
use entity::async_graphql::{self, InputObject};
use entity::user;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, Set};
use crate::graphql::mutation::common::*;
use crate::db::Database;
#[derive(InputObject)]
pub struct CreateUserInput {
    pub id: i32,
}
#[derive(Default)]
pub struct UserMutation;
#[Object]
impl UserMutation {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<user::Model> {
        let db = ctx.data::<Database>().unwrap();
        let naive_date_time = Utc::now().naive_utc();
        let user = user::ActiveModel {
            id: Set(input.id),
            created_at: Set(naive_date_time),
            updated_at: Set(naive_date_time),
            ..Default::default()
        };
        Ok(user.insert(db.get_connection()).await?)
    }
    pub async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<user::Model, Error> {
        let db = ctx.data::<Database>().unwrap();
        let naive_date_time = Utc::now().naive_utc();
        let user: Option<user::Model> = user::Entity::find_by_id(id)
            .one(db.get_connection())
            .await?;
        let mut user: user::ActiveModel = user.unwrap().into();
        user.updated_at = Set(naive_date_time);
        let user: user::Model = user.update(db.get_connection()).await?;
        Ok(user)
    }
    pub async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();
        let res = user::Entity::delete_by_id(id).exec(db.get_connection()).await?;
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
