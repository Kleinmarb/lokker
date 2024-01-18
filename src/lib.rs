//! Lokker is a simple logger which focuses on simplicity (like actually).
//!
//! ## Example
//!
//! ```
//! use lokker::Logger;
//!
//! Logger::init().unwrap();
//! ```
//!
//! ## This Crate
//!
//! The `lokker` crate is the default logger for the [byzantium libDBMS](https://docs.rs/byzantium/*/byzantium/).
//! It is this simple since the default for byzantium should be easy-to-use.
//! You can still use your own logger that is compatible with the [log crate](https://docs.rs/log/*/log/).

pub(crate) mod time;

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

/// Implements [`Log`] to be compatible
/// This logger is intentionally minimalistic, offering only essential features, it wont offer configuration
///
/// # Examples
///
/// ## Initialize logger
///
/// ```
/// use lokker::Logger;
///
/// Logger::init().unwrap();
/// ```
pub struct Logger {
    /// Logging level
    level: LevelFilter,
}

impl Logger {

    /// Initialize with a given log level
    ///
    /// This method (or `init`) has to be called in order the logger to take effect
    ///
    /// # Example
    ///
    /// ```
    /// use lokker::Logger;
    /// use log::LevelFilter;
    ///
    /// Logger::init_with_level(LevelFilter::Debug).unwrap();
    /// ```
    pub fn init_with_level(level: LevelFilter) -> Result<(), SetLoggerError> {
        let logger =  Logger { level };

        log::set_max_level(logger.level);
        log::set_boxed_logger(Box::new(logger))

    }

    /// Initializes the actual logger
    ///
    /// This method (or `init_with_level`) has to be called in order the logger to take effect
    ///
    /// Defaults the Logger with log level set to `Level::Trace`
    ///
    /// # Example
    ///
    /// ```
    /// use lokker::Logger;
    ///
    /// Logger::init().unwrap();
    /// ```
    pub fn init() -> Result<(), SetLoggerError> {
        let logger =  Logger { level: LevelFilter::Trace };

        log::set_max_level(logger.level);
        log::set_boxed_logger(Box::new(logger))
    }
}

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let (hour, minute, second) = time::utc_now();

        let formatted_time = format!(
            "{:02}:{:02}:{:02}",
            hour,
            minute,
            second,
        );

        let message = format!("{} [{}] {}", formatted_time, record.level(), record.args());

        println!("{}", message);
    }

    fn flush(&self) {}
}
