use entity::async_graphql;

pub mod common;
pub mod user;
pub mod user2;
pub mod user3;

pub use user::UserQuery;
pub use user2::User2Query;
pub use user3::User3Query;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery, User2Query, User3Query);