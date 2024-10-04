use std::path::PathBuf;

use wasm_pack::command::build::{Build, BuildOptions, Target};

fn main() {
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
        path: Some(PathBuf::from("../lc3b")),
        out_dir: "../lc3b/pkg".into(),
        disable_dts: true,
        target: Target::Web,
        ..Default::default()
    };
    let mut build = Build::try_from_opts(build_opts).unwrap();
    build.run().expect("wasm-pack build");
}
