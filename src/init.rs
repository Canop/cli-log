use {
    crate::file_logger::FileLogger,
    log::LevelFilter,
    std::{
        env,
        fs::File,
        str::FromStr,
        sync::Mutex,
    },
};

/// Configure the application log according to env variable, without failing
/// in case of io error.
///
/// If the log file cannot be created, the error is printed to stderr, then
/// the application proceeds without logging.
pub fn init(
    app_name: &str,
    app_version: &str,
) {
    if let Err(e) = try_init(app_name, app_version) {
        eprintln!("Failed to initialize log: {}", e);
    }
}

/// Configure the application log according to env variable.
///
/// If an io error occurs, it is returned, and all logging is disabled
/// (but there won't be any panic or error on log calls).
///
/// The caller can decide to print the error and not, to continue or not.
pub fn try_init(
    app_name: &str,
    app_version: &str,
) -> std::io::Result<()> {
    let env_var_name = format!("{}_LOG", app_name.to_ascii_uppercase().replace('-', "_"),);
    let level = env::var(env_var_name).unwrap_or_else(|_| "off".to_string());
    if level == "off" {
        return Ok(());
    }
    if let Ok(level) = LevelFilter::from_str(&level) {
        let log_file_name = format!("{}.log", app_name);
        let file = File::create(log_file_name)?;
        log::set_max_level(level);
        let logger = FileLogger {
            file: Mutex::new(file),
            level,
        };
        log::set_boxed_logger(Box::new(logger)).unwrap();
        log::info!(
            "Starting {} v{} with log level {}",
            app_name,
            app_version,
            level
        );
    }
    Ok(())
}

/// Configure the application log according to env variable, without failing
/// in case of io error.
///
/// If the log file cannot be created, the error is printed to stderr, then
/// the application proceeds without logging.
///
/// Example:
///
/// ```
/// cli_log::init_cli_log!();
/// ```
/// You may specify an altername application name instead
/// of your crate name:
///
/// ```
/// cli_log::init_cli_log!("my-app");
/// ```
///
/// The application name will also be used to derive the
/// env variable name giving the log level, for example
/// `MY_APP_LOG=info` for an application named `my-app`.
///
/// The point of using this macro instead of the init function is to ensure
/// `env!(CARGO_PKG_NAME)` and  `env!(CARGO_PKG_VERSION)` are expanded for
/// the outer package, not for cli-log
#[macro_export]
macro_rules! init_cli_log {
    () => {
        cli_log::init(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    };
    ($app_name: expr) => {
        cli_log::init($app_name, env!("CARGO_PKG_VERSION"));
    };
}

/// Configure the application log according to env variable.
///
/// If an io error occurs, it is returned, and all logging is disabled
/// (but there won't be any panic or error on log calls).
///
/// The caller can decide to print the error and not, to continue or not.
///
/// Example:
///
/// ```
/// cli_log::try_init_cli_log!().expect("Failed to initialize log");
/// ```
/// You may specify an altername application name instead
/// of your crate name:
///
/// ```
/// if Err(e) = cli_log::try_init_cli_log!("my-app") {
///      eprintln!("Running without log because of error: {}", e);
/// }
/// ```
///
/// The application name will also be used to derive the
/// env variable name giving the log level, for example
/// `MY_APP_LOG=info` for an application named `my-app`.
///
/// The point of using this macro instead of the init function is to ensure
/// `env!(CARGO_PKG_NAME)` and  `env!(CARGO_PKG_VERSION)` are expanded for
/// the outer package, not for cli-log
#[macro_export]
macro_rules! try_init_cli_log {
    () => {
        cli_log::try_init(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    };
    ($app_name: expr) => {
        cli_log::try_init($app_name, env!("CARGO_PKG_VERSION"));
    };
}
