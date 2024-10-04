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

    let mut build_opts = BuildOptions::default();
    build_opts.path = Some(PathBuf::from("../lc3b"));
    build_opts.out_dir = "../lc3b/pkg".into();
    build_opts.disable_dts = true;
    build_opts.target = Target::Web;
    let mut build = Build::try_from_opts(build_opts).unwrap();
    build.run().expect("wasm-pack build");
}
