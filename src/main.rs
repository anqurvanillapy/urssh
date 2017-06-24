use std::io::Write;

fn log_fatal(errmsg: std::io::Error) {
    eprintln!("{}", errmsg);
    std::process::exit(1);
}

fn main() {
    loop {
        let cwd = std::env::current_dir()
            .unwrap();
        print!("\x1B[31m\x1B[1m=(V)..(V)=\x1B[0m {}\n$ ", cwd.display());
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                // EOF and no bytes read.
                if n == 0 && input.is_empty() { break; }
            },
            Err(e) => log_fatal(e)
        }

        if let Some('\n') = input.chars().next_back() {
            // Strip the newline.
            input.pop();
        } else {
            // No newline?  So it is the EOF so continue the loop.
            println!("");
            continue;
        }

        // Start the command.
        let mut args_it = input.split(" ");
        let cmd = args_it.next().unwrap();
        let mut child = std::process::Command::new(cmd);
        for arg in args_it { child.arg(arg); }

        match child.spawn() {
            Ok(mut child) => {
                match child.wait() {
                    Ok(ecode) => {
                        assert!(ecode.success());
                    },
                    Err(e) => log_fatal(e)
                };
            },
            // Ignore ErrorKind NotFound, and fatally log other kinds.
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    println!("{}", e);
                } else {
                    log_fatal(e);
                }
            }
        };
    }
}
