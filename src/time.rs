/// print the time that executing some expression took
/// but only when relevant according to log level.
///
/// The goal of this macro is to avoid doing useless
/// `Instant::now`.
///
/// Arguments:
/// - log level, optional (default is `Debug`)
/// - a category, optional (only if name is set)
/// - a name, optional (stringified expression is used by default)
/// - the expression whose duration we want to log depending on the level
///
/// Examples:
///
/// ```
/// # use log::*;
/// # use cli_log::*;
/// # fn do_stuff(arg: usize) -> Result<usize, String> {
/// #    Ok(arg)
/// # }
/// # fn main() -> Result<(), String> {
/// let result = time!(do_stuff(4));
/// let result = time!(Debug, do_stuff(3))?;
/// let result = time!("World creation", do_stuff(7));
/// let sum = time!(Debug, "summing", 2 + 2);
/// let sum = time!(Debug, "summing", 2 + 2);
/// let mult = time!("operations", "mult 4", 3 * 4);
/// let mult = time!(Info, "operations", "mult 4", 3 * 4);
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! time {
    ($timed: expr $(,)?) => {{
        use cli_log::{*, Level::*};
        if log_enabled!(Debug) {
            let start = std::time::Instant::now();
            match $timed {
                value => {
                    log!(Debug, "{} took {:?}", stringify!($timed), start.elapsed());
                    value
                }
            }
        } else {
            $timed
        }
    }};
    ($level: ident, $timed: expr $(,)?) => {{
        use cli_log::{*, Level::*};
        if log_enabled!($level) {
            let start = std::time::Instant::now();
            match $timed {
                value => {
                    log!($level, "{} took {:?}", stringify!($timed), start.elapsed());
                    value
                }
            }
        } else {
            $timed
        }
    }};
    ($name: expr, $timed: expr $(,)?) => {{
        use cli_log::{*, Level::*};
        if log_enabled!(Debug) {
            let start = std::time::Instant::now();
            match $timed {
                value => {
                    log!(Debug, "{} took {:?}", $name, start.elapsed());
                    value
                }
            }
        } else {
            $timed
        }
    }};
    ($level: ident, $name: expr, $timed: expr $(,)?) => {{
        use cli_log::{*, Level::*};
        if log_enabled!($level) {
            let start = std::time::Instant::now();
            match $timed {
                value => {
                    log!($level, "{} took {:?}", $name, start.elapsed());
                    value
                }
            }
        } else {
            $timed
        }
    }};
    ($cat: expr, $name :expr,  $timed: expr $(,)?) => {{
        use cli_log::{*, Level::*};
        if log_enabled!(Debug) {
            let start = std::time::Instant::now();
            match $timed {
                value => {
                    log!(Debug, "{} on {:?} took {:?}", $cat, $name, start.elapsed());
                    value
                }
            }
        } else {
            $timed
        }
    }};
    ($level: ident, $cat: expr, $name :expr,  $timed: expr $(,)?) => {{
        use cli_log::{*, Level::*};
        if log_enabled!($level) {
            let start = std::time::Instant::now();
            match $timed {
                value => {
                    log!($level, "{} on {:?} took {:?}", $cat, $name, start.elapsed());
                    value
                }
            }
        } else {
            $timed
        }
    }};
}
