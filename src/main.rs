use std::fs;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

extern crate ini;
use ini::Ini;

fn main() {
    list_demos(read_cfg().to_string());
}

// Generate default config.ini
fn write_defaults() {
    let mut conf = Ini::new();
    conf.with_section(Some("General".to_owned()))
        .set("DemoPath", "/Demos")
        .set("GameRoot", "");
    conf.with_section(Some("Groupings".to_owned()));
    conf.write_to_file("config.ini").unwrap();
}


// Reads from the user cfg file.
fn read_cfg() -> &'static str {
    let cfg = File::open("config.ini");
    if cfg.is_err() {
        let cfg = File::create("config.ini");
        write_defaults();
    }

    // TODO
    // TEMPORARY
    let demopath = "Demos";
    return demopath;
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
        let slice = &file.unwrap().path().display().to_string()[pathlen..]; // Substring to just the filename.
        println!("{}", slice)
    }
}

struct Demo {
    title: String,
    path: String,
}
