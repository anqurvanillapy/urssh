use std::str::Split;

use cd;

pub fn run(mut args_it: Split<&str>) -> Result<bool, ()> {
    let ret = match args_it.next() {
        Some("cd") => Ok(cd::run(args_it)),
        _ => Err(())
    };

    ret
}
