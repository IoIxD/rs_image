use cbindgen::{Config, ExportConfig, Language};
use std::env;
use std::path::{Path, PathBuf};

fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let output_file = workspace_dir()
        .join("include")
        .join("rs_image.h")
        .display()
        .to_string();

    let mut config = Config::default();
    config.include_guard = Some("__INTERNAL_IMAGE_LOAD_H".to_owned());
    config.language = Language::C;
    config.namespace = None;
    config.export = ExportConfig::default();

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(&output_file);
}
