use std::fs;
use std::error::Error;
use std::io::prelude::*;
use std::option::Option;
use config;

pub struct Command {
    pub cmd: String,
    pub item: String,
    pub dest: String,
}

impl Command {
    pub fn new(args: &[String]) -> Result<Command, &'static str> {
        if args.contains(&String::from("ls")) { 
            if args.len() < 3 {
                let path: String = config::read_cfg("General".to_string(), "DemoPath".to_string());
                Command::list_demos(path+"/");
                break;
            }
            else {
                let path: String = format!("{}/", args[2]);
                Command::list_demos(path);
                break;
            }
        }
        else if args.len() < 4 {
            return Err("you must enter at least 4 arguments.");
        }
        let cmd = args[1].clone();
        let item = args[2].clone();
        let dest = args[3].clone();

        Ok(Command {cmd, item, dest })
    }

    fn list_demos(path: String) { 
        let dir = fs::read_dir(&path);
        let pathlen: usize = path.len();
        let dir = match dir {
            Ok(dct) => dct,
            Err(e) => {
                panic!("{}: {}", e, path);
            }
        };

        for file in dir {
            let slice: &str = &file.unwrap().path().display().to_string()[pathlen..]; // Substring to just the filename.
            if slice.contains(".dem") {
                println!("{}", slice);
            }
        }
    }
}
