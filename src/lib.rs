//! The boilerplate to have some file logging with a level given by an environment variable,
//! and a facility to log execution durations according to the relevant log level.
//!
//! It's especially convenient for terminal applications
//! because you don't want to mix log with stdout or stderr.
//!
//! The use of an env variable makes it possible to distribute
//! the application and have users generate some logs without
//! recompilation or configuration.
//!
//! The names of the log file and the env variable are
//! computed from the name of the application.
//!
//! So log initialization is just
//!
//! ```
//! use cli_log::*; // also import logging macros
//! init_cli_log!();
//! ```
//!
//! If you prefer not having to declare cli_log import for
//! all the log and cli-log logging macros, you may use the
//! old `#[macro_use]` import in your main.rs file:
//!
//! ```
//! #[macro_use] extern crate cli_log;
//! init_cli_log!();
//! ```
//!
//! With the `"mem"` feature (enabled by default), when the OS is compatible
//! (unix like), you may dump the current and peak memory usage with
//! the `log_mem` function.
//!
//!
//! Here's a complete application using cli-log (it can be found in examples):
//!
//! ```
//! use cli_log::*;
//!
//! #[derive(Debug)]
//! struct AppData {
//!     count: usize,
//! }
//! impl AppData {
//!     fn compute(&mut self) {
//!         self.count += 7;
//!     }
//! }
//!
//! fn main() {
//!     init_cli_log!();
//!     let mut app_data = AppData { count: 35 };
//!     time!(Debug, app_data.compute());
//!     info!("count is {}", app_data.count);
//!     debug!("data: {:#?}", &app_data);
//!     warn!("this application does nothing");
//!     log_mem(Level::Info);
//!     info!("bye");
//! }
//! ```
//!
//! If you don't set any `SMALL_APP_LOG` env variable, there won't be any log.
//!
//! A convenient way to set the env variable is to launch the app as
//!
//! ```cli
//! SMALL_APP_LOG=debug small_app
//! ```
//!
//! or, during development,
//!
//! ```cli
//! SMALL_APP_LOG=debug cargo run
//! ```
//!
//! This creates a `small_app.log` file containing information like the level,
//! app version, and of course the log operations you did with time precise to
//! the ms and the logging module (target):
//!
//! ```log
//! 21:03:24.081 [INFO] cli_log::init: Starting small-app v1.0.1 with log level DEBUG
//! 21:03:24.081 [DEBUG] small_app: app_data.compute() took 312ns
//! 21:03:24.081 [INFO] small_app: count is 42
//! 21:03:24.081 [DEBUG] small_app: data: AppData {
//!     count: 42,
//! }
//! 21:03:24.081 [WARN] small_app: this application does nothing
//! 21:03:24.081 [INFO] cli_log::mem: Physical mem usage: current=938K, peak=3.3M
//! 21:03:24.082 [INFO] small_app: bye
//! ```

mod file_logger;
mod init;
mod time;

pub use {
    init::*,
    log::*,
};

#[cfg(feature = "mem")]
mod mem;
#[cfg(feature = "mem")]
pub use mem::log_mem;
