use std::env;
use crate::utils::const_func::parse_hex;

pub mod arch;
pub mod instruction;
pub mod img;

pub const RESET_VECTOR: u32 = parse_hex(env!("CONFIG_MBASE"));