mod ui;
use std::{
    ffi::OsStr,
    fs,
    io::{self, Write},
    path::Path,
    process::exit,
};

fn parse_extensions(string_ext: &String) -> Vec<String> {
    let extensions: Vec<String> = string_ext
        .replace(" ", "")
        .replace("\n", "")
        .replace(".", "")
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

fn handle_path(path: &mut String) -> Result<bool, String> {
    *path = path.replace("\n", "");
    let exists = Path::new(&path).exists();
    if !exists {
        return Err(format!("Path {} doesn't exists", path));
    }
    Ok(true)
}

fn handle_extensions(string_ext: &String) {
    let extensions = parse_extensions(string_ext);
    println!(">> Deleting {:?} from", extensions);
}

fn delete(source_path: String, extensions: Vec<String>) {
    let paths = fs::read_dir(source_path).unwrap();

    for p in paths {
        let path = p.unwrap().path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(OsStr::to_str);
        match ext {
            Some(e) => {
                if !extensions.contains(&e.to_string()) {
                    continue;
                }
                println!("Found: {}", path.display());
            }
            None => (),
        }
    }
}

fn main() {
    ui::welcome();

    let mut path = String::new();
    input("Path: ", &mut path);

    match handle_path(&mut path) {
        Ok(_) => {}
        Err(error) => {
            println!("[ERROR] - {}", error);
            exit(-1)
        }
    }

    let mut string_ext = String::new();
    input("Extensions (comma separated): ", &mut string_ext);

    let extensions = parse_extensions(&string_ext);

    let mut confirm = String::new();
    input(
        format!("Are you sure you want to clean {} [y/n] ", path).as_str(),
        &mut confirm,
    );
    let confirm_words = vec!["yes", "y"];
    if !confirm_words.contains(&confirm.to_lowercase().as_str()) {
        delete(path, extensions);
    } else {
        println!("Aborting...");
    }
}
