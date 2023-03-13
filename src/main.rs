mod ui;
use std::{
    io::{self, Write},
    path::Path,
    process::exit,
};

fn parse_extensions(string_ext: &String) -> Vec<String> {
    let extensions: Vec<String> = string_ext
        .replace(" ", "")
        .replace("\n", "")
        .split(",")
        .map(|x| x.to_string())
        .collect();

    extensions
}

fn input(message: &str, out: &mut String) {
    print!(">> {}", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(out).expect("Failed to read line");

    if *out == "--help" {}
}

fn handle_path(path: &String) -> Result<bool, String> {
    let exists = Path::new(path).exists();
    if !exists {
        return Err(format!("Path {} doesn't exists", path).replace("\n", ""));
    }
    Ok(true)
}

fn handle_extensions(string_ext: &String) {
    let extensions = parse_extensions(string_ext);
    println!(">> Deleting {:?} from", extensions);
}

fn main() {
    ui::welcome();

    let mut path = String::new();
    input("Path: ", &mut path);

    match handle_path(&path) {
        Ok(_) => {},
        Err(error) => {
            println!("[ERROR] - {}", error);
            exit(-1)
        }
    }

    let mut string_ext = String::new();
    input("Extensions (comma separated): ", &mut string_ext);
    handle_extensions(&string_ext);
}
