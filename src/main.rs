use std::io::Write;

mod log;
mod builtins;
mod cd;

const RED: &'static str = "\x1B[31m";
const GRN: &'static str = "\x1B[32m";

fn main() {
    // Store the previous process return value.
    let mut prev_ret: bool = true;

    loop {
        let cwd = std::env::current_dir().unwrap();

        print!("{}\x1B[1m=(V),,(V)=\x1B[0m {}\n$ ",
            if prev_ret { GRN } else { RED },
            cwd.display());
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                // EOF and no bytes read.
                if n == 0 && input.is_empty() { break; }
            },
            Err(e) => log::log_fatal(e)
        }

        if let Some('\n') = input.chars().next_back() {
            // Strip the newline.
            input.pop();
        } else {
            // No newline?  So it is the EOF so continue the loop.
            print!("\n");
            continue;
        }

        // Is it an empty command?
        if input.is_empty() { continue; }

        /* Start the command. */

        let mut args_it = input.trim().split(" ");
        // Is it a built-in command?
        if let Ok(ret) = builtins::run(args_it.clone()) {
            prev_ret = ret;
            continue;
        }

        let cmd = args_it.next().unwrap();
        let mut child = std::process::Command::new(cmd);
        for arg in args_it {
            if !arg.is_empty() { child.arg(arg); }
        }

        match child.spawn() {
            Ok(mut child) => {
                match child.wait() {
                    Ok(ecode) => {
                        prev_ret = ecode.success();
                    },
                    Err(e) => log::log_fatal(e)
                };
            },
            Err(e) => {
                // Ignore ErrorKind NotFound, and fatally log other kinds.
                if e.kind() == std::io::ErrorKind::NotFound {
                    println!("{}: command not found", cmd);
                    prev_ret = false;
                } else {
                    log::log_fatal(e);
                }
            }
        }
    } /* loop */
}
