use std::io;
use std::fs;
use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::fs::OpenOptions;

extern crate ini;
use ini::Ini;

// Reads from the user cfg file.
pub fn read_cfg(_section: String, _key: String) -> String {
    let cfg = File::open("config.ini");
    if cfg.is_err() {
        let cfg = File::create("config.ini");
        write_defaults();
    }
    let conf = Ini::load_from_file("config.ini").unwrap();
    for (sec, prop) in &conf {
        for (key, value) in prop.iter() {
            if key == &_key {
                return value.to_string();
            }
        }
    }
    return format!("Could not find key: {}", &_key);
}

// Generate default config.ini
pub fn write_defaults() {
    let mut conf = Ini::new();
    conf.with_section(Some(" "))
        .set("encoding", "utf-8");
    conf.with_section(Some("General".to_owned()))
        .set("DemoPath", "/Demos")
        .set("GameRoot", "");
    conf.with_section(Some("Groupings".to_owned()));
    conf.write_to_file("config.ini").unwrap();
}
