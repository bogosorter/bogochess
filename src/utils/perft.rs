use crate::core::model::{GameState, Move};

pub fn perft(game_state: &mut GameState, depth: u32) -> u32 {
    let result = perft_aux(game_state, depth, 0);
    println!("nodes searched: {}", result);
    result
}

fn perft_aux(game_state: &mut GameState, depth: u32, current_depth: u32) -> u32 {
    if current_depth == depth {
        return 1;
    }

    let mut result = 0;
    for m in moves(game_state) {
        game_state.apply(&m);
        let count = perft_aux(game_state, depth, current_depth + 1);
        result += count;
        game_state.undo(&m);

        if current_depth == 0 {
            println!("{}: {}", m, count);
        }
    }

    result
}

// We define a custom moves function because we want to generate legal moves
// instead of pseudo-legal moves
pub fn moves(game_state: &mut GameState) -> Vec<Move> {
    let moves = game_state.moves();

    // Since the generated moves are pseudo-legal (see comment), we have to
    // filter those that will result in a check
    moves.into_iter().filter(|m| {
        game_state.apply(m);
        game_state.current_player = !game_state.current_player;
        let in_check = game_state.in_check();
        game_state.current_player = !game_state.current_player;
        game_state.undo(m);
        !in_check
    }).collect()
}
