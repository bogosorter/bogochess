use crate::chess::GameState;

pub fn perft(state: &mut GameState, depth: u32) -> u32 {
    perft_aux(state, depth, 0)
}

fn perft_aux(state: &mut GameState, depth: u32, current_depth: u32) -> u32 {
    if current_depth == depth {
        return 1;
    }

    let mut result = 0;
    for m in state.moves() {
        state.apply(&m);
        let count = perft_aux(state, depth, current_depth + 1);
        result += count;
        state.undo(&m);

        if current_depth == 0 {
            println!("{}: {}", m, count);
        }
    }

    if current_depth == 0 {
        println!("nodes searched: {}", result)
    }

    result
}
