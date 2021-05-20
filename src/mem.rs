
use log::*;

/// log the current and peak physical memory used by
/// the current process, if the given log level is
/// reached
///
/// This function is only available when the feature
/// "mem" is enabled and when the OS supports it
/// (unix-like systems).
pub fn log_mem(level: Level) {
    if log_enabled!(level) {
        if let Ok(mem) = proc_status::mem_usage() {
            log!(
                level,
                "Physical mem usage: current={}, peak={}",
                file_size::fit_4(mem.current as u64),
                file_size::fit_4(mem.peak as u64),
            );
        }
    }
}
