// Performs all the possible moves, checking that doing and undoing a move is
// idempotent. Initial positions are the same as the perft tests.
// Sources: https://chessprogramming.org/Perft_Results


use bogochess::core::model::State;
use bogochess::uci::fen;

fn test(position: &str, depth: u32) {
    let mut state = fen::parse(position).unwrap();
    idempotent(&mut state, depth);
}

fn idempotent(state: &mut State, depth: u32) {
    if depth == 0 || state.ended == false {
        return;
    }

    for m in state.moves() {
        let before = state.clone();
        state.apply(&m);
        state.undo(&m);
        assert_eq!(&before, state);

        state.apply(&m);
        idempotent(state, depth - 1);
        state.undo(&m);
    }
}

const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
const POSITION_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
const POSITION_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
const POSITION_4_MIRRORED: &str = "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1";
const POSITION_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
const POSITION_6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

#[test]
fn initial() {
    test(INITIAL_FEN, 4);
}

#[test]
fn kiwipete() {
    test(KIWIPETE, 3);
}

#[test]
fn position_3() {
    test(POSITION_3, 4);
}

#[test]
fn position_4() {
    test(POSITION_4, 4);
}

#[test]
fn position_4_mirrored() {
    test(POSITION_4_MIRRORED, 4);
}

#[test]
fn position_5() {
    test(POSITION_5, 3);
}

#[test]
fn position_6() {
    test(POSITION_6, 3);
}

#[test]
fn extra() {
    test("8/2p5/3p4/KP5r/2R4k/5p2/4P1P1/8 b - - 1 1", 1);
}
