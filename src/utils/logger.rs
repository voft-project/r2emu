use crate::monitor::args::GLOBAL_R2EMU_CONFIG;
use fern;
use log::info;
use std::fs::OpenOptions;

pub fn init_log() -> Result<(), fern::InitError> {
    let log_file = &GLOBAL_R2EMU_CONFIG.get().unwrap().log;
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(log_file);

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{} {}] {}",
                record.level(),
                record.target(),
                record.line().unwrap(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file)?)
        .apply()?;
    info!("Log is written to {}", log_file);
    Ok(())
}
