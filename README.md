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

The boilerplate to have some file logging with a level given by an environment variable.

It's convenient for terminal applications because you don't want to mix log with stdout or stderr.

And the use of an env variable makes it possible to distribute
the application and have users generate some logs without
recompilation or configuration.

The names of the log file and the env variable are
computed from the name of the application.

So log initialization is just

```
cli_log::init("my-app");
```


### Import

In Cargo.toml:

    log = "0.4"
    cli-log = "0.1"


### Usage

Here's a complete application using cli-log (it can be found in examples):

```
#[macro_use]
extern crate log;

#[derive(Debug)]
struct AppData {
    count: usize,
}

fn main() {
    cli_log::init("small-app");
    let app_data = AppData { count: 42 };
    info!("count is {}", app_data.count);
    debug!("data: {:#?}", &app_data);
    warn!("this application does nothing");
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

```
13:39:53.511 [INFO] cli_log - Starting small-app v0.1.0 with log level DEBUG
13:39:53.511 [INFO] small_app - count is 42
13:39:53.511 [DEBUG] small_app - data: AppData {
    count: 42,
}
13:39:53.511 [WARN] small_app - this application does nothing
13:39:53.511 [INFO] small_app - bye
```

This log file can typically be followed with `tail -f small_app.log`.
