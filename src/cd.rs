use std;
use std::env::{home_dir, set_current_dir};

const USAGE: &'static str = "Usage: cd [directory]";

pub fn run(mut args: std::str::Split<&str>) -> bool {
    let argc = args.clone().count();
    match argc {
        0 => {
            if let Some(homedir) = home_dir() {
                if let Err(_) = set_current_dir(homedir) {
                    eprintln!("failed to set to home dir");
                    return false;
                }
            } else {
                eprintln!("Warning: failed to get home dir");
            }
        },
        1 => {
            let dir: &str = args.next().unwrap();
            if let Err(_) = set_current_dir(&std::path::Path::new(dir)) {
                eprintln!("cd: Directory \"{}\" does not exist", dir);
                return false;
            }
        },
        _ => {
            eprintln!("{}", USAGE);
            return false;
        }
    }

    true
}
