use {
    log::{
        LevelFilter,
        Log,
        Metadata,
        Record,
    },
    std::{
        fs::File,
        io::Write,
        sync::Mutex,
    },
};

static TIME_FORMAT: &str = "%T%.3f";

pub(crate) struct FileLogger {
    pub file: Mutex<File>,
    pub level: LevelFilter,
}

impl Log for FileLogger {
    fn enabled(
        &self,
        metadata: &Metadata<'_>,
    ) -> bool {
        metadata.level() <= self.level
    }

    fn log(
        &self,
        record: &Record<'_>,
    ) {
        if self.enabled(record.metadata()) {
            let mut w = self.file.lock().unwrap();
            let _ = writeln!(
                w,
                "{} [{}] {}: {}",
                chrono::Local::now().format(TIME_FORMAT),
                record.level(),
                record.target(),
                record.args(),
            ); // we ignore errors here
        }
    }

    fn flush(&self) {
        let _ = self.file.lock().unwrap().flush();
    }
}
