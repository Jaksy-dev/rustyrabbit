mod calculate;
mod uci;
use chess::Board;
use std::io;

fn main() {
    let mut board = Board::default();

    let mut command = String::new();
    loop {
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        uci::parse_command(command.trim(), &mut board);
        command.clear();
    }
}
