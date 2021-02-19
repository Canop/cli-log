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
    cli_log::init("small-app");
    let mut app_data = AppData { count: 35 };
    time!(Debug, app_data.compute());
    info!("count is {}", app_data.count);
    debug!("data: {:#?}", &app_data);
    warn!("this application does nothing");
    info!("bye");
}
