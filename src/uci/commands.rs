use crate::core::model::{State, Position, Move};
use crate::core::search::{SearchOptions, SearchStatistics};
use crate::uci::fen::{self, INITIAL_FEN};
use crate::utils::perft;

use std::fmt::Display;

pub enum GUICommand {
    UCI,
    IsReady,
    Position(Position),
    Perft(u32),
    Go(SearchOptions),
    Quit
}

pub enum EngineCommand<'a> {
    UCIOK,
    ReadyOK,
    BestMove(Move),
    Info(&'a SearchStatistics)
}

pub fn parse(command: &str) -> Option<GUICommand> {
    let words: Vec<&str> = command.trim().split_whitespace().collect();
    match words[0] {
        "uci" => Some(GUICommand::UCI),
        "isready" => Some(GUICommand::IsReady),
        "position" => parse_position(words),
        "go" => parse_go(words),
        "quit" => Some(GUICommand::Quit),
        _ => None
    }
}

pub fn process<'a>(command: GUICommand, state: &mut State) -> Option<EngineCommand<'a>> {
    match command {
        GUICommand::UCI => Some(EngineCommand::UCIOK),
        GUICommand::IsReady => Some(EngineCommand::ReadyOK),

        GUICommand::Position(p) => {
            state.position = Some(p);
            None
        },

        GUICommand::Perft(depth) =>
            if let Some(p) = state.position.as_mut() {
                perft::perft(p, depth);
                None
            } else { None },

        GUICommand::Go(options) =>
            if let Some(p) = state.position.as_mut() && let Some(m) = p.search(&options) {
                Some(EngineCommand::BestMove(m))
            } else { None },

        GUICommand::Quit => panic!("quit command should not be passed to process")
    }
}

fn parse_position(command: Vec<&str>) -> Option<GUICommand> {
    let position = match command.as_slice() {
        ["position", "startpos"] => fen::parse(INITIAL_FEN)?,
        ["position", "startpos", "moves", moves@..] => fen::parse_with_moves(INITIAL_FEN, moves)?,
        ["position", "fen", a, b, c, d, e, f] => fen::parse(&format!("{} {} {} {} {} {}", a, b, c, d, e, f))?,
        ["position", "fen", a, b, c, d, e, f, "moves", moves@..] => fen::parse_with_moves(&format!("{} {} {} {} {} {}", a, b, c, d, e, f), moves)?,
        _ => return None
    };
    Some(GUICommand::Position(position))
}

fn parse_go(command: Vec<&str>) -> Option<GUICommand> {
    match command.as_slice() {
        ["go", "perft", number] => {
            let n = number.parse::<u32>().ok()?;
            Some(GUICommand::Perft(n))
        }
        ["go", ..] => {
            let mut contents = SearchOptions::new();
            parse_go_arguments(&command, &mut contents);
            Some(GUICommand::Go(contents))
        },
        _ => None
    }
}

fn parse_go_arguments(mut arguments: &[&str], result: &mut SearchOptions) {
    loop {
        match arguments {
            ["wtime", time, rest@..] => {
                if let Ok(t) = time.parse::<u32>() {
                    result.white_time = Some(t);
                }
                arguments = rest;
            },
            ["winc", time, rest@..] => {
                if let Ok(t) = time.parse::<u32>() {
                    result.white_increment = Some(t);
                }
                arguments = rest;
            },
            ["btime", time, rest@..] => {
                if let Ok(t) = time.parse::<u32>() {
                    result.black_time = Some(t);
                }
                arguments = rest;
            },
            ["binc", time, rest@..] => {
                if let Ok(t) = time.parse::<u32>() {
                    result.black_increment = Some(t);
                }
                arguments = rest;
            },
            ["movestogo", ms, rest@..] => {
                if let Ok(n) = ms.parse::<u32>() {
                    result.moves_to_go = Some(n);
                }
                arguments = rest;
            },
            [] => { break; }
            _ => arguments = &arguments[1..]
        }
    }
}

impl<'a> Display for EngineCommand<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineCommand::UCIOK => {
                let name = env!("CARGO_PKG_NAME");
                let author = env!("CARGO_PKG_AUTHORS");
                write!(f, "id name {name}\nid author {author}\nuciok")
            },
            EngineCommand::ReadyOK => {
                write!(f, "readyok")
            },
            EngineCommand::BestMove(m) => {
                write!(f, "bestmove {m}")
            },
            EngineCommand::Info(statistics) => {
                let nps = statistics.nodes * 1000000 / (statistics.search_time as i64);
                let time = statistics.search_time / 1000; // milliseconds

                write!(f, "info depth {} seldepth {} score cp {} nodes {} nps {} time {}",
                    statistics.depth,
                    statistics.selective_depth,
                    (statistics.score * 10000.0).round(),
                    statistics.nodes,
                    nps,
                    time
                )
            }
        }
    }
}

// Implementation of the info command
impl Display for SearchStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", EngineCommand::Info(self))
    }
}
