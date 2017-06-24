use std::io::Write;

fn log_fatal(errmsg: std::io::Error) {
    eprintln!("{}", errmsg);
    std::process::exit(1);
}

fn main() {
    loop {
        let cwd = std::env::current_dir().unwrap();
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

        // Strip the newline.
        if let Some('\n') = input.chars().next_back() { input.pop(); }
        println!("\x1B[3m{}\x1B[0m", input);
    }
}
