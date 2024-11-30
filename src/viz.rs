use std::sync::Mutex;
pub mod graphviz;

pub fn viz_file_name(extension: &str) -> String {
    viz_file_stem() + "." + extension
}

pub fn viz_file_stem() -> String {
    static FILE_COUNTER: Mutex<u32> = Mutex::new(0);
    let counter = {
        let mut counter = FILE_COUNTER.lock().unwrap();
        *counter += 1;
        *counter
    };
    format!("viz/{}_{counter:04}", exe_name())
}

/// The name of the executable that is currently running. Started from
/// [nannou::app::App]'s method of the same name. Replaces non-Unicode
/// sequences with �. If there isn't a known executable, or it doesn't
/// have a file stem, return `"aoc-unknown"`.
fn exe_name() -> String {
    std::env::current_exe()
        .map(|p| p.file_stem().map(|s| s.to_string_lossy().to_string()))
        .unwrap_or(None)
        .unwrap_or_else(|| "aoc-unknown".to_string())
}
