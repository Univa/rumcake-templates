use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Dependency {
    String(String),
    Object { features: Option<Vec<String>> },
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    dependencies: HashMap<String, Dependency>,
}

fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    let mut g = TokenStream::new();

    let rumcake_features = find_rumcake_features();

    if rumcake_features.contains(&"vial".to_string()) {
        println!("cargo:rustc-cfg=vial");

        generate_keyboard_definition(&mut g);

        let out_file = Path::new(&env::var_os("OUT_DIR").unwrap()).join("_generated.rs");
        fs::write(out_file, g.to_string()).unwrap();
    }
}

fn generate_keyboard_definition(g: &mut TokenStream) {
    let compressed_definition = lzma::compress(
        json::stringify(json::parse(include_str!("src/definition.json")).unwrap()).as_bytes(),
        6,
    )
    .unwrap();
    let definition_bytes = compressed_definition.len();

    // For the type below: u8 instead of char, bc char is 1 byte in C, but 4 bytes in Rust
    g.extend(quote! {
        pub const GENERATED_KEYBOARD_DEFINITION: [u8; #definition_bytes] = [#(#compressed_definition),*];
    });
}

fn find_rumcake_features() -> Vec<String> {
    let cargo_toml: CargoToml = toml::from_str(include_str!("Cargo.toml")).unwrap();

    let rumcake = match cargo_toml.dependencies.get("rumcake") {
        Some(rumcake) => rumcake,
        None => panic!("rumcake dependency not found."),
    };

    let features: Vec<String> = match rumcake {
        Dependency::Object {
            features: Some(features),
        } => features.clone(),
        _ => vec![],
    };

    features
}
