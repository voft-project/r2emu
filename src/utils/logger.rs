use crate::monitor::args::GLOBAL_R2EMU_CONFIG;
use fern::{self, colors::ColoredLevelConfig, colors::Color};
use log::info;
use std::fs::OpenOptions;

pub fn init_log() -> Result<(), fern::InitError> {
    let config = GLOBAL_R2EMU_CONFIG.get().expect("GLOBAL_R2EMU_CONFIG not initialized");
    let log_file = &config.log;

    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(log_file);

    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{} {}] {}",
                // 设置颜色
                colors.color(record.level()),
                record.target(),
                record.line().unwrap(),
                message,
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file)?)
        .apply()?;

    info!("Log is written to {}", log_file);
    Ok(())
}
