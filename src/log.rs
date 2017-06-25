use std;

pub fn log_fatal(errmsg: std::io::Error) {
    eprintln!("{}", errmsg);
    std::process::exit(1);
}
