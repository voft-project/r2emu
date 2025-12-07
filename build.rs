use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

const CONFIG_RE_PATTERN: &str = r"^CONFIG_([A-Za-z0-9_]+)=y";

fn main() -> Result<()> {
    let config_name: &str = ".config";
    let config_file = Path::new(config_name);
    if config_file.exists() {
        println!("{config_name} exist");
    } else {
        println!("{config_name} not exists");
        return Ok(());
    }

    // Parse .config
    let re = Regex::new(CONFIG_RE_PATTERN).expect("Failed to compile regex pattern.");

    let file = File::open(config_file).unwrap();
    let reader = BufReader::new(file);

    for line_res in reader.lines() {
        let line = line_res?;
        if let Some(captures) = re.captures(&line) {
            let key = captures.get(1).map_or("", |m| m.as_str());
            let cfg_flag = key.to_lowercase();
            // 声明cfg
            println!("cargo::rustc-check-cfg=cfg({})", cfg_flag);
            // 启用cfg
            println!("cargo:rustc-cfg={}", cfg_flag);
        }
    }

    Ok(())
}
