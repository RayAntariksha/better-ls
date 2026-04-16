//Libraries
use std::path::Path;

pub fn get_icon_by_extension(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        // Languages
        Some("rs") => " ",
        Some("c") => " ",
        Some("cpp") | Some("cc") | Some("cxx") => " ",
        Some("py") => " ",
        Some("js") => " ",
        Some("ts") => " ",
        Some("java") => " ",
        Some("kt") => " ",
        Some("go") => " ",
        Some("lua") => " ",

        // Web
        Some("html") => "",
        Some("css") => "",
        Some("json") => "",
        Some("yaml") | Some("yml") => " ",

        // Docs
        Some("md") => " ",
        Some("txt") => " ",

        // Images
        Some("png") | Some("jpg") | Some("jpeg") | Some("gif") => " ",

        // Archives
        Some("zip") | Some("tar") | Some("gz") | Some("rar") => " ",

        // Media
        Some("mp3") => " ",
        Some("mp4") => " ",

        _ => " ",
    }
}
