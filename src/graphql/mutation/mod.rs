use entity::async_graphql;

pub mod common;
pub mod user;
pub mod user2;
pub mod user3;

pub use user::UserMutation;
pub use user2::User2Mutation;
pub use user3::User3Mutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(UserMutation, User2Mutation, User3Mutation);