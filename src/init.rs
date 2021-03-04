use {
    crate::{
        file_logger::FileLogger,
    },
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
pub fn init(app_name: &str, app_version: &str) {
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
            app_version,
            level
        );
    }
}

/// configure the application log according to env variable
///
/// Example:
///
/// ```
/// #[macro_use] extern crate log;
/// #[macro_use] extern crate cli_log;
/// init_cli_log!();
/// ```
/// You may specify an altername application name instead
/// of your crate name:
///
/// ```
/// #[macro_use] extern crate log;
/// #[macro_use] extern crate cli_log;
/// init_cli_log!("my-app");
/// ```
///
/// The application name will also be used to derive the
/// env variable name giving the log level, for example
/// `MY_APP_LOG=info` for an application named `my-app`.
// The point of this macro is to ensure `env!(CARGO_PKG_NAME)`
// and  `env!(CARGO_PKG_VERSION)` are expanded for the outer
// package, not for cli-log
#[macro_export]
macro_rules! init_cli_log {
    () => {
        cli_log::init(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    };
    ($app_name: expr) => {
        cli_log::init($app_name, env!("CARGO_PKG_VERSION"));
    };
}
