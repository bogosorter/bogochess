use crate::core::model::{State, Move};

use std::cmp::Ordering;
use std::time::{Instant, Duration};

mod search_options_impl;
mod search_statistics_impl;


pub struct SearchOptions {
    pub white_time: Option<u32>,
    pub white_increment: Option<u32>,
    pub black_time: Option<u32>,
    pub black_increment: Option<u32>,
    pub moves_to_go: Option<u32>
}

pub struct SearchStatistics {
    pub depth: u32,
    pub selective_depth: u32,
    pub nodes: i64,
    pub score: f32,
    pub search_time: u64 // in microseconds
}


impl State {
    pub fn search(&mut self, options: &SearchOptions) -> Option<Move> {
        let (_, m) = iterative_deepening(self, options, &mut SearchStatistics::new());
        m
    }
}


fn iterative_deepening(state: &mut State, options: &SearchOptions, statistics: &mut SearchStatistics) -> (f32, Option<Move>) {
    let start = Instant::now();
    let search_time = options.search_time(state.curent_player);
    let deadline = start + Duration::from_millis(search_time as u64);

    let mut i = 1;
    let mut current_move = None;
    let mut current_score = f32::MIN;

    while Instant::now() < deadline {
        let mut options = AlphaBetaOptions {
            state: state,
            max_depth: i,
            deadline,
            statistics: statistics
        };

        if let Some((new_move, new_score)) = alphabeta(&mut options, 0, f32::MIN, f32::MAX) {
            current_move = new_move;
            current_score = new_score;

            statistics.depth = i;
            statistics.score = current_score;
            statistics.search_time = start.elapsed().as_micros() as u64;
            println!("{}", statistics);

            // There is no need to search further than a checkmate
            if current_score == 1.0 {
                return (current_score, current_move)
            }

            i += 1;
        }
    };

    (current_score, current_move)
}


pub struct AlphaBetaOptions<'a> {
    pub state: &'a mut State,
    pub max_depth: u32,
    pub deadline: Instant,
    pub statistics: &'a mut SearchStatistics
}

pub fn alphabeta(options: &mut AlphaBetaOptions, depth: u32, mut alpha: f32, beta: f32) -> Option<(Option<Move>, f32)> {

    // End the search earlier if the time has run out
    if Instant::now() > options.deadline {
        return None
    }

    // End of normal search, pass on to quiescent search
    if depth == options.max_depth {
        return Some(quiescent(options, depth, alpha, beta));
    }

    options.statistics.nodes += 1;

    let mut moves = options.state.moves();

    // Look for check-mate or stalemate
    if moves.is_empty() {
        let score = if options.state.in_check() { -1.0 } else { 0.0 };
        return Some((None, score));
    }

    let mut best_move = None;
    let mut best_score = f32::MIN;
    moves.sort_by(compare_moves);

    for m in moves {
        options.state.apply(&m);
        // We invert alpha and beta since the next player expects scores
        // according to his perspective
        let (_, score) = alphabeta(options, depth + 1, -beta, -alpha)?;
        options.state.undo(&m);

        // Scores are returned from the next player's perspective, so we have to
        // invert them
        let score = -score;

        if score > best_score {
            best_move = Some(m);
            best_score = score;

            // Return earlier if the score is better than the worst the
            // minimizing player can do
            if score >= beta {
                return Some((best_move, score));
            }

            alpha = alpha.max(score);
        }
    }

    Some((best_move, best_score))
}

pub fn quiescent(options: &mut AlphaBetaOptions, depth: u32, mut alpha: f32, beta: f32) -> (Option<Move>, f32) {
    options.statistics.nodes += 1;
    options.statistics.selective_depth = depth;

    let mut moves = options.state.moves();

    // Look for check-mate or stalemate
    if moves.is_empty() {
        let score = if options.state.in_check() { -1.0 } else { 0.0 };
        return (None, score);
    }

    moves = moves.into_iter().filter(|m| !m.captured.is_none() || !m.promotion.is_none()).collect();

    // If there are no moves with captures, the quiescent search should be ended
    if moves.is_empty() {
        return (None, options.state.value());
    }

    let mut best_move = None;
    let mut best_score = options.state.value(); // stand-pat score
    alpha = alpha.max(best_score);

    if best_score >= beta {
        return (None, best_score);
    }

    moves.sort_by(compare_moves);
    for m in moves {
        options.state.apply(&m);
        // We invert alpha and beta since the next player expects scores
        // according to his perspective
        let (_, score) = quiescent(options, depth + 1, -beta, -alpha);
        options.state.undo(&m);

        // Scores are returned from the next player's perspective, so we have to
        // invert them
        let score = -score;

        if score > best_score {
            best_move = Some(m);
            best_score = score;

            // Return earlier if the score is better than the worst the
            // minimizing player can do
            if score >= beta {
                return (best_move, score);
            }

            alpha = alpha.max(score);
        }
    }

    (best_move, best_score)
}

fn compare_moves(a: &Move, b: &Move) -> Ordering {
    a.partial_cmp(b).unwrap_or(Ordering::Less)
}
