mod uci;
mod state;
use std::io;

fn main() {
    let mut command = String::new();
    loop {
            io::stdin()
        .read_line(&mut command) 
        .expect("Failed to read line");

    uci::parse_command(command.trim());
    command.clear();
    }

}

