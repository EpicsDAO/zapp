use crate::gen::{emit_generated_code, to_upper_camel};
use crate::style_print::log_success;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;

pub(super) fn create_query(model: &str, query_dir: &Path) {
    let file_content_tokens = create_query_tokens(model);

    let file_path = emit_generated_code(query_dir, &format!("{}.rs", model), &file_content_tokens);

    log_success(&format!(
        "Successfully created `{}` GraphQL query file: {}",
        model,
        file_path.display()
    ));
}

fn create_query_tokens(model_str: &str) -> TokenStream {
    let model = format_ident!("{}", model_str);
    let model_query = format_ident!("{}Query", to_upper_camel(model_str));
    let get_models = format_ident!("get_{}", model);
    let get_by_id = format_ident!("get_{}_by_id", model);

    quote! {
        use async_graphql::{Context, Object, Result};
        use entity::{async_graphql, #model};
        use sea_orm::EntityTrait;
        use crate::db::Database;

        #[derive(Default)]
        pub struct #model_query;

        #[Object]
        impl model_query {
            async fn #get_models(&self, ctx: &Context<'_>) -> Result<Vec<#model::Model>> {
                let db = ctx.data::<Database>().unwrap();

                Ok(#model::Entity::find()
                    .all(db.get_connection())
                    .await
                    .map_err(|e| e.to_string())?)
            }

            async fn #get_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<#model::Model>> {
                let db = ctx.data::<Database>().unwrap();

                Ok(#model::Entity::find_by_id(id)
                    .one(db.get_connection())
                    .await
                    .map_err(|e| e.to_string())?)
            }
        }
    }
}
