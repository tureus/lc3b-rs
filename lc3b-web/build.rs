use std::path::PathBuf;

use wasm_pack::command::build::{Build, BuildOptions, Target};

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lc3b_path = PathBuf::from(manifest_dir).join("../lc3b");

    for path in glob::glob("../lc3b/src/**/*.rs").unwrap() {
        println!("cargo::rerun-if-changed={}", path.unwrap().display())
    }
    for path in glob::glob("../lc3b/Cargo.*").unwrap() {
        println!("cargo::rerun-if-changed={}", path.unwrap().display())
    }
    for path in glob::glob("../lc3b/pkg/lc3b_bg.wasm").unwrap() {
        println!("cargo::rerun-if-changed={}", path.unwrap().display())
    }

    let build_opts = BuildOptions {
        path: Some(lc3b_path.clone()),
        out_dir: lc3b_path.join("pkg").to_str().unwrap().into(),
        disable_dts: true,
        target: Target::Web,
        ..Default::default()
    };
    let mut build = Build::try_from_opts(build_opts).unwrap();
    build.run().expect("wasm-pack build");
}
