use crate::core::model::{State, Move, Position};

pub fn perft(state: &mut State, depth: u32) -> u32 {
    let result = perft_aux(state, depth, 0);
    println!("nodes searched: {}", result);
    result
}

fn perft_aux(state: &mut State, depth: u32, current_depth: u32) -> u32 {
    if current_depth == depth {
        return 1;
    }

    let mut result = 0;
    for m in moves(state) {
        state.apply(&m);
        let count = perft_aux(state, depth, current_depth + 1);
        result += count;
        state.undo(&m);

        if current_depth == 0 {
            println!("{}: {}", m, count);
        }
    }

    result
}

// We define a custom moves function because we want to generate legal moves
// instead of pseudo-legal moves
pub fn moves(state: &mut State) -> Vec<Move> {
    let mut moves = Vec::new();

    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = state.board[row][col] && piece.color == state.current_player {
                moves.extend(state.piece_move(Position::new(row as i32, col as i32), piece, false));
            }
        }
    }

    moves
}
