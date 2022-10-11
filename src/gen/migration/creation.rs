use crate::gen::{emit_generated_code, read_dir};
use crate::style_print::log_success;
use chrono::NaiveDateTime;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;

pub(super) fn create_migration(model: &str, dt: NaiveDateTime, migration_src_dir: &Path) {
    let mirgration_name = format!(
        "m{}{}{}_{}{}{}_create_{}_table",
        dt.format("%Y"),
        dt.format("%m"),
        dt.format("%d"),
        dt.format("%H"),
        dt.format("%M"),
        dt.format("%S"),
        model
    );

    let file_content_tokens = create_migration_tokens(&model, &mirgration_name);

    let file_path = emit_generated_code(
        migration_src_dir,
        &format!("{}.rs", mirgration_name),
        &file_content_tokens,
    );

    log_success(&format!(
        "Successfully created migration file for model `{}`: {}",
        model,
        file_path.display()
    ));
}

fn create_migration_tokens(model: &str, mirgration_name_str: &str) -> TokenStream {
    let model = format_ident!("{}", model);
    let migration_name_lit = syn::LitStr::new(mirgration_name_str, proc_macro2::Span::call_site());

    quote! {
        use entity::#model;
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
                #migration_name_lit
            }
        }

        #[async_trait::async_trait]
        impl MigrationTrait for Migration {
            async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                let stmts = vec![get_seaorm_create_stmt(#model::Entity)];

                for stmt in stmts {
                    manager.create_table(stmt.to_owned()).await?;
                }

                Ok(())
            }

            async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                let stmts = vec![get_seaorm_drop_stmt(#model::Entity)];

                for stmt in stmts {
                    manager.drop_table(stmt.to_owned()).await?;
                }
                Ok(())
            }
        }
    }
}
