use crate::gen::{emit_generated_code, read_dir};
use crate::style_print::log_success;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;

pub(super) fn register_entity(model: &str, entity_src_dir: &Path) {
    let file_content_tokens = register_entity_tokens(entity_src_dir);

    let file_path = emit_generated_code(entity_src_dir, "lib.rs", &file_content_tokens);

    log_success(&format!(
        "Successfully registered `{}` entity in {}",
        model,
        file_path.display()
    ));
}

fn register_entity_tokens(entity_src_dir: &Path) -> TokenStream {
    let entity_files = read_dir(entity_src_dir).unwrap();
    let mut entity_box = entity_files
        .iter()
        .cloned()
        .filter(|i| i != "lib.rs")
        .map(|i| i.replace(".rs", ""))
        .collect::<Vec<_>>();
    entity_box.sort();

    let modules = entity_box
        .iter()
        .map(|i| format_ident!("{}", i))
        .collect::<Vec<_>>();

    quote! {
        pub use async_graphql;
        #(pub mod #modules;)*
    }
}
