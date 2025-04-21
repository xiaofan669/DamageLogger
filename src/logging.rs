use std::{fs::OpenOptions, sync::{Mutex, OnceLock}};
use egui_logger::EguiLogger;
use log::{Level, LevelFilter, Metadata, Record};
use slog::{o, Drain, Logger};

static LOGGER: OnceLock<MultiLogger> = OnceLock::new();

pub struct MultiLogger<> {
    sloggers: Vec<Logger>,
    egui_logger: EguiLogger,
}

impl log::Log for MultiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::STATIC_MAX_LEVEL
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.egui_logger.log(record);   
        }
        let fmt_log = if let Some(mod_path) = record.module_path() {
            format!("{} {}", mod_path, record.args())
        }
        else {
            format!("{}", record.args())
        };
        for slogger in &self.sloggers {
            match record.metadata().level() {
                Level::Error => slog::error!(slogger, "{}", fmt_log),
                Level::Warn => slog::warn!(slogger, "{}", fmt_log),
                Level::Info => slog::info!(slogger, "{}", fmt_log),
                Level::Debug => slog::debug!(slogger, "{}", fmt_log),
                Level::Trace => slog::trace!(slogger, "{}", fmt_log),
            }
    
        }
    }

    fn flush(&self) {}
}


impl MultiLogger {
    pub fn init() {
        let log_path = "veritas.log";
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(log_path)
            .unwrap();

        let mut sloggers = Vec::new();
    
        {
            let decorator = slog_term::PlainSyncDecorator::new(file);
            let drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();
            let slog = slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));
            sloggers.push(slog);
        }

        {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();
            let slog = slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));
            sloggers.push(slog);
        }
    
        let egui_logger = egui_logger::builder().build();
        let multi_logger = MultiLogger {
            sloggers,
            egui_logger
        };
        if LOGGER.set(multi_logger).is_err() {
            panic!("Failed to initialize MultiLogger");
        }
        log::set_logger(LOGGER.get().unwrap()).unwrap();
        
        #[cfg(debug_assertions)]
        log::set_max_level(LevelFilter::Debug);
        
        #[cfg(not(debug_assertions))]
        log::set_max_level(LevelFilter::Info);


    }
}


