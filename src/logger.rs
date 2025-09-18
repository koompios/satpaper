use comat::{cformat_args, cprintln};
use log::{Level, Metadata, Record};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Logger {}

impl Logger {
    pub fn init(level: Level) {
        static LOGGER: OnceLock<Logger> = OnceLock::new();
        LOGGER.set(Self {}).unwrap();
        log::set_logger(LOGGER.get().unwrap())
            .map(|()| log::set_max_level(level.to_level_filter()))
            .unwrap();
    }
}

impl log::Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let level = match record.level() {
            Level::Error => cformat_args!("{bold_red}err{reset}"),
            Level::Warn => cformat_args!("{bold_yellow}wrn{reset}"),
            Level::Trace => cformat_args!("{magenta}trc{reset}"),
            Level::Debug => cformat_args!("{green}dbg{reset}"),
            Level::Info => cformat_args!("{blue}inf{reset}"),
        };
        cprintln!(
            "[{} {:bold_blue}:{:blue}] {}",
            level,
            record.file().unwrap_or("<source>"),
            record.line().unwrap_or(0),
            record.args(),
        );
    }

    fn flush(&self) {}
}
