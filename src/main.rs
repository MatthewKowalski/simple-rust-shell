use std::io;
use std::process::Command;
use std::io::Write;
use std::path::Path;
use std::env;

fn main() {
    loop {
        print!("-> ");
        io::stdout().flush();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Everything after the first whitespace character is interpreted as args to the command  
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },

            "exit" => return,

            command => {
                let mut child = Command::new(command).args(args).spawn();

                // gracefully handle malformed user input
                match child {
                    // child.wait() -- don't accept another command until this one completes
                    Ok(mut child) => { child.wait(); },
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}
