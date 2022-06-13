pub use sea_orm_migration::prelude::*;

// Load module here;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Define migration file here;
        ]
    }
}