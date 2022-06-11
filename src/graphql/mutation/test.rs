use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::test;
use sea_orm::{ActiveModelTrait, Set};

use crate::db::Database;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateTestInput {
  // Define schema here
}

#[derive(SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct TestMutation;

#[Object]
impl TestMutation {
    pub async fn create_test(
        &self,
        ctx: &Context<'_>,
        input: CreateTestInput,
    ) -> Result<test::Model> {
        let db = ctx.data::<Database>().unwrap();

        // Define schema here
        let test = test::ActiveModel {
            ..Default::default()
        };

        Ok(test.insert(db.get_connection()).await?)
    }

    pub async fn delete_test(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = test::Entity::delete_by_id(id)
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