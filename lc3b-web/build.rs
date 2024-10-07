use std::path::PathBuf;

use wasm_pack::command::build::{Build, BuildOptions, Target};

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));

    let parent_path = PathBuf::from(&manifest_dir).join("..");
    let sibling_folders = list_folders(&parent_path.display().to_string()).unwrap();

    let lc3b_directory = sibling_folders
        .iter()
        .filter(|f| !f.contains("-assembler") && !f.contains("-isa") && !f.contains("-web"))
        .find(|f| f.starts_with("lc3b"))
        .expect("a sibling lc3b directory");

    let lc3b_web_directory = sibling_folders
        .iter()
        .filter(|f| f.contains("-web"))
        .find(|f| f.starts_with("lc3b"))
        .expect("a lc3b-web directory");
    let lc3b_web_directory_path = PathBuf::from("..").join(lc3b_web_directory);

    let lc3b_path = PathBuf::from(manifest_dir).join("..").join(lc3b_directory);

    for path in glob::glob(&lc3b_path.display().to_string()).unwrap() {
        println!("cargo::rerun-if-changed={}", path.unwrap().display())
    }
    for path in glob::glob(&lc3b_path.join("Cargo.*").display().to_string()).unwrap() {
        println!("cargo::rerun-if-changed={}", path.unwrap().display())
    }
    for path in glob::glob(&lc3b_path.join("pkg/lc3b_bg.wasm").display().to_string()).unwrap() {
        println!("cargo::rerun-if-changed={}", path.unwrap().display())
    }

    let build_opts = BuildOptions {
        path: Some(lc3b_path.clone()),
        out_dir: out_dir.join("pkg").display().to_string(),
        disable_dts: true,
        target: Target::Web,
        ..Default::default()
    };
    let mut build = Build::try_from_opts(build_opts).unwrap();
    build.run().expect("wasm-pack build");

    println!(
        "cargo:rustc-env=LC3B_PKG_JS_PATH={}",
        out_dir.join("pkg").join("lc3b.js").display().to_string()
    );
    println!(
        "cargo:rustc-env=LC3B_PKG_WASM_PATH={}",
        out_dir
            .join("pkg")
            .join("lc3b_bg.wasm")
            .display()
            .to_string()
    );
}

fn list_folders(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut folders = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(folder_name) = path.file_name() {
                if let Some(folder_name_str) = folder_name.to_str() {
                    folders.push(folder_name_str.to_string());
                }
            }
        }
    }

    Ok(folders)
}
