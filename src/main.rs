use bogochess::{chess::GameState, fen};
use std::io;

fn main() {
    let mut state = None;

    loop {
        // Per the standard, unrecognized commands are simply ignored
        if let Some(command) = get_command() {
            match command {
                Command::UCI => uci(),
                Command::IsReady => is_ready(),
                Command::Position(new_state) => state = Some(new_state)
            }
        }
    }
}

enum Command {
    UCI,
    IsReady,
    Position(GameState)
}

const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn get_command() -> Option<Command> {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("failed to read line");

    let words: Vec<&str> = command.trim().split_whitespace().collect();
    match words[0] {
        "uci" => Some(Command::UCI),
        "isready" => Some(Command::IsReady),
        "position" => parse_position(words),
        _ => None
    }
}

fn parse_position(command: Vec<&str>) -> Option<Command> {
    match command.as_slice() {
        [_, "startpos", ..] => Some(Command::Position(fen::parse(INITIAL_FEN)?)),
        [_, fen_string@..] => Some(Command::Position(fen::parse(&fen_string.join(" "))?)),
        _ => None
    }
}

fn uci() {
    let name = env!("CARGO_PKG_NAME");
    let author = env!("CARGO_PKG_AUTHORS");
    println!("id name {name}");
    println!("id author {author}");
    println!("uciok");
}

fn is_ready() {
    println!("readyok");
}
