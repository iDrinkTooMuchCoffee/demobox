use std::fs;
use std::env;
use std::option::Option;

extern crate ini;
use ini::Ini;


mod demo;
mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    //let path = config::read_cfg(args[1].to_string(), args[2].to_string());
    //println!("{:?}", &path);
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
