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
