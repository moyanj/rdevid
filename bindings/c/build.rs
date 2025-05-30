extern crate cbindgen;

use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_WORKSPACE_DIR")
        .or_else(|_| env::var("CARGO_MANIFEST_DIR"))
        .unwrap();
    let profile = env::var("PROFILE").unwrap(); // debug/release
    let target_dir = Path::new(&crate_dir).join("../../target").join(&profile);

    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file(target_dir.join("rdevid.h"));
}
