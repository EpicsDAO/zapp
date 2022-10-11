use crate::gen::{emit_generated_code, read_dir, to_upper_camel};
use crate::style_print::log_success;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;

pub(super) fn register_mutation(model: &str, mutation_dir: &Path) {
    let file_content_tokens = register_mutation_tokens(&mutation_dir);

    let file_path = emit_generated_code(mutation_dir, "mod.rs", &file_content_tokens);

    log_success(&format!(
        "Successfully registered GraphQL mutation for `{}` in {}",
        model,
        file_path.display()
    ))
}

fn register_mutation_tokens(mutation_dir: &Path) -> TokenStream {
    let mutation_files = read_dir(mutation_dir).unwrap();
    let mut mutation_box = mutation_files
        .iter()
        .cloned()
        .filter(|i| i != "mod.rs")
        .filter(|i| i != "common.rs")
        .map(|i| i.replace(".rs", ""))
        .collect::<Vec<_>>();
    mutation_box.sort();

    let modules = mutation_box
        .iter()
        .map(|i| format_ident!("{}", i))
        .collect::<Vec<_>>();

    let members = mutation_box
        .iter()
        .map(|s| (format_ident!("{}Mutation", to_upper_camel(s))))
        .collect::<Vec<_>>();

    quote! {
        use entity::async_graphql;

        pub mod common;
        #(pub mod #modules)*;
        #(pub use #modules::#members)*;

        #[derive(async_graphql::MergedObject, Default)]
        pub struct Mutation(#(#members),*);

    }
}
