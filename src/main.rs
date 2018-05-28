use std::fs;
use std::io::prelude::*;

const FOLDER_PATH: &'static str = "C:/Users/Anthony/Desktop/Demos";

fn main() {
    list_demos(FOLDER_PATH.to_string());
}

// Lists all demos found in the path.
fn list_demos(path: String) { 
    let dir = fs::read_dir(path).unwrap();
    let pathlen: usize = FOLDER_PATH.len();
    for file in dir {
        let slice = &file.unwrap().path().display().to_string()[pathlen..];
        println!("{}", slice)
    }
}

struct Demo {
    title: String,
    path: String,
}
