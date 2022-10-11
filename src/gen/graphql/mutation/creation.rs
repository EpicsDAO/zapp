use crate::{
    gen::{emit_generated_code, to_upper_camel},
    style_print::*,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;
use std::str;

pub(super) fn create_mutation(model: &str, mutation_dir: &Path) {
    let file_content_tokens = create_mutation_tokens(model);

    let file_path =
        emit_generated_code(mutation_dir, &format!("{}.rs", model), &file_content_tokens);

    log_success(&format!(
        "Successfully created `{}` GraphQL mutation file: {}",
        model,
        file_path.display()
    ))
}

fn create_mutation_tokens(model: &str) -> TokenStream {
    let model_name = format_ident!("{}", model);
    let create_model_input = format_ident!("Create{}Input", to_upper_camel(model));
    let mutation_struct = format_ident!("{}Mutation", to_upper_camel(model));
    let create_function = format_ident!("create_{}", model);
    let update_function = format_ident!("update_{}", model);
    let delete_function = format_ident!("delete_{}", model);

    quote! {
        use async_graphql::{Context, Object, Result, Error};
        use entity::async_graphql::{{self, InputObject}};
        use entity::#model_name;
        use chrono::Utc;
        use sea_orm::{ActiveModelTrait, Set};
        use crate::graphql::mutation::common::*;
        use crate::db::Database;

        #[derive(InputObject)]
        pub struct #create_model_input {
            pub id: i32,
        }

        #[derive(Default)]
        pub struct #mutation_struct;

        #[Object]
        impl #mutation_struct {
            pub async fn #create_function(
                &self,
                ctx: &Context<'_>,
                input: #create_model_input,
            ) -> Result<#model_name::Model> {
                let db = ctx.data::<Database>().unwrap();
                let naive_date_time = Utc::now().naive_utc();

                // Define schema here
                let #model_name = #model_name::ActiveModel {
                    id: Set(input.id),
                    created_at: Set(naive_date_time),
                    updated_at: Set(naive_date_time),
                    ..Default::default()
                };

                Ok(#model_name.insert(db.get_connection()).await?)
            }

            pub async fn #update_function(
                &self,
                ctx: &Context<'_>,
                id: i32,
            ) -> Result<#model_name::Model, Error> {
                let db = ctx.data::<Database>().unwrap();
                let naive_date_time = Utc::now().naive_utc();
                let #model_name: Option<#model_name::Model> =
                    #model_name::Entity::find_by_id(id)
                        .one(db.get_connection())
                        .await?;
                let mut #model_name: #model_name::ActiveModel = #model_name.unwrap().into();
                #model_name.updated_at = Set(naive_date_time);
                let #model_name: #model_name::Model =
                    #model_name.update(db.get_connection()).await?;

                Ok(#model_name)
            }

            pub async fn #delete_function(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
                let db = ctx.data::<Database>().unwrap();

                let res = #model_name::Entity::delete_by_id(id)
                    .exec(db.get_connection())
                    .await?;

                if res.rows_affected <= 1 {
                    Ok(DeleteResult {
                        success: true,
                        rows_affected: res.rows_affected,
                    })
                } else {
                    unimplemented!()
                }
            }
        }

    }
}
