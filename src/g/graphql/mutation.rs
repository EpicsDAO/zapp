use crate::style_print::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::str;
use crate::g::{read_dir, to_upper_camel};

pub(in crate::g) async fn process_graphql_mutation(model: &str, gen_path: &Path) {
    create_mutation(model, gen_path).await;
    register_mutation(model, gen_path).await;
}

async fn create_mutation(model: &str, gen_path: &Path) {
    let capital_model = to_upper_camel(model);
    let filename = format!("{}.rs", model);
    let file_dir = gen_path.join("src").join("graphql").join("mutation");
    fs::create_dir_all(file_dir.as_path()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let file_path = file_dir.join(filename);

    let file_content = format!(
        "use async_graphql::{{Context, Object, Result, Error}};
use entity::async_graphql::{{self, InputObject}};
use entity::{};
use chrono::Utc;
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
        let naive_date_time = Utc::now().naive_utc();

        // Define schema here
        let {} = {}::ActiveModel {{
            id: Set(input.id),
            created_at: Set(naive_date_time),
            updated_at: Set(naive_date_time),
            ..Default::default()
        }};

        Ok({}.insert(db.get_connection()).await?)
    }}

    pub async fn update_{}(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<{}::Model, Error> {{
        let db = ctx.data::<Database>().unwrap();
        let naive_date_time = Utc::now().naive_utc();
        let {}: Option<{}::Model> =
            {}::Entity::find_by_id(id)
                .one(db.get_connection())
                .await?;
        let mut {}: {}::ActiveModel = {}.unwrap().into();
        {}.updated_at = Set(naive_date_time);
        let {}: {}::Model =
            {}.update(db.get_connection()).await?;

        Ok({})
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
        model,
        model,
        model,
        model,
        model,
        model,
        model,
        model,
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
        "Successfully created `{}` GraphQL mutation file: {}", model, file_path.display()
    ))
        .await;
}


async fn register_mutation(model: &str, gen_path: &Path) {
    let mutation_dir = gen_path.join("src").join("graphql").join("mutation");
    let mutation_files = read_dir(mutation_dir.as_path()).await.unwrap();
    let mut mutation_box = mutation_files
        .iter()
        .cloned()
        .filter(|i| i != "mod.rs")
        .filter(|i| i != "common.rs")
        .map(|i| i.replace(".rs", ""))
        .collect::<Vec<_>>();
    mutation_box.sort();

    let file_path = mutation_dir.join("mod.rs");
    let content1 = b"use entity::async_graphql;\n\npub mod common;\n";
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content1).unwrap();

    for model in &mutation_box {
        let name = model.split(".").collect::<Vec<_>>();
        let content2 = format!("pub mod {};\n", &name[0]);
        let mut add_line = OpenOptions::new().append(true).open(file_path.as_path()).unwrap();
        add_line.write_all(content2.as_bytes()).unwrap();
    }


    let mut add_line = OpenOptions::new().append(true).open(file_path.as_path()).unwrap();
    add_line.write_all("\n".as_bytes()).unwrap();
    for model in &mutation_box {
        let name = model.split(".").collect::<Vec<_>>();
        let content3 = format!(
            "pub use {}::{}Mutation;\n",
            &name[0],
            to_upper_camel(&name[0])
        );
        let mut add_line = OpenOptions::new().append(true).open(file_path.as_path()).unwrap();
        add_line.write_all(content3.as_bytes()).unwrap();
    }
    let content4 = b"\n#[derive(async_graphql::MergedObject, Default)]";
    let mut add_line = OpenOptions::new().append(true).open(file_path.as_path()).unwrap();
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
    let mut add_line = OpenOptions::new().append(true).open(file_path.as_path()).unwrap();
    add_line.write_all(&content5.as_bytes()).unwrap();
    log_success(&format!(
        "Successfully registered GraphQL mutation for `{}` in {}", model, file_path.display()
    ))
        .await;
}
