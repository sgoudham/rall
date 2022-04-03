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
//! - [x] Datetime & Coloured Output
//! - [ ] Options for Datetime, Current Function, Line Number, Custom Colours, etc.
//! - [ ] Custom Formatting
//! - [ ] File support
//!
//! And much more to come... soon™!
//!
//! ## Quick Start
//!
//! For the fastest setup possible, declarative macros are exposed that have a predefined format.
//! This is to allow hassle-free and painless setup that will let you log instantly!
//!
//! ```rust
//! use rall::{debug, error, fatal, info, trace, warn};
//!
//! // Log Out To Standard Output
//! trace!("My Best Friend Hazel :D");
//! debug!("My Best Friend Hazel :D");
//! info!("My Best Friend Hazel :D");
//! warn!("My Best Friend Hazel :D");
//! error!("My Best Friend Hazel :D");
//! fatal!("My Best Friend Hazel :D");
//! ```
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc =::embed_doc_image::embed_image ! ("windows_logs", "images/windows_logs.png"),
doc =::embed_doc_image::embed_image ! ("unix_logs", "images/unix_logs.png")))]
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

/// Represents all the possible logging levels:
///
/// **TRACE**,
/// **DEBUG**,
/// **INFO**,
/// **WARN**,
/// **ERROR**,
/// **FATAL**
pub enum Level {
    /// # Usage
    ///
    /// For fine-grained information, only within rare cases where full visibility of what is
    /// happening in your application is needed.
    ///
    /// # Colour
    ///
    /// Blue
    TRACE,

    /// # Usage
    ///
    /// Less granular when compared to [`TRACE`](Level::TRACE) but still more than what is needed
    /// for normal use. This should be used for diagnosing issues and/or troubleshooting.
    ///
    /// # Colour
    ///
    /// Green
    DEBUG,

    /// # Usage
    ///
    /// Standard log level indicating that something has happened, all logs using [`INFO`](Level::INFO)
    /// should be _purely informational_ and not require any further investigation.
    ///
    /// # Colour
    ///
    /// White
    INFO,

    /// # Usage
    ///
    /// Indicates that something _unexpected_ has happened within the program. This could represent
    /// many things such as a problem or a simple disturbance. This should be used when something
    /// unexpected has happened BUT the code can still continue to work.
    ///
    /// # Colour
    ///
    /// Yellow
    WARN,

    /// # Usage
    ///
    /// Indicates that the program has hit an issue that is preventing one or more functionalities
    /// from properly functioning. This should be used when the application is currently displaying
    /// incorrect behaviour that _needs_ to get fixed.
    ///
    /// # Colour
    ///
    /// Dark Red
    ERROR,

    /// # Usage
    ///
    /// Indicates that the program has entered a state in which it has lost _critical business
    /// functionality_ and cannot be used in production anymore. This should be used when the
    /// program is in **URGENT** need of attention and absolutely should not be in a live environment.
    ///
    /// # Colour
    ///
    /// Red
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
            .set_color(
                termcolor::ColorSpec::new()
                    .set_fg(Some(termcolor::Color::Blue))
                    .set_bold(true),
            )
            .unwrap();
        writeln!(&mut stream, "[{} {}] {}", now, rall::Level::TRACE, $str).unwrap();
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
            .set_color(
                termcolor::ColorSpec::new()
                    .set_fg(Some(termcolor::Color::Green))
                    .set_bold(true),
            )
            .unwrap();
        writeln!(&mut stream, "[{} {}] {}", now, rall::Level::DEBUG, $str).unwrap();
        stream.reset().unwrap();
    }};
}

#[macro_export]
macro_rules! info {
    ($str:expr) => {
        let now = chrono::Utc::now().format("%Y-%M-%dT%H:%M:%S%z");
        println!("{}", format!("[{} {}] {}", now, rall::Level::INFO, $str));
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
            .set_color(
                termcolor::ColorSpec::new()
                    .set_fg(Some(termcolor::Color::Yellow))
                    .set_bold(true),
            )
            .unwrap();
        writeln!(&mut stream, "[{} {}] {}", now, rall::Level::WARN, $str).unwrap();
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
            .set_color(
                termcolor::ColorSpec::new()
                    .set_fg(Some(termcolor::Color::Red))
                    .set_intense(true),
            )
            .unwrap();
        writeln!(&mut stream, "[{} {}] {}", now, rall::Level::ERROR, $str).unwrap();
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
            .set_color(
                termcolor::ColorSpec::new()
                    .set_fg(Some(termcolor::Color::Red))
                    .set_bold(true),
            )
            .unwrap();
        writeln!(&mut stream, "[{} {}] {}", now, rall::Level::FATAL, $str).unwrap();
        stream.reset().unwrap();
    }};
}