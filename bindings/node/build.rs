use std::fs;

fn main() {
    node_bindgen::build::configure();
    fs::copy("metadata/package.json", "dist/package.json").expect("Copy failed");
    fs::copy("metadata/rdevid.d.ts", "dist/rdevid.d.ts").expect("Copy failed");
}
