// build.rs

use std::process::Command;
use std::env;
use std::path::Path;


fn main() {
    let package_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let js_dir = "/static/js".to_string();

    let work_dir = package_dir + & js_dir;

    Command::new("node").args(&["run", "build/build.js"])
        .current_dir(&Path::new(&work_dir))
        .status().unwrap();


}