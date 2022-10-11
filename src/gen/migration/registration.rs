use crate::gen::{emit_generated_code, read_dir};
use crate::style_print::log_success;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;

pub(super) fn register_migration(model: &str, migration_src_dir: &Path) {
    let file_content_tokens = register_migration_tokens(&migration_src_dir);

    let file_path = emit_generated_code(migration_src_dir, "lib.rs", &file_content_tokens);

    log_success(&format!(
        "Successfully registered migration file for model `{}` in {}",
        model,
        file_path.display()
    ));
}

fn register_migration_tokens(migration_src_dir: &&Path) -> TokenStream {
    let files = read_dir(&migration_src_dir).unwrap();
    let mut files_box = files
        .iter()
        .cloned()
        .filter(|i| i != "lib.rs")
        .filter(|i| i != "main.rs")
        .map(|i| i.replace(".rs", ""))
        .collect::<Vec<_>>();
    files_box.sort();

    let modules = files_box
        .iter()
        .map(|i| format_ident!("{}", i))
        .collect::<Vec<_>>();

    quote! {
        pub use sea_orm_migration::prelude::*;

        pub struct Migrator;

        #(mod #modules;)*

        #[async_trait::async_trait]
        impl MigratorTrait for Migrator {
            fn migrations() -> Vec<Box<dyn MigrationTrait>> {
                vec![
                    #(Box::new(#modules::Migration))*
                ]
            }
        }
    }
}
