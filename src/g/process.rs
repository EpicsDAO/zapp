use crate::style_print::*;
use chrono::Local;
use convert_case::{Case, Casing};
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;
use std::str;

pub fn to_upper_camel(s: &str) -> String {
    s.to_case(Case::UpperCamel)
}

pub async fn process_create_migration(model: &str) {
    let dt = Local::now();
    let filename = format!(
        "m{}{}{}_{}{}{}_create_{}_table",
        dt.format("%Y"),
        dt.format("%m"),
        dt.format("%d"),
        dt.format("%H"),
        dt.format("%M"),
        dt.format("%S"),
        model
    );
    let file_dir = "migration/src/";
    fs::create_dir_all(file_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let file_path = String::from(file_dir) + &filename + ".rs";
    let file_content = format!(
        "use entity::{};
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
        \"m{}{}{}_{}{}{}_create_{}_table\"
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
}}",
        model,
        dt.format("%Y"),
        dt.format("%m"),
        dt.format("%d"),
        dt.format("%H"),
        dt.format("%M"),
        dt.format("%S"),
        model,
        model,
        model
    );
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(file_content.as_bytes()).unwrap();
    log_success(&format!(
        "Successfully created migration file: {}",
        &file_path
    ))
    .await;
    // Edit migration/src/lib.rs
    edit_migration_lib().await;
}

pub async fn edit_migration_lib() {
    let content1 = b"pub use sea_orm_migration::prelude::*;\n\npub struct Migrator;\n\n";
    let dir = "migration/src/";
    let file_path = String::from(dir) + "lib.rs";
    let files = read_dir(&dir).await.unwrap();
    let mut files_box = files
        .iter()
        .cloned()
        .filter(|i| i != "lib.rs")
        .filter(|i| i != "main.rs")
        .map(|i| i.replace(".rs", ""))
        .collect::<Vec<_>>();
    files_box.sort();
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content1).unwrap();

    for model in &files_box {
        let content2 = format!("mod {};\n", model);
        let mut add_line = OpenOptions::new().append(true).open(&file_path).unwrap();
        add_line.write_all(content2.as_bytes()).unwrap();
    }

    let content3 = b"\n#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![\n             ";
    let mut add_line = OpenOptions::new().append(true).open(&file_path).unwrap();
    add_line.write_all(content3).unwrap();

    let migration_box = &files_box
        .iter()
        .cloned()
        .map(|i| String::from("Box::new(") + &i + "::Migration)")
        .collect::<Vec<_>>();

    let content4 = format!("{}", &migration_box.join(", "));
    let mut add_line = OpenOptions::new().append(true).open(&file_path).unwrap();
    add_line.write_all(&content4.as_bytes()).unwrap();

    let content5 = b"\n               ]
        }
}";
    let mut add_line = OpenOptions::new().append(true).open(&file_path).unwrap();
    add_line.write_all(content5).unwrap();
    log_success("Successfully added route to `migration/src/lib.rs`").await;
}

pub async fn process_create_entity(model: &str) {
    let capital_model = to_upper_camel(model);
    let filename = format!("{}.rs", model);
    let file_dir = "entity/src/";
    fs::create_dir_all(file_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let file_path = String::from(file_dir) + &filename;
    let file_content = format!(
        "use async_graphql::*;
use sea_orm::{{entity::prelude::*, DeleteMany}};
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = \"{}s\")]
#[graphql(concrete(name = \"{}\", params()))]
pub struct Model {{
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32
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
}}",
        model, capital_model
    );
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(file_content.as_bytes()).unwrap();
    log_success(&format!("Successfully created entity file: {}", &file_path)).await;
}

pub async fn process_create_mutation(model: &str) {
    let capital_model = to_upper_camel(model);
    let filename = format!("{}.rs", model);
    let file_dir = "src/graphql/mutation/";
    fs::create_dir_all(file_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let file_path = String::from(file_dir) + &filename;
    let file_content = format!(
        "use async_graphql::{{Context, Object, Result}};
use entity::async_graphql::{{self, InputObject}};
use entity::{};
use sea_orm::{{ActiveModelTrait, Set}};
use crate::graphql::mutation::common::*;
use crate::db::Database;


#[derive(InputObject)]
pub struct Create{}Input {{
    pub id: i32
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
            id: Set(input.id),
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
}}",
        model,
        capital_model,
        capital_model,
        capital_model,
        model,
        capital_model,
        model,
        model,
        model,
        model,
        model,
        model
    );
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(file_content.as_bytes()).unwrap();
    log_success(&format!(
        "Successfully created mutation file: {}",
        &file_path
    ))
    .await;
}

pub async fn process_create_query(model: &str) {
    let capital_model = to_upper_camel(model);
    let filename = format!("{}.rs", model);
    let file_dir = "src/graphql/query/";
    fs::create_dir_all(file_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let file_path = String::from(file_dir) + &filename;
    let file_content = format!(
        "use async_graphql::{{Context, Object, Result}};
use entity::{{async_graphql, {}}};
use sea_orm::EntityTrait;
use crate::graphql::mutation::common::*;
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
}}",
        model, capital_model, capital_model, model, model, model, model, model, model
    );
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
    let mut mutation_box = files
        .iter()
        .cloned()
        .filter(|i| i != "mod.rs")
        .filter(|i| i != "common.rs")
        .map(|i| i.replace(".rs", ""))
        .collect::<Vec<_>>();
    mutation_box.sort();

    let file_path = "src/graphql/mutation/mod.rs";
    let content1 = b"use entity::async_graphql;\n\npub mod common;\n";
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content1).unwrap();

    for model in &mutation_box {
        let name = model.split(".").collect::<Vec<_>>();
        let content2 = format!("pub mod {};\n", &name[0]);
        let mut add_line = OpenOptions::new().append(true).open(file_path).unwrap();
        add_line.write_all(content2.as_bytes()).unwrap();
    }

    let file_path2 = "entity/src/lib.rs";
    let content1 = b"pub use async_graphql;\n";
    let mut file = fs::File::create(&file_path2).unwrap();
    file.write_all(content1).unwrap();
    for model in &mutation_box {
        let name = model.split(".").collect::<Vec<_>>();
        let content2 = format!("\npub mod {};", &name[0]);
        let mut add_line = OpenOptions::new().append(true).open(file_path2).unwrap();
        add_line.write_all(content2.as_bytes()).unwrap();
    }
    log_success("Successfully added route to `entity/src/lib.rs`").await;
    let mut add_line = OpenOptions::new().append(true).open(file_path).unwrap();
    add_line.write_all("\n".as_bytes()).unwrap();
    for model in &mutation_box {
        let name = model.split(".").collect::<Vec<_>>();
        let content3 = format!(
            "pub use {}::{}Mutation;\n",
            &name[0],
            to_upper_camel(&name[0])
        );
        let mut add_line = OpenOptions::new().append(true).open(file_path).unwrap();
        add_line.write_all(content3.as_bytes()).unwrap();
    }
    let content4 = b"\n#[derive(async_graphql::MergedObject, Default)]";
    let mut add_line = OpenOptions::new().append(true).open(file_path).unwrap();
    add_line.write_all(content4).unwrap();
    let capital_box = mutation_box
        .iter()
        .cloned()
        .map(|i| to_upper_camel(&i))
        .collect::<Vec<_>>();
    let last_line = capital_box
        .iter()
        .cloned()
        .map(|i| i + "Mutation")
        .collect::<Vec<_>>();

    let content5 = format!("\npub struct Mutation({});", &last_line.join(", "));
    let mut add_line = OpenOptions::new().append(true).open(file_path).unwrap();
    add_line.write_all(&content5.as_bytes()).unwrap();
    log_success(&format!(
        "Successfully added mutation route: {}",
        &file_path
    ))
    .await;
}

pub async fn process_create_query_route() {
    let dir = "src/graphql/query/";
    let files = read_dir(dir).await.unwrap();
    let mut query_box = files
        .iter()
        .cloned()
        .filter(|i| i != "mod.rs")
        .filter(|i| i != "common.rs")
        .map(|i| i.replace(".rs", ""))
        .collect::<Vec<_>>();
    query_box.sort();

    let file_path = "src/graphql/query/mod.rs";
    let content1 = b"use entity::async_graphql;\n\npub mod common;\n";
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content1).unwrap();

    for model in &query_box {
        let name = model.split(".").collect::<Vec<_>>();
        let content2 = format!("pub mod {};\n", &name[0]);
        let mut add_line = OpenOptions::new().append(true).open(file_path).unwrap();
        add_line.write_all(content2.as_bytes()).unwrap();
    }
    let mut add_line = OpenOptions::new().append(true).open(file_path).unwrap();
    add_line.write_all("\n".as_bytes()).unwrap();
    for model in &query_box {
        let name = model.split(".").collect::<Vec<_>>();
        let content3 = format!("pub use {}::{}Query;\n", &name[0], to_upper_camel(&name[0]));
        let mut add_line = OpenOptions::new().append(true).open(file_path).unwrap();
        add_line.write_all(content3.as_bytes()).unwrap();
    }
    let content4 = b"\n#[derive(async_graphql::MergedObject, Default)]";
    let mut add_line = OpenOptions::new().append(true).open(file_path).unwrap();
    add_line.write_all(content4).unwrap();
    let capital_box = query_box
        .iter()
        .cloned()
        .map(|i| to_upper_camel(&i))
        .collect::<Vec<_>>();
    let last_line = capital_box
        .iter()
        .cloned()
        .map(|i| i + "Query")
        .collect::<Vec<_>>();

    let content5 = format!("\npub struct Query({});", &last_line.join(", "));
    let mut add_line = OpenOptions::new().append(true).open(file_path).unwrap();
    add_line.write_all(&content5.as_bytes()).unwrap();
    log_success(&format!(
        "Successfully added mutation route: {}",
        &file_path
    ))
    .await;
}
