use crate::gen::{emit_generated_code, read_dir, to_upper_camel};
use crate::style_print::log_success;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;

pub(super) fn register_query(model: &str, query_dir: &Path) {
    let file_content_tokens = register_query_tokens(query_dir);

    let file_path = emit_generated_code(query_dir, "mod.rs", &file_content_tokens);

    log_success(&format!(
        "Successfully registered GraphQL query for `{}` in {}",
        model,
        file_path.display()
    ));
}

fn register_query_tokens(query_dir: &Path) -> TokenStream {
    let files = read_dir(query_dir).unwrap();
    let mut query_box = files
        .iter()
        .cloned()
        .filter(|i| i != "mod.rs")
        .map(|i| i.replace(".rs", ""))
        .collect::<Vec<_>>();
    query_box.sort();

    let modules = query_box
        .iter()
        .map(|i| format_ident!("{}", i))
        .collect::<Vec<_>>();

    let members = query_box
        .iter()
        .map(|s| {
            let name = to_upper_camel(s);
            format_ident!("{}Query", name)
        })
        .collect::<Vec<_>>();

    quote! {
        use entity::async_graphql;

        #(pub mod #modules)*;
        #(pub use #modules::#members)*;

        #[derive(async_graphql::MergedObject, Default)]
        pub struct Query(#(#members),*);
    }
}
