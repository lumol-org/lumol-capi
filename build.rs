extern crate cbindgen;

use cbindgen::{Config, Language};
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let header = "#ifdef __cplusplus\nextern \"C\" {\n#endif";
    let trailer = "#ifdef __cplusplus\n}\n#endif";

    let config = Config {
        header: Some(header.into()),
        trailer: Some(trailer.into()),
        include_guard: Some("LUMOL_CAPI_H".into()),
        autogen_warning: Some("/* Automatically generated file, do not edit */".into()),
        include_version: true,
        language: Language::C,
        ..Default::default()
    };

    match cbindgen::generate_with_config(&crate_dir, config) {
        Ok(bindings) => bindings.write_to_file("include/lumol.h"),
        Err(_) => {
            // Do nothing: it is very probable that the code does not compile
        }
    }
}
