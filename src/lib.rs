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
//! cli_log::init("my-app");
//! ```
//!
//! Here's a complete application using cli-log (it can be found in examples):
//!
//! ```
//! #[macro_use] extern crate log;
//! #[macro_use] extern crate cli_log;
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
//!     cli_log::init("small-app");
//!     let mut app_data = AppData { count: 35 };
//!     time!(Debug, app_data.compute());
//!     info!("count is {}", app_data.count);
//!     debug!("data: {:#?}", &app_data);
//!     warn!("this application does nothing");
//!     info!("bye");
//! }
//!
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
//! 13:39:53.511 [INFO] cli_log: Starting small-app v0.1.0 with log level DEBUG
//! 13:39:53.511 [INFO] small_app: count is 42
//! 13:39:53.511 [DEBUG] small_app: data: AppData {
//!     count: 42,
//! }
//! 13:39:53.511 [WARN] small_app: this application does nothing
//! 13:39:53.511 [INFO] small_app: bye
//! ```
//!

#[macro_use] extern crate log;

mod file_logger;
mod time;

use {
    file_logger::FileLogger,
    log::{
        LevelFilter,
    },
    std::{
        env,
        fs::File,
        str::FromStr,
        sync::Mutex,
    },
};


/// configure the application log according to env variable.
pub fn init(app_name: &str) {
    let env_var_name = format!(
        "{}_LOG",
        app_name.to_ascii_uppercase().replace('-', "_"),
    );
    let level = env::var(&env_var_name).unwrap_or_else(|_| "off".to_string());
    if level == "off" {
        return;
    }
    if let Ok(level) = LevelFilter::from_str(&level) {
        let log_file_name = format!("{}.log", app_name);
        let file = File::create(&log_file_name)
            .expect("Log file can't be created");
        log::set_max_level(level);
        let logger = FileLogger {
            file: Mutex::new(file),
            level,
        };
        log::set_boxed_logger(Box::new(logger)).unwrap();
        info!(
            "Starting {} v{} with log level {}",
            app_name,
            env!("CARGO_PKG_VERSION"),
            level
        );
    }
}

