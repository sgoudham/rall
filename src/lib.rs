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
//! use rall::{SimpleLogger, Level};
//!
//! // Create Default SimpleLogger
//! let mut logger = SimpleLogger::default();
//!
//! // Log Out To Standard Output
//! logger.log(Level::TRACE, "My Best Friend Hazel :D");
//! logger.log(Level::DEBUG, "My Best Friend Hazel :D");
//! logger.log(Level::INFO, "My Best Friend Hazel :D");
//! logger.log(Level::WARN, "My Best Friend Hazel :D");
//! logger.log(Level::ERROR, "My Best Friend Hazel :D");
//! logger.log(Level::FATAL, "My Best Friend Hazel :D");
//! ```
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("windows_logs", "images/windows_logs.png"),
doc = ::embed_doc_image::embed_image!("unix_logs", "images/unix_logs.png")))]
//!
//! ### Windows Output
//!
//! ![Windows Logs][windows_logs]
//!
//! ### Unix Output
//!
//! ![Unix Logs][unix_logs]
//!
//! ### Author Notes
//!
//! I'm still incredibly early in my Rust journey and so I wanted to get comfortable and try to pick
//! my own brain about exposing different APIs in a Rust crate. I hope to expose an intuitive and
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
    FATAL,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::TRACE => write!(f, "TRACE"),
            Level::DEBUG => write!(f, "DEBUG"),
            Level::INFO => write!(f, "INFO"),
            Level::WARN => write!(f, "WARN"),
            Level::ERROR => write!(f, "ERROR"),
            Level::FATAL => write!(f, "FATAL"),
        }
    }
}

#[macro_export]
macro_rules! trace {
    ($str:expr) => {{
        use std::io::Write;
        use termcolor::WriteColor;

        let now = chrono::Utc::now().format("%Y-%M-%dT%H:%M:%S%z");
        let mut stream = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
        stream
            .set_color(termcolor::ColorSpec::new()
            .set_fg(Some(termcolor::Color::Blue))
            .set_bold(true))
            .unwrap();
        writeln!(&mut stream, "[{} {}] {}", now, Level::TRACE, $str).unwrap();
        stream.reset().unwrap();
    }};
}

#[macro_export]
macro_rules! debug {
    ($str:expr) => {{
        use std::io::Write;
        use termcolor::WriteColor;

        let now = chrono::Utc::now().format("%Y-%M-%dT%H:%M:%S%z");
        let mut stream = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
        stream
            .set_color(termcolor::ColorSpec::new()
            .set_fg(Some(termcolor::Color::Green))
            .set_bold(true))
            .unwrap();
        writeln!(&mut stream, "[{} {}] {}", now, Level::DEBUG, $str).unwrap();
        stream.reset().unwrap();
    }};
}

#[macro_export]
macro_rules! info {
    ($str:expr) => {
        let now = chrono::Utc::now().format("%Y-%M-%dT%H:%M:%S%z");
        println!("{}", format!("[{} {}] {}", now, Level::INFO, $str));
    };
}

#[macro_export]
macro_rules! warn {
    ($str:expr) => {{
        use std::io::Write;
        use termcolor::WriteColor;

        let now = chrono::Utc::now().format("%Y-%M-%dT%H:%M:%S%z");
        let mut stream = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
        stream
            .set_color(termcolor::ColorSpec::new()
            .set_fg(Some(termcolor::Color::Yellow))
            .set_bold(true))
            .unwrap();
        writeln!(&mut stream, "[{} {}] {}", now, Level::WARN, $str).unwrap();
        stream.reset().unwrap();
    }};
}

#[macro_export]
macro_rules! error {
    ($str:expr) => {{
        use std::io::Write;
        use termcolor::WriteColor;

        let now = chrono::Utc::now().format("%Y-%M-%dT%H:%M:%S%z");
        let mut stream = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
        stream
            .set_color(termcolor::ColorSpec::new()
            .set_fg(Some(termcolor::Color::Red))
            .set_intense(true))
            .unwrap();
        writeln!(&mut stream, "[{} {}] {}", now, Level::ERROR, $str).unwrap();
        stream.reset().unwrap();
    }};
}

#[macro_export]
macro_rules! fatal {
    ($str:expr) => {{
        use std::io::Write;
        use termcolor::WriteColor;

        let now = chrono::Utc::now().format("%Y-%M-%dT%H:%M:%S%z");
        let mut stream = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
        stream
            .set_color(termcolor::ColorSpec::new()
            .set_fg(Some(termcolor::Color::Red))
            .set_bold(true))
            .unwrap();
        writeln!(&mut stream, "[{} {}] {}", now, Level::FATAL, $str).unwrap();
        stream.reset().unwrap();
    }};
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
            Level::INFO => {}
            Level::WARN => self
                .standard_stream
                .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
                .unwrap(),
            Level::ERROR => self
                .standard_stream
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_intense(true))
                .unwrap(),
            Level::FATAL => self
                .standard_stream
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))
                .unwrap(),
        }
    }
}