use std::fs;
use std::fs::{OpenOptions};
use std::io::Write;
use std::path::Path;
use chrono::NaiveDateTime;
use quote::{format_ident, quote};
use syn::File;
use crate::g::read_dir;
use crate::style_print::log_success;

pub(in crate::g) async fn process_migration(model: &str, dt: NaiveDateTime, gen_path: &Path) {
    create_migration(model, dt, gen_path).await;
    register_migration(model, gen_path).await;
}

async fn create_migration(model: &str, dt: NaiveDateTime, gen_path: &Path) {
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

    // create tokens used in the quasi-templating below
    let model_name_ident = format_ident!("{}", model);


    // template the rust code
    let file_content_tokens = quote! {
        use entity::#model_name_ident;
        use sea_orm::{DbBackend, EntityTrait, Schema};
        use sea_orm_migration::prelude::*;

        pub struct Migration;

        fn get_seaorm_create_stmt<E: EntityTrait>(e: E) -> TableCreateStatement {
            let schema = Schema::new(DbBackend::Postgres);

            schema
                .create_table_from_entity(e)
                .if_not_exists()
                .to_owned()
        }

        fn get_seaorm_drop_stmt<E: EntityTrait>(e: E) -> TableDropStatement {
            Table::drop().table(e).if_exists().to_owned()
        }

        impl MigrationName for Migration {
            fn name(&self) -> &str {
                #filename
            }
        }

        #[async_trait::async_trait]
        impl MigrationTrait for Migration {
            async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                let stmts = vec![get_seaorm_create_stmt(#model_name_ident::Entity)];

                for stmt in stmts {
                    manager.create_table(stmt.to_owned()).await?;
                }

                Ok(())
            }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            let stmts = vec![get_seaorm_drop_stmt(#model_name_ident::Entity)];

            for stmt in stmts {
                manager.drop_table(stmt.to_owned()).await?;
            }

            Ok(())
        }
    }
};
    // formatting and pretty printing
    let syntax_tree = syn::parse_str::<File>(&file_content_tokens.to_string()).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    // write the rust code to the file
    let migration_src_dir = gen_path.join("migration").join("src");
    fs::create_dir_all(migration_src_dir.as_path()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    let file_path = migration_src_dir.join(filename + ".rs");
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(formatted.as_bytes()).unwrap();

    log_success(&format!(
        "Successfully created migration file for model `{}`: {}", model, file_path.display()
    ))
        .await;
}

async fn register_migration(model: &str, gen_path: &Path) {
    let migration_src_dir = gen_path.join("migration").join("src");
    let content1 = b"pub use sea_orm_migration::prelude::*;\n\npub struct Migrator;\n\n";

    let file_path = migration_src_dir.join("lib.rs");
    let files = read_dir(&migration_src_dir).await.unwrap();
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

    log_success(&format!("Successfully registered migration file for model `{}` in {}", model, file_path.display())).await;
}
