use std::fs;
use std::env;
use std::process;
use std::option::Option;

extern crate ini;
use ini::Ini;

mod command;

mod demo;
mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = command::Command::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //println!("{:?}", args);
    //let path = config::read_cfg(args[1].to_string(), args[2].to_string());
    //println!("{:?}", &path);
}

