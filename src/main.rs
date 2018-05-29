use std::fs;

extern crate ini;
use ini::Ini;

use std::option::Option;

mod demo;
mod config;

fn main() {
    let path = config::read_cfg("General".to_string(), "DemoPath".to_string());
    println!("{:?}", &path);
}

// Lists all demos found in the path.
fn list_demos(path: String) { 
    let dir = fs::read_dir(&path);
    let pathlen: usize = path.len();
    let dir = match dir {
        Ok(dct) => dct,
        Err(e) => {
            panic!("{}", e);
        }
    };

    for file in dir {
        let slice: &str = &file.unwrap().path().display().to_string()[pathlen..]; // Substring to just the filename.
        if slice.contains(".dem") {
            println!("{}", slice);
        }
    }
}
