//! # Really...? Another Logging Library?
//!
//! **Yes!**
//!
//! ## Description
//!
//! _rall_ is an incredibly simple and intuitive logger, consider this crate a _failure_ if you
//! can't get setup within **30 seconds!**
//!
//! ## Feature Set
//!
//! - [x] Logging Levels
//! - [x] Coloured Output
//! - [ ] Options for Datetime, Current Function, Line Number, Custom Colours, etc.
//! - [ ] Custom Formatting
//! - [ ] File support
//!
//! And much more to come... soon™!
//!
//! ## Quick Start
//!
//! ```rust
//! use rall::SimpleLogger;
//!
//! // Create Default SimpleLogger
//! let mut simple_logger = SimpleLogger::default();
//!
//! // Log Out To Standard Output
//! logger.log(Level::TRACE, "My Best Friend Hazel :D");
//! logger.log(Level::DEBUG, "My Best Friend Hazel :D");
//! logger.log(Level::INFO, "My Best Friend Hazel :D");
//! logger.log(Level::WARN, "My Best Friend Hazel :D");
//! logger.log(Level::ERROR, "My Best Friend Hazel :D");
//! logger.log(Level::CRITICAL, "My Best Friend Hazel :D");
//! ```
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("windows_logs", "images/windows_logs.png")))]
//!
//! ### Windows Output
//!
//! ![Example Logs][windows_logs]
//!
//! ### Unix Output
//!
//!
//!
//! ### Author Notes
//!
//! I'm still incredibly early in my Rust journey and so I wanted to get comfortable and try to pick
//! my own brain about exposing different API's in a Rust crate. I hope to expose an intuitive and
//! easy to understand API design that users can instantly get started with.

use std::fmt::{Display, Formatter};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// TODO
pub enum Level {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    CRITICAL,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::TRACE => write!(f, "[TRACE]"),
            Level::DEBUG => write!(f, "[DEBUG]"),
            Level::INFO => write!(f, "[INFO]"),
            Level::WARN => write!(f, "[WARN]"),
            Level::ERROR => write!(f, "[ERROR]"),
            Level::CRITICAL => write!(f, "[FATAL]"),
        }
    }
}

/// TODO
pub struct SimpleLogger {
    standard_stream: StandardStream,
}

impl Default for SimpleLogger {
    fn default() -> Self {
        Self {
            standard_stream: StandardStream::stdout(ColorChoice::Always),
        }
    }
}

/// TODO
impl SimpleLogger {
    /// TODO
    pub fn new(standard_stream: StandardStream) -> Self {
        Self { standard_stream }
    }

    /// TODO
    pub fn log(&mut self, level: Level, str: &str) {
        self.set_colour(&level);
        writeln!(&mut self.standard_stream, "{} {}", level, str).unwrap();
        self.standard_stream.reset().unwrap();
    }

    /// TODO
    fn set_colour(&mut self, logging_level: &Level) {
        match logging_level {
            Level::TRACE => self
                .standard_stream
                .set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true))
                .unwrap(),
            Level::DEBUG => self
                .standard_stream
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))
                .unwrap(),
            Level::INFO => self
                .standard_stream
                .set_color(ColorSpec::new().set_fg(None))
                .unwrap(),
            Level::WARN => self
                .standard_stream
                .set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::Yellow))
                        .set_dimmed(true),
                )
                .unwrap(),
            Level::ERROR => self
                .standard_stream
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .unwrap(),
            Level::CRITICAL => self
                .standard_stream
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_intense(true))
                .unwrap(),
        }
    }
}