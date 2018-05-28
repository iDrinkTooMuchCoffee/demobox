use std::fs;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

extern crate ini;
use ini::Ini;

// Reads from the user cfg file.
pub fn read_cfg() -> &'static str {
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

// Generate default config.ini
pub fn write_defaults() {
    let mut conf = Ini::new();
    conf.with_section(Some("General".to_owned()))
        .set("DemoPath", "/Demos")
        .set("GameRoot", "");
    conf.with_section(Some("Groupings".to_owned()));
    conf.write_to_file("config.ini").unwrap();
}
