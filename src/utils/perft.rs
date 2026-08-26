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
    let mut moves = Vec::with_capacity(220);

    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = state.board[row][col] && piece.color == state.current_player {
                state.piece_move(&mut moves, Position::new(row as i32, col as i32), piece);
            }
        }
    }

    // Since the generated moves are pseudo-legal (see comment), we have to
    // filter those that will result in a check
    moves.into_iter().filter(|m| {
        state.apply(m);
        state.current_player = !state.current_player;
        let in_check = state.in_check();
        state.current_player = !state.current_player;
        state.undo(m);
        !in_check
    }).collect()
}
