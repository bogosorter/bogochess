use crate::chess::GameState;
use crate::chess::PieceType;
use crate::chess::Color;
use crate::chess::Move;

use std::cmp::Ordering;
use std::time::Instant;
use std::time::Duration;

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
    pub nodes: u32,
    pub selective_depth: u32
}

impl SearchStatistics {
    pub fn new() -> SearchStatistics {
        SearchStatistics { nodes: 0, selective_depth: 0 }
    }
}


pub fn minimax(state: &mut GameState, depth: u32, statistics: &mut SearchStatistics) -> (Option<Move>, f32) {
    statistics.nodes += 1;

    let moves = state.moves();

    // Look for check mate or stalemate
    if moves.is_empty() {
        let score = if state.in_check() { -1.0 } else { 0.0 };
        return (None, score);
    }

    // End of search, compute score of the current board
    if depth == 0 {
        return (None, eval(state));
    }

    let mut best = None;
    let mut best_score = f32::MIN;
    for m in moves {
        state.apply(&m);
        let (_, score) = minimax(state, depth - 1, statistics);
        state.undo(&m);

        let score = -score;
        if score > best_score {
            best = Some(m);
            best_score = score;
        }
    }

    (best, best_score)
}

pub fn alphabeta(state: &mut GameState, depth: u32, max_depth: u32, mut alpha: f32, beta: f32, statistics: &mut SearchStatistics, deadline: Instant) -> Option<(Option<Move>, f32)> {
    // End the search earlier if the time has run out
    if Instant::now() > deadline {
        return None
    }

    // End of normal search, pass on to quiescent search
    if depth == max_depth {
        return Some(quiescent(state, depth, alpha, beta, statistics));
    }

    statistics.nodes += 1;

    let mut moves = state.moves();

    // Look for check-mate or stalemate
    if moves.is_empty() {
        let score = if state.in_check() { -1.0 } else { 0.0 };
        return Some((None, score));
    }

    let mut best_move = None;
    let mut best_score = f32::MIN;
    moves.sort_by(compare_moves);

    for m in moves {
        state.apply(&m);
        // We invert alpha and beta since the next player expects scores
        // according to his perspective
        let (_, score) = alphabeta(state, depth + 1, max_depth, -beta, -alpha, statistics, deadline)?;
        state.undo(&m);

        // Scores are returned from the next player's perspective, so we have to
        // invert them
        let score = -score;

        if score > best_score {
            best_move = Some(m);
            best_score = score;

            // Return earlier if the score is better than the worst the
            // minimizing player can do
            if score >= beta {
                return Some((best_move, score));
            }

            alpha = alpha.max(score);
        }
    }

    Some((best_move, best_score))
}

pub fn quiescent(state: &mut GameState, depth: u32, mut alpha: f32, beta: f32, statistics: &mut SearchStatistics) -> (Option<Move>, f32) {
    statistics.nodes += 1;
    statistics.selective_depth = depth;

    let mut moves = state.moves();

    // Look for check-mate or stalemate
    if moves.is_empty() {
        let score = if state.in_check() { -1.0 } else { 0.0 };
        return (None, score);
    }

    moves = moves.into_iter().filter(|m| !m.captured.is_none() || !m.promotion.is_none()).collect();

    // If there are no moves with captures, the quiescent search should be ended
    if moves.is_empty() {
        return (None, eval(state));
    }

    let mut best_move = None;
    let mut best_score = eval(state); // stand-pat score

    if best_score >= beta {
        return (None, best_score);
    }

    moves.sort_by(compare_moves);
    for m in moves {
        state.apply(&m);
        // We invert alpha and beta since the next player expects scores
        // according to his perspective
        let (_, score) = quiescent(state, depth + 1, -beta, -alpha, statistics);
        state.undo(&m);

        // Scores are returned from the next player's perspective, so we have to
        // invert them
        let score = -score;

        if score > best_score {
            best_move = Some(m);
            best_score = score;

            // Return earlier if the score is better than the worst the
            // minimizing player can do
            if score >= beta {
                return (best_move, score);
            }

            alpha = alpha.max(score);
        }
    }

    (best_move, best_score)
}

fn iterative_deepening(state: &mut GameState, options: &SearchOptions, statistics: &mut SearchStatistics) -> (Option<Move>, f32) {
    let mut m = None;
    let mut score = f32::MIN;

    let (time, moves_to_go) = options.get_time(state.active_color);
    let time_available = time / (moves_to_go + 1);

    let mut i = 1;
    let start = Instant::now();
    let deadline = Instant::now() + Duration::from_millis(time_available as u64);
    loop {
        if let Some((new_m, new_score)) = alphabeta(state, 0, i, f32::MIN, f32::MAX, statistics, deadline) {
            m = new_m;
            score = new_score;
            let ns = start.elapsed().as_micros();
            let nps = statistics.nodes * 1000000 / ns as u32;
            println!("info depth {} seldepth {} score cp {} nodes {} nps {} time {}", i, statistics.selective_depth, (score * 39.0 * 100.0).round(), statistics.nodes, nps, ns / 1000);

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

    if state.active_color == Color::White {
        score / 40.0
    } else {
        -score / 40.0
    }
}
