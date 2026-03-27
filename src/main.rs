use std::process::Command;
fn main() {

    println!(" ./");

    print_files();
}
//This function executes the ls command and gets the name of each file/folders and stores them in
//an vector.
fn ls_to_vec(args: &str) -> Vec<String>{
    let output = Command::new("ls")
        .args(["-F", args])
        .output()
        .expect("Fatal: failed to run ls command");
    let out_string = String::from_utf8_lossy(&output.stdout);
    let vector: Vec<String> = out_string
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    return vector;
}
fn print_files() {
    let output = Command::new("ls")
        .args(["-F"])
        .output()
        .expect("Fatal: failed to run ls command");
    let out_string = String::from_utf8_lossy(&output.stdout);
    let vector: Vec<String> = out_string
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    for items in vector {
        if items.chars().last() == Some('/') {
            println!("{}", items.clone());
            let a = ls_to_vec(format!("{}", items).trim());
            println!("{:?}", a);
        }else {
            println!("{items}");
        }
    }

}
