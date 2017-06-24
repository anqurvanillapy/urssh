use std::str::Split;

pub fn run(s: Split<&str>) -> Result<(), ()> {
    let mut args_it = s;

    match args_it.next() {
        Some("cd") => println!("cd!"),
        _ => return Err(())
    };

    Ok(())
}
