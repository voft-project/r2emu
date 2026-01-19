use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

fn main() -> Result<()> {
    let config_name: &str = "include/config/auto.conf";
    let config_file: &Path = Path::new(config_name);
    if config_file.exists() {
        println!("{config_name} exist");
    } else {
        println!("{config_name} not exists");
        return Ok(());
    }
    println!("cargo:rerun-if-changed={}", config_name);

    let file = File::open(config_file).unwrap();
    let reader = BufReader::new(file);
    for line_res in reader.lines() {
        let line = line_res?;
        if let Some((key, value)) = line.split_once('=') {
            let clean_value = value.trim().trim_matches('"');
            let clean_key = key.trim();
            if clean_value == "y" {
                println!("cargo:rustc-check-cfg=cfg({})", clean_key);
                println!("cargo::rustc-cfg={}", clean_key);
            } else {
                println!("cargo:rustc-check-cfg=cfg({}, values(\"{}\"))", clean_key, clean_value);
                println!("cargo::rustc-cfg={}=\"{}\"", clean_key, clean_value);
                // 注入环境变量
                println!("cargo::rustc-env={}={}", clean_key, clean_value);
            }
        }
    }

    Ok(())
}
