use crate::gen::{emit_generated_code, to_upper_camel};
use crate::style_print::log_success;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;

pub(super) fn create_entity(model: &str, entity_src_dir: &Path) {
    let file_content_tokens = create_model_tokens(model);

    let file_path = emit_generated_code(
        entity_src_dir,
        &format!("{}.rs", model),
        &file_content_tokens,
    );

    log_success(&format!(
        "Successfully created `{}` entity file: {}",
        model,
        file_path.display()
    ));
}

fn create_model_tokens(model_str: &str) -> TokenStream {
    let model = format_ident!("{}", model_str);
    let cap_model = format_ident!("{}", to_upper_camel(model_str));

    quote! {
        use async_graphql::*;
        use sea_orm::{entity::prelude::*, DeleteMany};
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
        #[sea_orm(table_name = #model)]
        #[graphql(concrete(name = "#cap_model", params()))]
        pub struct Model {
            #[sea_orm(primary_key)]
            #[serde(skip_deserializing)]
            pub id: i32,
            #[sea_orm(indexed)]
            pub created_at: DateTime,
            #[sea_orm(indexed)]
            pub updated_at: DateTime
        }

        #[derive(Copy, Clone, Debug, EnumIter)]
        pub enum Relation {}

        impl RelationTrait for Relation {
            fn def(&self) -> RelationDef {
                panic!("No RelationDef")
            }
        }

        impl ActiveModelBehavior for ActiveModel {}

        impl Entity {
            pub fn find_by_id(id: i32) -> Select<Entity> {
                Self::find().filter(Column::Id.eq(id))
            }

            pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
                Self::delete_many().filter(Column::Id.eq(id))
            }
        }
    }
}
