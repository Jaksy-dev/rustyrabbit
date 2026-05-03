use chess::Board;
use std::{process::exit, str::FromStr};

use crate::{DEPTH, calculate::calculate};

pub fn parse_command(command: &str, board: &mut Board) {
    if command.is_empty() {
        return;
    }
    let mut tokens: std::str::SplitAsciiWhitespace<'_> = command.split_ascii_whitespace();

    // Main command
    match tokens.next() {
        Some("uci") => println!("id name rustybunny\nuciok"),
        Some("isready") => println!("readyok"),
        Some("ucinewgame") => ucinewgame(board),
        Some("position") => position(&mut tokens, board),
        Some("go") => go(&mut tokens, board),
        Some("stop") => (), //TODO implement
        Some("quit") => exit(0),
        _ => (),
    }
}

fn ucinewgame(board: &mut Board) {
    *board = Board::default();
}

fn position(tokens: &mut std::str::SplitAsciiWhitespace, board: &mut Board) {
    match tokens.next() {
        Some("startpos") => startpos(tokens, board),
        Some("fen") => (), // TODO implement
        _ => (),
    }
}

// fn fen(tokens: &mut std::str::SplitAsciiWhitespace, board: &mut Board){}

fn startpos(tokens: &mut std::str::SplitAsciiWhitespace, board: &mut Board) {
    *board = Board::default();
    if tokens.next() != None {
        // "moves"
        while let Some(move_str) = tokens.next() {
            let cm = chess::ChessMove::from_str(move_str)
                .expect("Unable to convert from move string to chess move");

            *board = board.make_move_new(cm);
        }
    }
}

fn go(tokens: &mut std::str::SplitAsciiWhitespace, board: &mut Board) {
    tokens.next(); // wtime
    let _wtime = tokens.next().unwrap();
    tokens.next(); // btime
    let _btime = tokens.next().unwrap();
    tokens.next(); // winc
    let _winc = tokens.next().unwrap();
    tokens.next(); // binc
    let _binc = tokens.next().unwrap();
    tokens.next(); // movestogo
    let _movestogo = tokens.next().unwrap();


    let best_move = calculate(board, DEPTH);

    //println!("info score cp {}", current_score); // TODO
    println!("bestmove {}", chess::ChessMove::to_string(&best_move));

    // prints current fen
    // println!("{}", board.to_string());
}
