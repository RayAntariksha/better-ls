//Libraries
use std::path::Path;
use std::fs;

pub fn get_dir_size(path: &str) -> String {
    let size = calculate_size(Path::new(path));
    format_size(size)
}

fn calculate_size(path: &Path) -> u64 {
    if path.is_file() {
        return fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    }

    fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| calculate_size(&e.path()))
                .sum()
        })
        .unwrap_or(0)
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let i = (bytes as f64).log(1024.0).floor() as usize;
    let index = i.min(UNITS.len() - 1);
    let value = bytes as f64 / 1024_f64.powi(index as i32);
    if index == 0 {
        format!("{} {}", bytes, UNITS[index])
    } else {
        format!("{:.2} {}", value, UNITS[index])
    }
}
