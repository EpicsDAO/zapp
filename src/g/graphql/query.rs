use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use crate::g::{read_dir, to_upper_camel};
use crate::style_print::log_success;

pub(in crate::g) async fn process_graphql_query(model: &str, gen_path: &Path) {
    create_query(model, gen_path).await;
    register_query(model, gen_path).await;
}

async fn create_query(model: &str, gen_path: &Path) {
    let capital_model = to_upper_camel(model);
    let filename = format!("{}.rs", model);
    let query_dir = gen_path.join("src").join("graphql").join("query");
    fs::create_dir_all(query_dir.as_path()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let model_query = query_dir.join(filename);
    let file_content = format!(
        "use async_graphql::{{Context, Object, Result}};
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
}}",
        model, capital_model, capital_model, model, model, model, model, model, model
    );
    let mut file = fs::File::create(model_query.as_path()).unwrap();
    file.write_all(file_content.as_bytes()).unwrap();
    log_success(&format!("Successfully created `{}` GraphQL query file: {}", model, model_query.display())).await;
}

async fn register_query(model: &str, gen_path: &Path) {
    let query_dir = gen_path.join("src").join("graphql").join("query");
    let files = read_dir(query_dir.as_path()).await.unwrap();
    let mut query_box = files
        .iter()
        .cloned()
        .filter(|i| i != "mod.rs")
        .map(|i| i.replace(".rs", ""))
        .collect::<Vec<_>>();
    query_box.sort();

    let query_mod_file = query_dir.join("mod.rs");
    let content1 = b"use entity::async_graphql;\n\n";
    let mut file = fs::File::create(&query_mod_file).unwrap();
    file.write_all(content1).unwrap();

    for model in &query_box {
        let name = model.split(".").collect::<Vec<_>>();
        let content2 = format!("pub mod {};\n", &name[0]);
        let mut add_line = OpenOptions::new().append(true).open(query_mod_file.as_path()).unwrap();
        add_line.write_all(content2.as_bytes()).unwrap();
    }
    let mut add_line = OpenOptions::new().append(true).open(query_mod_file.as_path()).unwrap();
    add_line.write_all("\n".as_bytes()).unwrap();
    for model in &query_box {
        let name = model.split(".").collect::<Vec<_>>();
        let content3 = format!("pub use {}::{}Query;\n", &name[0], to_upper_camel(&name[0]));
        let mut add_line = OpenOptions::new().append(true).open(query_mod_file.as_path()).unwrap();
        add_line.write_all(content3.as_bytes()).unwrap();
    }
    let content4 = b"\n#[derive(async_graphql::MergedObject, Default)]";
    let mut add_line = OpenOptions::new().append(true).open(query_mod_file.as_path()).unwrap();
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
    let mut add_line = OpenOptions::new().append(true).open(query_mod_file.as_path()).unwrap();
    add_line.write_all(&content5.as_bytes()).unwrap();
    log_success(&format!(
        "Successfully registered GraphQL query for `{}` in {}", model, query_mod_file.display()
    ))
        .await;
}
