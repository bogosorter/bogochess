// Performs a series of Perft tests
// Tests with higher depths are ignored by default, run with --include-ignored
// to also run them.
// Sources:
// - https://chessprogramming.org/Perft
// - https://chessprogramming.org/Perft_Results

use bogochess::uci::fen;
use bogochess::utils::perft;


fn test(position: &str, depth: u32, expected: u32) {
    let mut position = fen::parse(position).unwrap();
    let result = perft::perft(&mut position, depth);
    assert_eq!(result, expected)
}

const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
const POSITION_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
const POSITION_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
const POSITION_4_MIRRORED: &str = "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1";
const POSITION_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
const POSITION_6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

#[test]
fn initial_0() {
    test(INITIAL_FEN, 0, 1);
}

#[test]
fn initial_1() {
    test(INITIAL_FEN, 1, 20);
}

#[test]
fn initial_2() {
    test(INITIAL_FEN, 2, 400);
}

#[test]
fn initial_3() {
    test(INITIAL_FEN, 3, 8902);
}

#[test]
fn initial_4() {
    test(INITIAL_FEN, 4, 197281);
}

#[test]
fn initial_5() {
    test(INITIAL_FEN, 5, 4865609);
}

#[test]
#[ignore]
fn initial_6() {
    test(INITIAL_FEN, 6, 119060324);
}

#[test]
fn kiwipete_0() {
    test(KIWIPETE, 0, 1);
}

#[test]
fn kiwipete_1() {
    test(KIWIPETE, 1, 48);
}

#[test]
fn kiwipete_2() {
    test(KIWIPETE, 2, 2039);
}

#[test]
fn kiwipete_3() {
    test(KIWIPETE, 3, 97862);
}

#[test]
fn kiwipete_4() {
    test(KIWIPETE, 4, 4085603);
}

#[test]
#[ignore]
fn kiwipete_5() {
    test(KIWIPETE, 5, 193690690);
}

#[test]
fn position_3_0() {
    test(POSITION_3, 0, 1);
}

#[test]
fn position_3_1() {
    test(POSITION_3, 1, 14);
}

#[test]
fn position_3_2() {
    test(POSITION_3, 2, 191);
}

#[test]
fn position_3_3() {
    test(POSITION_3, 3, 2812);
}

#[test]
fn position_3_4() {
    test(POSITION_3, 4, 43238);
}

#[test]
#[ignore]
fn position_3_5() {
    test(POSITION_3, 5, 674624);
}

#[test]
#[ignore]
fn position_3_6() {
    test(POSITION_3, 6, 11030083);
}

#[test]
fn position_4_0() {
    test(POSITION_4, 0, 1);
}

#[test]
fn position_4_1() {
    test(POSITION_4, 1, 6);
}

#[test]
fn position_4_2() {
    test(POSITION_4, 2, 264);
}

#[test]
fn position_4_3() {
    test(POSITION_4, 3, 9467);
}

#[test]
fn position_4_4() {
    test(POSITION_4, 4, 422333);
}

#[test]
#[ignore]
fn position_4_5() {
    test(POSITION_4, 5, 15833292);
}

#[test]
fn position_4_mirrored_0() {
    test(POSITION_4_MIRRORED, 0, 1);
}

#[test]
fn position_4_mirrored_1() {
    test(POSITION_4_MIRRORED, 1, 6);
}

#[test]
fn position_4_mirrored_2() {
    test(POSITION_4_MIRRORED, 2, 264);
}

#[test]
fn position_4_mirrored_3() {
    test(POSITION_4_MIRRORED, 3, 9467);
}

#[test]
fn position_4_mirrored_4() {
    test(POSITION_4_MIRRORED, 4, 422333);
}

#[test]
#[ignore]
fn position_4_mirrored_5() {
    test(POSITION_4_MIRRORED, 5, 15833292);
}

#[test]
fn position_5_0() {
    test(POSITION_5, 0, 1);
}

#[test]
fn position_5_1() {
    test(POSITION_5, 1, 44);
}

#[test]
fn position_5_2() {
    test(POSITION_5, 2, 1486);
}

#[test]
fn position_5_3() {
    test(POSITION_5, 3, 62379);
}

#[test]
#[ignore]
fn position_5_4() {
    test(POSITION_5, 4, 2103487);
}

#[test]
#[ignore]
fn position_5_5() {
    test(POSITION_5, 5, 89941194);
}

#[test]
fn position_6_0() {
    test(POSITION_6, 0, 1);
}

#[test]
fn position_6_1() {
    test(POSITION_6, 1, 46);
}

#[test]
fn position_6_2() {
    test(POSITION_6, 2, 2079);
}

#[test]
fn position_6_3() {
    test(POSITION_6, 3, 89890);
}

#[test]
#[ignore]
fn position_6_4() {
    test(POSITION_6, 4, 3894594);
}

#[test]
#[ignore]
fn position_6_5() {
    test(POSITION_6, 5, 164075551);
}
