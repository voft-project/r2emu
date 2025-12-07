use clap::Parser;
use log::info;
use std::sync::OnceLock;

pub static GLOBAL_R2EMU_CONFIG: OnceLock<CommandArgs> = OnceLock::new();

#[derive(Parser, Debug)]
#[command(long_about = None)]
pub struct CommandArgs {
    /// Specify guest image
    #[arg(index = 1)]
    pub image: Option<String>,

    /// Set Batch Mode
    #[arg(short, long)]
    pub batch: bool,

    /// Specify log file
    #[arg(short, long, default_value_t = String::from("r2emu-log"))]
    pub log: String,

    /// difftest ref file
    #[arg(short, long, default_value_t = String::new())]
    pub diff: String,

    /// difftest port
    #[arg(short, long, default_value_t = 1234)]
    pub port: u32,
}

pub fn parse_args() {
    let ca = CommandArgs::parse();
    GLOBAL_R2EMU_CONFIG.set(ca).expect("arg not initialized");
}

#[allow(unused)]
pub fn print_args() {
    let ca = CommandArgs::parse();
    println!("{:?}", ca);
}
