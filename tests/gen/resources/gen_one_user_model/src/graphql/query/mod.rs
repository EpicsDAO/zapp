use entity::async_graphql;
pub mod user;
pub use user::UserQuery;
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery);
