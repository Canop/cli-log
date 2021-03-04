[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/cli-log.svg
[l1]: https://crates.io/crates/cli-log

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/cli-log/badge.svg
[l3]: https://docs.rs/cli-log/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3

# cli-log

The boilerplate to have some file logging with a level given by an environment variable,
and a facility to log execution durations according to the relevant log level.

It's especially convenient for terminal applications
because you don't want to mix log with stdout or stderr.

The use of an env variable makes it possible to distribute
the application and have users generate some logs without
recompilation or configuration.

The names of the log file and the env variable are
computed from the name of the application.

So log initialization is just

```
#[macro_use] extern crate log;
#[macro_use] extern crate cli_log;
init_cli_log!();
```

With the `"mem"` feature (enabled by default), when the OS is compatible
(unix like), you may dump the current and peak memory usage with
the `log_mem` function.


Here's a complete application using cli-log (it can be found in examples):

```
#[macro_use] extern crate log;
#[macro_use] extern crate cli_log;

#[derive(Debug)]
struct AppData {
    count: usize,
}
impl AppData {
    fn compute(&mut self) {
        self.count += 7;
    }
}

fn main() {
    init_cli_log!();
    let mut app_data = AppData { count: 35 };
    time!(Debug, app_data.compute());
    info!("count is {}", app_data.count);
    debug!("data: {:#?}", &app_data);
    warn!("this application does nothing");
    cli_log::log_mem(log::Level::Info);
    info!("bye");
}

```

If you don't set any `SMALL_APP_LOG` env variable, there won't be any log.

A convenient way to set the env variable is to launch the app as

```cli
SMALL_APP_LOG=debug small_app
```

or, during development,

```cli
SMALL_APP_LOG=debug cargo run
```

This creates a `small_app.log` file containing information like the level,
app version, and of course the log operations you did with time precise to
the ms and the logging module (target):

```log
21:03:24.081 [INFO] cli_log::init: Starting small-app v1.0.1 with log level DEBUG
21:03:24.081 [DEBUG] small_app: app_data.compute() took 312ns
21:03:24.081 [INFO] small_app: count is 42
21:03:24.081 [DEBUG] small_app: data: AppData {
    count: 42,
}
21:03:24.081 [WARN] small_app: this application does nothing
21:03:24.081 [INFO] cli_log::mem: Physical mem usage: current=938K, peak=3.3M
21:03:24.082 [INFO] small_app: bye
```
