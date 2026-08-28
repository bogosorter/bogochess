use crate::core::model::{Position, Move};

pub fn perft(position: &mut Position, depth: u32) -> u32 {
    let result = perft_aux(position, depth, 0);
    println!("nodes searched: {}", result);
    result
}

fn perft_aux(position: &mut Position, depth: u32, current_depth: u32) -> u32 {
    if current_depth == depth {
        return 1;
    }

    let mut result = 0;
    for m in moves(position) {
        position.apply(&m);
        let count = perft_aux(position, depth, current_depth + 1);
        result += count;
        position.undo(&m);

        if current_depth == 0 {
            println!("{}: {}", m, count);
        }
    }

    result
}

// We define a custom moves function because we want to generate legal moves
// instead of pseudo-legal moves
pub fn moves(position: &mut Position) -> Vec<Move> {
    let moves = position.moves();

    // Since the generated moves are pseudo-legal (see comment), we have to
    // filter those that will result in a check
    moves.into_iter().filter(|m| {
        position.apply(m);
        position.current_player = !position.current_player;
        let in_check = position.in_check();
        position.current_player = !position.current_player;
        position.undo(m);
        !in_check
    }).collect()
}
