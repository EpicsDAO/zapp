use std::str;
use std::fs;
use std::io::Write;
use chrono::Local;
use crate::style_print::*;
use std::io;
use std::path::Path;
use std::fs::OpenOptions;

pub async fn process_create_migration(model: &str) {
  let dt = Local::now();
  let filename = format!("m{}{}{}_{}{}{}_create_{}_table", dt.format("%Y"), dt.format("%m"), dt.format("%d"),dt.format("%H"), dt.format("%M"), dt.format("%S"), model);
  let file_dir = "migration/src/";
  fs::create_dir_all(file_dir).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
  let file_path = String::from(file_dir) + &filename + ".rs";
  let file_content = format!("use entity::{};
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
        let stmts = vec![get_seaorm_create_stmt({}::Entity)];

        for stmt in stmts {{
            manager.create_table(stmt.to_owned()).await?;
        }}

        Ok(())
    }}

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        let stmts = vec![get_seaorm_drop_stmt({}::Entity)];

        for stmt in stmts {{
            manager.drop_table(stmt.to_owned()).await?;
        }}

        Ok(())
    }}
}}", model, dt.format("%Y"), dt.format("%m"), dt.format("%d"),dt.format("%H"), dt.format("%M"), dt.format("%S"), model, model, model);
  let mut file = fs::File::create(&file_path).unwrap();
  file.write_all(file_content.as_bytes()).unwrap();
  log_success(&format!("Successfully created migration file: {}", &file_path)).await;
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
      None => String::new(),
      Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}

pub async fn process_create_entity(model: &str) {
  let capital_model = some_kind_of_uppercase_first_letter(model);
  let filename = format!("{}.rs", model);
  let file_dir = "entity/src/";
  fs::create_dir_all(file_dir).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
  let file_path = String::from(file_dir) + &filename;
  let file_content = format!("use async_graphql::*;
use sea_orm::{{entity::prelude::*, DeleteMany}};
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = \"{}s\")]
#[graphql(concrete(name = \"{}\", params()))]
pub struct Model {{
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    // Define schema here
}}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {{}}

impl RelationTrait for Relation {{
    fn def(&self) -> RelationDef {{
        panic!(\"No RelationDef\")
    }}
}}

impl ActiveModelBehavior for ActiveModel {{}}

impl Entity {{
    pub fn find_by_id(id: i32) -> Select<Entity> {{
        Self::find().filter(Column::Id.eq(id))
    }}

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {{
        Self::delete_many().filter(Column::Id.eq(id))
    }}
}}", model, capital_model);
  let mut file = fs::File::create(&file_path).unwrap();
  file.write_all(file_content.as_bytes()).unwrap();
  log_success(&format!("Successfully created entity file: {}", &file_path)).await;
}

pub async fn process_create_mutation(model: &str) {
  let capital_model = some_kind_of_uppercase_first_letter(model);
  let filename = format!("{}.rs", model);
  let file_dir = "src/graphql/mutation/";
  fs::create_dir_all(file_dir).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
  let file_path = String::from(file_dir) + &filename;
  let file_content = format!("use async_graphql::{{Context, Object, Result}};
use entity::async_graphql::{{self, InputObject, SimpleObject}};
use entity::{};
use sea_orm::{{ActiveModelTrait, Set}};

use crate::db::Database;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct Create{}Input {{
  // Define schema here
}}

#[derive(SimpleObject)]
pub struct DeleteResult {{
    pub success: bool,
    pub rows_affected: u64,
}}

#[derive(Default)]
pub struct {}Mutation;

#[Object]
impl {}Mutation {{
    pub async fn create_{}(
        &self,
        ctx: &Context<'_>,
        input: Create{}Input,
    ) -> Result<{}::Model> {{
        let db = ctx.data::<Database>().unwrap();

        // Define schema here
        let {} = {}::ActiveModel {{
            ..Default::default()
        }};

        Ok({}.insert(db.get_connection()).await?)
    }}

    pub async fn delete_{}(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {{
        let db = ctx.data::<Database>().unwrap();

        let res = {}::Entity::delete_by_id(id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {{
            Ok(DeleteResult {{
                success: true,
                rows_affected: res.rows_affected,
            }})
        }} else {{
            unimplemented!()
        }}
    }}
}}", model, capital_model, capital_model, capital_model, model, capital_model, model, model, model, model, model, model);
  let mut file = fs::File::create(&file_path).unwrap();
  file.write_all(file_content.as_bytes()).unwrap();
  log_success(&format!("Successfully created mutation file: {}", &file_path)).await;
}

pub async fn process_create_query(model: &str) {
  let capital_model = some_kind_of_uppercase_first_letter(model);
  let filename = format!("{}.rs", model);
  let file_dir = "src/graphql/query/";
  fs::create_dir_all(file_dir).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
  let file_path = String::from(file_dir) + &filename;
  let file_content = format!("use async_graphql::{{Context, Object, Result}};
use entity::{{async_graphql, {}}};
use sea_orm::EntityTrait;

use crate::db::Database;

#[derive(Default)]
pub struct {}Query;

#[Object]
impl {}Query {{
    async fn get_{}s(&self, ctx: &Context<'_>) -> Result<Vec<{}::Model>> {{
        let db = ctx.data::<Database>().unwrap();

        Ok({}::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }}

    async fn get_{}_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<{}::Model>> {{
        let db = ctx.data::<Database>().unwrap();

        Ok({}::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }}
}}", model, capital_model, capital_model, model, model, model, model, model, model);
  let mut file = fs::File::create(&file_path).unwrap();
  file.write_all(file_content.as_bytes()).unwrap();
  log_success(&format!("Successfully created query file: {}", &file_path)).await;
}

pub async fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect())
}

pub async fn process_create_mutation_route() {
    let dir = "src/graphql/mutation/";
    let files = read_dir(dir).await.unwrap();
    let mutation_box = files.iter().cloned()
        .filter(|i| i != "mod.rs")
        .map(|i| {i.replace(".rs", "")})
        .collect::<Vec<_>>();

    let file_path = "src/graphql/mutation/mod.rs";
    let content1 = b"use entity::async_graphql;\n\n";
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content1).unwrap();

    for model in &mutation_box {
        let name = model.split(".")
            .collect::<Vec<_>>();
        let content2 = format!("pub mod {};\n", &name[0]);
        let mut add_line = OpenOptions::new()
            .append(true)
            .open(file_path)
            .unwrap();
        add_line.write_all(content2.as_bytes()).unwrap();
    }
    for model in &mutation_box {
        let name = model.split(".")
            .collect::<Vec<_>>();
        let content2 = format!("\npub mod {};", &name[0]);
        let mut add_line = OpenOptions::new()
            .append(true)
            .open("entity/src/lib.rs")
            .unwrap();
        add_line.write_all(content2.as_bytes()).unwrap();
    }
    let mut add_line = OpenOptions::new()
        .append(true)
        .open(file_path)
        .unwrap();
    add_line.write_all("\n".as_bytes()).unwrap();
    for model in &mutation_box {
        let name = model.split(".")
            .collect::<Vec<_>>();
        let content3 = format!("pub use {}::{}Mutation;\n", &name[0], some_kind_of_uppercase_first_letter(&name[0]));
        let mut add_line = OpenOptions::new()
            .append(true)
            .open(file_path)
            .unwrap();
        add_line.write_all(content3.as_bytes()).unwrap();
    }
    
    let content4 = b"\n#[derive(async_graphql::MergedObject, Default)]";
    let mut add_line = OpenOptions::new()
        .append(true)
        .open(file_path)
        .unwrap();
    add_line.write_all(content4).unwrap();
    let capital_box = mutation_box.iter().cloned()
        .map(|i|{
            some_kind_of_uppercase_first_letter(&i)
        })
        .collect::<Vec<_>>();
    let last_line = capital_box.iter().cloned()
        .map(|i| { i + "Mutation" })
        .collect::<Vec<_>>();

    let content5 = format!("\npub struct Mutation({});", &last_line.join(", "));
    let mut add_line = OpenOptions::new()
        .append(true)
        .open(file_path)
        .unwrap();
    add_line.write_all(&content5.as_bytes()).unwrap();
    log_success(&format!("Successfully added mutation route: {}", &file_path)).await;
}


pub async fn process_create_query_route() {
    let dir = "src/graphql/query/";
    let files = read_dir(dir).await.unwrap();
    let query_box = files.iter().cloned()
        .filter(|i| i != "mod.rs")
        .map(|i| {i.replace(".rs", "")})
        .collect::<Vec<_>>();

    let file_path = "src/graphql/query/mod.rs";
    let content1 = b"use entity::async_graphql;\n\n";
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content1).unwrap();

    for model in &query_box {
        let name = model.split(".")
            .collect::<Vec<_>>();
        let content2 = format!("pub mod {};\n", &name[0]);
        let mut add_line = OpenOptions::new()
            .append(true)
            .open(file_path)
            .unwrap();
        add_line.write_all(content2.as_bytes()).unwrap();
    }
    let mut add_line = OpenOptions::new()
        .append(true)
        .open(file_path)
        .unwrap();
    add_line.write_all("\n".as_bytes()).unwrap();
    for model in &query_box {
        let name = model.split(".")
            .collect::<Vec<_>>();
        let content3 = format!("pub use {}::{}Query;\n", &name[0], some_kind_of_uppercase_first_letter(&name[0]));
        let mut add_line = OpenOptions::new()
            .append(true)
            .open(file_path)
            .unwrap();
        add_line.write_all(content3.as_bytes()).unwrap();
    }
    
    let content4 = b"\n#[derive(async_graphql::MergedObject, Default)]";
    let mut add_line = OpenOptions::new()
        .append(true)
        .open(file_path)
        .unwrap();
    add_line.write_all(content4).unwrap();
    let capital_box = query_box.iter().cloned()
        .map(|i|{
            some_kind_of_uppercase_first_letter(&i)
        })
        .collect::<Vec<_>>();
    let last_line = capital_box.iter().cloned()
        .map(|i| { i + "Query" })
        .collect::<Vec<_>>();

    let content5 = format!("\npub struct Query({});", &last_line.join(", "));
    let mut add_line = OpenOptions::new()
        .append(true)
        .open(file_path)
        .unwrap();
    add_line.write_all(&content5.as_bytes()).unwrap();
    log_success(&format!("Successfully added mutation route: {}", &file_path)).await;
}