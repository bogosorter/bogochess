use crate::chess::GameState;
use crate::chess::PieceType;
use crate::chess::Color;
use crate::chess::Move;

use std::cmp::Ordering;
use std::time::Instant;

pub struct SearchOptions {
    pub white_time: Option<u32>,
    pub white_increment: Option<u32>,
    pub black_time: Option<u32>,
    pub black_increment: Option<u32>,
    pub moves_to_go: Option<u32>
}

impl SearchOptions {
    pub fn new() -> SearchOptions {
        SearchOptions {
            white_time: None,
            white_increment: None,
            black_time: None,
            black_increment: None,
            moves_to_go: None
        }
    }

    fn get_time(&self, color: Color) -> (u32, u32) {
        if color == Color::White {
            (self.white_time.unwrap(), self.moves_to_go.unwrap())
        } else {
            (self.black_time.unwrap(), self.moves_to_go.unwrap())
        }
    }
}

pub fn best_move(state: &mut GameState, options: &SearchOptions) -> (Option<Move>, f32) {
    iterative_deepening(state, options, &mut SearchStatistics::new())
}

pub struct SearchStatistics {
    pub nodes: u32
}

impl SearchStatistics {
    pub fn new() -> SearchStatistics {
        SearchStatistics { nodes: 0 }
    }
}


pub fn minimax(state: &mut GameState, depth: u32, statistics: &mut SearchStatistics) -> (Option<Move>, f32) {
    let minimizing = state.active_color == Color::Black;
    let mut best = None;
    let mut best_score = if minimizing { f32::MAX } else { f32::MIN };

    let moves = state.moves();
    if moves.is_empty() {
        statistics.nodes += 1;
        if state.in_check() {
            return (None, if minimizing { 1.0 } else { -1.0 })
        } else {
            return (None, 0.0);
        }
    }

    for m in moves {
        state.apply(&m);
        statistics.nodes += 1;

        let score;
        if depth == 1 {
            score = eval(state);
        } else {
            (_, score) = minimax(state, depth - 1, statistics);
        }

        state.undo(&m);

        if minimizing && score < best_score || !minimizing && score > best_score {
            best = Some(m);
            best_score = score;
        }
    }

    (best, best_score)
}

pub fn alphabeta(state: &mut GameState, depth: u32, mut alpha: f32, mut beta: f32, statistics: &mut SearchStatistics) -> (Option<Move>, f32) {
    let minimizing = state.active_color == Color::Black;
    let mut best = None;
    let mut best_score = if minimizing { f32::MAX } else { f32::MIN };

    let mut moves = state.moves();
    if moves.is_empty() {
        statistics.nodes += 1;
        if state.in_check() {
            return (None, if minimizing { 1.0 } else { -1.0 })
        } else {
            return (None, 0.0);
        }
    }

    moves.sort_by(compare_moves);
    for m in moves {
        state.apply(&m);
        statistics.nodes += 1;

        let score;
        if depth == 1 {
            score = eval(state);
        } else {
            (_, score) = alphabeta(state, depth - 1, alpha, beta, statistics);
        }

        state.undo(&m);

        if minimizing && score < best_score || !minimizing && score > best_score {
            best = Some(m);
            best_score = score;

            if minimizing && best_score < alpha || !minimizing && best_score > beta {
                return (best, best_score)
            }

            if minimizing {
                beta = beta.min(best_score);
            } else {
                alpha = alpha.max(best_score);
            }
        }
    }

    (best, best_score)
}

pub fn timed_alphabeta(state: &mut GameState, depth: u32, mut alpha: f32, mut beta: f32, statistics: &mut SearchStatistics, elapsed: Instant, available: u32) -> Option<(Option<Move>, f32)> {
    if elapsed.elapsed().as_millis() > available as u128 {
        return None
    }

    let minimizing = state.active_color == Color::Black;
    let mut best = None;
    let mut best_score = if minimizing { f32::MAX } else { f32::MIN };

    let mut moves = state.moves();
    if moves.is_empty() {
        statistics.nodes += 1;
        if state.in_check() {
            return Some((None, if minimizing { 1.0 } else { -1.0 }))
        } else {
            return Some((None, 0.0));
        }
    }

    moves.sort_by(compare_moves);
    for m in moves {
        state.apply(&m);
        statistics.nodes += 1;

        let score;
        if depth == 1 {
            score = eval(state);
        } else {
            (_, score) = timed_alphabeta(state, depth - 1, alpha, beta, statistics, elapsed, available)?;
        }

        state.undo(&m);

        if minimizing && score < best_score || !minimizing && score > best_score {
            best = Some(m);
            best_score = score;

            if minimizing && best_score < alpha || !minimizing && best_score > beta {
                return Some((best, best_score))
            }

            if minimizing {
                beta = beta.min(best_score);
            } else {
                alpha = alpha.max(best_score);
            }
        }
    }

    Some((best, best_score))
}

fn iterative_deepening(state: &mut GameState, options: &SearchOptions, statistics: &mut SearchStatistics) -> (Option<Move>, f32) {
    let mut m = None;
    let mut score = f32::MIN;

    let (time, moves_to_go) = options.get_time(state.active_color);
    let time_available = time / (moves_to_go + 1);

    let mut i = 1;
    let start = Instant::now();
    loop {
        if let Some((new_m, new_score)) = timed_alphabeta(state, i, f32::MIN, f32::MAX, statistics, start, time_available) {
            m = new_m;
            score = new_score;
            let ns = start.elapsed().as_micros();
            let nps = statistics.nodes * 1000000 / ns as u32;
            println!("info depth {} seldepth {} score cp {} nodes {} nps {} time {}", i, i, (score * 39.0 * 100.0).round(), statistics.nodes, nps, ns / 1000);

            if score.abs() == 1.0 {
                return (m, score)
            }
            i += 1;
        } else {
            break;
        }
    }

    (m, score)
}

fn compare_moves(a: &Move, b: &Move) -> Ordering {
    if a.promotion != b.promotion {
        return b.promotion.cmp(&a.promotion);
    }

    if a.captured != b.captured {
        return b.captured.cmp(&a.captured);
    }

    Ordering::Less
}

fn eval(state: &mut GameState) -> f32 {
    let score: f32 = state.board.iter().map(|(position, piece)| {
        let value = match piece.t {
            PieceType::Pawn => {
                let forward = if piece.color == Color::White {
                    6 - position.row
                } else {
                    position.row - 1
                };
                1.0 + forward as f32 * 0.1
            },
            PieceType::Knight => 3.0,
            PieceType::Bishop => 3.0,
            PieceType::Rook => 5.0,
            PieceType::Queen => 9.0,
            PieceType::King => 0.0
        };

        if piece.color == Color::White {
            value
        } else {
            -value
        }
    }).sum();

    score / 40.0
}
