use serde::Deserialize;
use std::fs::read_to_string;
use std::io::Write;
use std::path::PathBuf;

use std::process;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    loader: String,
    option: String,
    shellcode: String,
}

fn load_config() -> Config {
    let cwd = std::env::current_dir().unwrap();
    let config_path = cwd.join("config.toml");
    if let Ok(c) = read_to_string(config_path) {
        let config: Config = toml::from_str(&c).unwrap();
        config
    } else {
        panic!("Failed to read config.toml")
    }
}

pub fn init() -> (PathBuf, String) {
    let cwd = std::env::current_dir().unwrap();
    let config = load_config();

    let loader = config.loader.clone();
    let loader_path = cwd.join("loaders").join(&loader);
    if !loader_path.exists() {
        eprintln!("Loader {} not found in loaders directory", loader);
        std::process::exit(1);
    }

    let sc_path = loader_path.join("sc.txt");
    // create new file of sc_path
    let mut file = std::fs::File::create(&sc_path).unwrap();
    // write some data to the file
    file.write_all(config.shellcode.as_bytes()).unwrap();
    // close the file
    file.flush().unwrap();

    (loader_path, config.loader)
}

pub fn work(d: (PathBuf, String)) {
    let (wd, loader) = d;
    let previous_wd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&wd).unwrap();

    println!("[*] 正在构建子项目");
    process::Command::new("cmd.exe")
        .args(&[
            "/C",
            "cargo.exe",
            "run",
            "--package",
            &loader,
            "--bin",
            &loader,
        ])
        .output()
        .unwrap();

    process::Command::new("cmd.exe")
        .args(&[
            "/C",
            "cargo.exe",
            "build",
            "--package",
            &loader,
            "--bin",
            "loader",
            "--release",
        ])
        .output()
        .unwrap();

    std::env::set_current_dir(previous_wd).unwrap();

    let target_path = wd.join("target").join("release").join("loader.exe");
    std::fs::copy(&target_path, "loader.exe").unwrap();
    println!("[+] loader.exe 已在当前目录生成");
}
