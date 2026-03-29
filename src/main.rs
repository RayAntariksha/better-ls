use std::{fs, io, path::Path, env};
use colored::Colorize;

fn main() -> io::Result<()> {
    let mut show_hidden_files: bool = false;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        
    let arg = args[1].trim();
    match arg {
        "-a" => show_hidden_files = true,
        _ => {println!("Unknown argument: {}", arg)}
    }
    }
    
    let mut entries: Vec<_> = fs::read_dir("./")?
        .filter_map(|e| e.ok())
        .collect();

    // Sort: directories first, then alphabetically
    entries.sort_by_key(|e| {
        let is_file = e.path().is_file();
        (is_file, e.file_name())
    });
    println!("./");
    for entry in entries {
        if  show_hidden_files == true && is_hidden(&entry) == true{
            print_item(&entry);
        }else if is_hidden(&entry) == false{
            print_item(&entry);
        }
    }
    println!("");

    Ok(())
}

#[cfg(unix)]
fn is_hidden(entry: &fs::DirEntry) -> bool{
    if let Some(filename) = entry.file_name().to_str() {
        filename.starts_with('.')
    }else {
        false
    }
}

fn print_item(entry: &fs::DirEntry) {
    let path = entry.path();
    let name = entry.file_name();
    let name = name.to_string_lossy();

    // Directory
    if path.is_dir() {
        println!(" ├─   {}/", name.blue());
        return;
    }

    // Special filenames (more important than extensions)
    let icon = match name.as_ref() {
        ".gitignore" => " ",
        "Cargo.toml" => " ",
        "Cargo.lock" => " ",
        "Makefile" => "",
        _ => get_icon_by_extension(&path),
    };

    println!(" ├─ {} {}", icon, name);
}

fn get_icon_by_extension(path: &Path) -> &'static str {
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
