use std::io;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

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
    let ret: String = format!("Trouble finding section or key. {} : {}", &_section, &_key).to_string();
    return String::from(ret)
}

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
