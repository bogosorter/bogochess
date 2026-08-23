use crate::chess::GameState;
use crate::chess::PieceType;
use crate::chess::Color;
use crate::chess::Move;
use std::cmp::Ordering;

pub fn best_move(state: &mut GameState) -> (Option<Move>, f32) {
    alphabeta(state, 5, f32::MIN, f32::MAX, &mut SearchStatistics::new())
}

pub struct SearchStatistics {
    pub nodes: i32
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
    let score: f32 = state.board.iter().map(|(_, piece)| {
        let value = match piece.t {
            PieceType::Pawn => 1.0,
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
