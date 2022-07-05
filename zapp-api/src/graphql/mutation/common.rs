use entity::async_graphql;
use crate::async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}