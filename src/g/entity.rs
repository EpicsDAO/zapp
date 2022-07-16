use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use crate::g::{read_dir, to_upper_camel};
use crate::style_print::log_success;

pub(in crate::g)  async fn process_entity(model: &str, gen_path: &Path) {
    create_entity(model, gen_path).await;
    register_entity(model, gen_path).await;
}

async fn create_entity(model: &str, gen_path: &Path) {
    let capital_model = to_upper_camel(model);
    let filename = format!("{}.rs", model);
    let file_dir = gen_path.join("entity").join("src");
    fs::create_dir_all(file_dir.as_path()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let file_path = file_dir.join(&filename);

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
    pub id: i32,
    #[sea_orm(indexed)]
    pub created_at: DateTime,
    #[sea_orm(indexed)]
    pub updated_at: DateTime
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
    log_success(&format!("Successfully created `{}` entity file: {}", model, file_path.display())).await;
}

async fn register_entity(model: &str, gen_path: &Path) {
    let entity_dir = gen_path.join("entity").join("src");
    let entity_files = read_dir(entity_dir.as_path()).await.unwrap();
    let mut entity_box = entity_files
        .iter()
        .cloned()
        .filter(|i| i != "lib.rs")
        .map(|i| i.replace(".rs", ""))
        .collect::<Vec<_>>();
    entity_box.sort();

    let entity_src_lib = gen_path.join("entity").join("src").join("lib.rs");
    let content1 = b"pub use async_graphql;\n";
    let mut file = fs::File::create(entity_src_lib.as_path()).unwrap();
    file.write_all(content1).unwrap();
    for model in &entity_box {
        let name = model.split(".").collect::<Vec<_>>();
        let content2 = format!("\npub mod {};", &name[0]);
        let mut add_line = OpenOptions::new().append(true).open(entity_src_lib.as_path()).unwrap();
        add_line.write_all(content2.as_bytes()).unwrap();
    }
    log_success(&format!("Successfully registered `{}` entity in {}", model, entity_src_lib.display())).await;
}