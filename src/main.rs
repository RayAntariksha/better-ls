use std::fs;
use std::io;

fn main() {
    let item_list = read("../zellij/").unwrap();
    for item in item_list {
        print_items(item);
    }
}

fn read(path: &str) -> io::Result<Vec<String>> {
    let return_value: Vec<String> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            // file_name() gives the bare name (e.g., "main.rs" instead of "./main.rs")
            entry.file_name()
                .into_string() // Converts OsString to Result<String, OsString>
                .ok()          // Converts Result to Option, discarding non-UTF8 names
        })
    .collect();
    Ok(return_value)
}
fn print_items(item: String) {
    if !item.contains('.'){
        println!(" {}", item);
    }else {
        let split_item: Vec<_> = item.split('.').collect();
        match split_item.last(){
            Some(&"rs") => print!(" "),
            Some(&"toml") => print!(" "),
            Some(&"git") => print!(" "),
            Some(&"gitignore") => print!(" "),
            Some(&"lock") => print!(" "),

            // Common programming languages
            Some(&"c") => print!(" "),
            Some(&"cpp") | Some(&"cc") | Some(&"cxx") => print!(" "),
            Some(&"h") | Some(&"hpp") => print!(" "),
            Some(&"py") => print!(" "),
            Some(&"js") => print!(" "),
            Some(&"ts") => print!(" "),
            Some(&"java") => print!(" "),
            Some(&"kt") | Some(&"kts") => print!(" "),
            Some(&"go") => print!(" "),
            Some(&"php") => print!(" "),
            Some(&"rb") => print!(" "),
            Some(&"swift") => print!(" "),
            Some(&"cs") => print!("󰌛 "),
            Some(&"sh") => print!(" "),
            Some(&"bash") => print!(" "),

            // Web stuff
            Some(&"html") => print!(" "),
            Some(&"css") => print!(" "),
            Some(&"scss") => print!(" "),
            Some(&"json") => print!(" "),
            Some(&"yaml") | Some(&"yml") => print!(" "),
            Some(&"xml") => print!("󰗀 "),

            // Config & docs
            Some(&"md") => print!(" "),
            Some(&"txt") => print!(" "),
            Some(&"ini") | Some(&"conf") => print!(" "),
            Some(&"env") => print!(" "),

            // Images
            Some(&"png") | Some(&"jpg") | Some(&"jpeg") | Some(&"gif") | Some(&"webp") => print!(" "),
            Some(&"svg") => print!("󰜡 "),
            Some(&"ico") => print!(" "),

            // Archives
            Some(&"zip") | Some(&"tar") | Some(&"gz") | Some(&"rar") | Some(&"7z") => print!(" "),

            // Audio / Video
            Some(&"mp3") | Some(&"wav") | Some(&"flac") => print!(" "),
            Some(&"mp4") | Some(&"mkv") | Some(&"avi") => print!(" "),

            // Databases
            Some(&"db") | Some(&"sqlite") => print!(" "),
            Some(&"sql") => print!(" "),

            // Default fallback
            _ => print!(" "),
        }
        println!("{}", item);
    }
    
}
