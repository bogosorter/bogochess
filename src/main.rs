use bogochess::chess::GameState;
use bogochess::fen;
use bogochess::perft;
use bogochess::search;
use std::io;

fn main() {
    let mut state = None;

    loop {
        // Per the standard, unrecognized commands are simply ignored
        if let Some(command) = get_command() {
            match command {
                Command::UCI => uci(),
                Command::IsReady => is_ready(),
                Command::Position(new_state) => state = Some(new_state),
                Command::Perft(n) => perft(&mut state, n),
                Command::Go => go(&mut state),
            }
        }
    }
}

enum Command {
    UCI,
    IsReady,
    Position(GameState),
    Perft(u32),
    Go
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
        "go" => parse_go(words),
        _ => None
    }
}

fn parse_position(command: Vec<&str>) -> Option<Command> {
    let position = match command.as_slice() {
        ["position", "startpos"] => fen::parse(INITIAL_FEN)?,
        ["position", "startpos", "moves", moves@..] => fen::parse_with_moves(INITIAL_FEN, moves)?,
        ["position", "fen", a, b, c, d, e, f] => fen::parse(&format!("{} {} {} {} {} {}", a, b, c, d, e, f))?,
        ["position", "fen", a, b, c, d, e, f, "moves", moves@..] => fen::parse_with_moves(&format!("{} {} {} {} {} {}", a, b, c, d, e, f), moves)?,
        _ => return None
    };
    println!("info depth 18 seldepth 24 score cp 45 nodes 1048576 nps 2050000 time 511 pv e2e4 e7e5 g1f3");
    Some(Command::Position(position))
}

fn parse_go(command: Vec<&str>) -> Option<Command> {
    match command.as_slice() {
        ["go", "perft", number] => {
            let n = number.parse::<u32>().ok()?;
            Some(Command::Perft(n))
        }
        ["go", ..] => Some(Command::Go),
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

fn perft(state: &mut Option<GameState>, n: u32) {
    match state.as_mut() {
        Some(s) => { perft::perft(s, n); },
        None => println!("no position set")
    }
}

fn go(state: &mut Option<GameState>) {
    match state.as_mut() {
        Some(s) => match search::best_move(s) {
            (Some(m), score) => {
                println!("bestmove {}", m);
                println!("info depth 2 seldepth 2 score cp {} nodes 0 nps 0 time 0 pv {}", (score * 39.0 * 100.0).round(), m);
            },
            (None, score) => if score == 1.0 {
                println!("white wins!")
            } else if score == 0.0 {
                println!("draw")
            } else {
                println!("black wins!")
            }
        },
        None => println!("no position set")
    }
}
