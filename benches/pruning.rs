// Benchmarks the alpha-beta pruning algorithm by comparing it to vanilla minimax on
// various positions and depths. Tests with higher depths are ignored by default,
// run with --include-ignored
// Sources:
// - https://chessprogramming.org/Perft_Results

// - Initial position, up to depth 6
// - Kiwipete, up to depth 5
// - Position 3, up to depth 6
// - Position 4, normal and mirrored, up to depth 5
// - Position 5, up to depth 5
// - Position 6, up to depth 5


use bogochess::fen;
use bogochess::search;
use bogochess::search::SearchStatistics;

use std::time::Instant;
use std::time::Duration;

fn benchmark(name: &str, position: &str, depth: u32) {
    let mut state = fen::parse(position).unwrap();

    let mut left_statistics = SearchStatistics::new();
    let mut right_statistics = SearchStatistics::new();
    search::minimax(&mut state, depth, &mut left_statistics);
    // 100 years from now (infinite deadline)
    let infinite_deadline = Instant::now() + Duration::from_secs(365 * 86400 * 100);
    search::alphabeta(&mut state, depth, f32::MIN, f32::MAX, &mut right_statistics, infinite_deadline).unwrap();

    println!("{}", name);
    println!("minimax nodes: {}", left_statistics.nodes);
    println!("alphabeta nodes: {}", right_statistics.nodes);
    println!()
}

const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
const POSITION_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
const POSITION_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
const POSITION_4_MIRRORED: &str = "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1";
const POSITION_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
const POSITION_6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

fn main() {
    benchmark("initial_1", INITIAL_FEN, 1);
    benchmark("initial_2", INITIAL_FEN, 2);
    benchmark("initial_3", INITIAL_FEN, 3);
    benchmark("initial_4", INITIAL_FEN, 4);
    benchmark("kiwipete_1", KIWIPETE, 1);
    benchmark("kiwipete_2", KIWIPETE, 2);
    benchmark("position_3_1", POSITION_3, 1);
    benchmark("position_3_2", POSITION_3, 2);
    benchmark("position_3_3", POSITION_3, 3);
    benchmark("position_3_4", POSITION_3, 4);
    benchmark("position_3_5", POSITION_3, 5);
    benchmark("position_4_1", POSITION_4, 1);
    benchmark("position_4_2", POSITION_4, 2);
    benchmark("position_4_3", POSITION_4, 3);
    benchmark("position_4_mirrored_1", POSITION_4_MIRRORED, 1);
    benchmark("position_4_mirrored_2", POSITION_4_MIRRORED, 2);
    benchmark("position_4_mirrored_3", POSITION_4_MIRRORED, 3);
    benchmark("position_5_1", POSITION_5, 1);
    benchmark("position_5_2", POSITION_5, 2);
    benchmark("position_5_3", POSITION_5, 3);
    benchmark("position_6_1", POSITION_6, 1);
    benchmark("position_6_2", POSITION_6, 2);
    benchmark("position_6_3", POSITION_6, 3);
}
