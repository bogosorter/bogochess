use crate::chess::GameState;
use crate::chess::PieceType;
use crate::chess::Color;
use crate::chess::Move;

pub fn best_move(state: &mut GameState) -> (Option<Move>, f32) {
    alphabeta(state, 4, f32::MIN, f32::MAX)
}

pub fn minimax(state: &mut GameState, depth: u32) -> (Option<Move>, f32) {
    let minimizing = state.active_color == Color::Black;
    let mut best = None;
    let mut best_score = if minimizing { f32::MAX } else { f32::MIN };

    let moves = state.moves();
    if moves.is_empty() {
        if state.in_check() {
            return (None, if minimizing { 1.0 } else { -1.0 })
        } else {
            return (None, 0.0);
        }
    }

    for m in moves {
        state.apply(&m);

        let score;
        if depth == 1 {
            score = eval(state);
        } else {
            (_, score) = minimax(state, depth - 1);
        }

        state.undo(&m);

        if minimizing && score < best_score || !minimizing && score > best_score {
            best = Some(m);
            best_score = score;
        }
    }

    (best, best_score)
}

pub fn alphabeta(state: &mut GameState, depth: u32, mut alpha: f32, mut beta: f32) -> (Option<Move>, f32) {
    let minimizing = state.active_color == Color::Black;
    let mut best = None;
    let mut best_score = if minimizing { f32::MAX } else { f32::MIN };

    let moves = state.moves();
    if moves.is_empty() {
        if state.in_check() {
            return (None, if minimizing { 1.0 } else { -1.0 })
        } else {
            return (None, 0.0);
        }
    }

    for m in moves {
        state.apply(&m);

        let score;
        if depth == 1 {
            score = eval(state);
        } else {
            (_, score) = alphabeta(state, depth - 1, alpha, beta);
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
                alpha = alpha.min(best_score);
            }
        }
    }

    (best, best_score)
}

pub fn eval(state: &mut GameState) -> f32 {
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

    score / 39.0
}
