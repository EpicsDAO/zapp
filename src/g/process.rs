use std::str;
use std::fs;
use std::io::Write;
use chrono::Local;
use crate::style_print::*;

pub async fn process_create_migration(model: &str) {
  let dt = Local::now();
  let filename = format!("m{}{}{}_{}{}{}_create_{}_table", dt.format("%Y"), dt.format("%m"), dt.format("%d"),dt.format("%H"), dt.format("%M"), dt.format("%S"), model);
  let file_dir = "migration/src/";
  let file_path = String::from(file_dir) + &filename;
  let migration = format!("use entity::task;
use sea_orm::{{DbBackend, EntityTrait, Schema}};
use sea_orm_migration::prelude::*;

pub struct Migration;

fn get_seaorm_create_stmt<E: EntityTrait>(e: E) -> TableCreateStatement {{
    let schema = Schema::new(DbBackend::Postgres);

    schema
        .create_table_from_entity(e)
        .if_not_exists()
        .to_owned()
}}

fn get_seaorm_drop_stmt<E: EntityTrait>(e: E) -> TableDropStatement {{
    Table::drop().table(e).if_exists().to_owned()
}}

impl MigrationName for Migration {{
    fn name(&self) -> &str {{
        'm{}{}{}_{}{}{}_create_{}_table'
    }}
}}

#[async_trait::async_trait]
impl MigrationTrait for Migration {{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        let stmts = vec![get_seaorm_create_stmt(task::Entity)];

        for stmt in stmts {{
            manager.create_table(stmt.to_owned()).await?;
        }}

        Ok(())
    }}

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        let stmts = vec![get_seaorm_drop_stmt(task::Entity)];

        for stmt in stmts {{
            manager.drop_table(stmt.to_owned()).await?;
        }}

        Ok(())
    }}
}}", dt.format("%Y"), dt.format("%m"), dt.format("%d"),dt.format("%H"), dt.format("%M"), dt.format("%S"), model);
  let mut file = fs::File::create(file_path).unwrap();
  file.write_all(migration.as_bytes()).unwrap();
  log_success(format!("Successfully created migration file: {}", file_path));
}