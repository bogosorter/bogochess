use crate::core::model::{Move, Position, TRANSPOSITION_MASK, TranspositionEntry, TranspositionTable, TranspositionType, ZobristValues};

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


impl Position {
    pub fn search(&mut self, tt: &mut TranspositionTable, zobrist: &ZobristValues, options: &SearchOptions) -> Option<Move> {
        let (_, m) = iterative_deepening(self, tt, zobrist, options, &mut SearchStatistics::new());
        m
    }
}


fn iterative_deepening(position: &mut Position, tt: &mut TranspositionTable, zobrist: &ZobristValues, options: &SearchOptions, statistics: &mut SearchStatistics) -> (f32, Option<Move>) {
    let start = Instant::now();
    let search_time = options.search_time(position.current_player);
    let deadline = start + Duration::from_millis(search_time as u64);

    let mut i = 1;
    let mut current_move = None;
    let mut current_score = f32::MIN;
    let mut history = [[[0; 64]; 64]; 2];

    while Instant::now() < deadline {
        let mut options = AlphaBetaOptions {
            position,
            max_depth: i,
            deadline,
            statistics: statistics,
            history: &mut history,
            tt,
            zobrist
        };

        if let Some((new_move, new_score)) = alphabeta(&mut options, i, f32::MIN, f32::MAX) {
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
    pub position: &'a mut Position,
    pub max_depth: u32,
    pub deadline: Instant,
    pub statistics: &'a mut SearchStatistics,
    pub history: &'a mut [[[u32; 64]; 64]; 2],
    pub tt: &'a mut TranspositionTable,
    pub zobrist: &'a ZobristValues
}

pub fn alphabeta(options: &mut AlphaBetaOptions, depth: u32, alpha: f32, beta: f32) -> Option<(Option<Move>, f32)> {

    // End the search earlier if the time has run out
    if Instant::now() > options.deadline {
        return None
    }

    if options.position.ended {
        return Some((None, options.position.value()));
    }

    let mut best_move = None;
    let mut best_score = f32::MIN;
    let mut current_alpha = alpha;
    let mut current_beta = beta;

    // Try to find the current position on the transposition table
    let hash = options.position.hash(options.zobrist);
    if let Some(entry) = options.tt[(hash as usize) & TRANSPOSITION_MASK].as_ref() && entry.hash == hash && entry.depth >= depth {
        match entry.t {
            TranspositionType::Exact => return Some((entry.best_move.clone(), entry.value)),
            TranspositionType::LowerBound => {
                if entry.value >= beta {
                    return Some((entry.best_move.clone(), entry.value));
                } else {
                    current_alpha = current_alpha.max(entry.value);
                }
            },
            TranspositionType::UpperBound => {
                if entry.value <= alpha {
                    return Some((entry.best_move.clone(), entry.value));
                } else {
                    current_beta = current_beta.min(entry.value);
                }
            }
        }
        best_move = entry.best_move.clone();
    }

    // End of normal search, pass on to quiescent
    if depth == 0 {
        return Some(quiescent(options, 0, current_alpha, current_beta));
    }

    options.statistics.nodes += 1;

    let mut moves = options.position.moves();

    // Look for check-mate or stalemate
    if moves.is_empty() {
        let score = if options.position.in_check() { -1.0 } else { 0.0 };
        return Some((None, score));
    }

    moves.sort_by(|a, b| compare_moves(&best_move, &options.history[options.position.current_player as usize], a, b));

    for m in moves {
        options.position.apply(&m);
        // We invert alpha and beta since the next player expects scores
        // according to his perspective
        let (_, score) = alphabeta(options, depth - 1, -current_beta, -current_alpha)?;
        options.position.undo(&m);

        // Scores are returned from the next player's perspective, so we have to
        // invert them
        let score = -score;

        if score > best_score {
            best_move = Some(m);
            best_score = score;

            // Return earlier if the score is better than the worst the
            // minimizing player can do
            if score >= current_beta {
                // Update the history table according to the history heuristics
                let update = depth * depth;
                let from = best_move.as_ref().unwrap().origin.index();
                let to = best_move.as_ref().unwrap().destination.index();
                options.history[options.position.current_player as usize][from][to] += update;

                options.tt[hash as usize & TRANSPOSITION_MASK] = Some(TranspositionEntry { hash, depth, value: score, best_move: best_move.clone(), t: TranspositionType::LowerBound });
                return Some((best_move, score));
            }

            current_alpha = current_alpha.max(score);
        }
    }

    options.tt[hash as usize & TRANSPOSITION_MASK] = Some(TranspositionEntry { hash, depth, value: best_score, best_move: best_move.clone(), t:
        if best_score > alpha {
            TranspositionType::Exact
        } else {
            TranspositionType::UpperBound
        }
    });
    Some((best_move, best_score))
}

pub fn quiescent(options: &mut AlphaBetaOptions, depth: u32, mut alpha: f32, beta: f32) -> (Option<Move>, f32) {
    options.statistics.nodes += 1;
    options.statistics.selective_depth = options.statistics.selective_depth.max(options.max_depth + depth);

    if options.position.ended {
        return (None, options.position.value());
    }

    let mut moves = options.position.moves();

    // Look for check-mate or stalemate
    if moves.is_empty() {
        let score = if options.position.in_check() { -1.0 } else { 0.0 };
        return (None, score);
    }

    moves = moves.into_iter().filter(|m| !m.captured.is_none() || !m.promotion.is_none()).collect();

    // If there are no moves with captures, the quiescent search should be ended
    if moves.is_empty() {
        return (None, options.position.value());
    }

    let mut best_move = None;
    let mut best_score = options.position.value(); // stand-pat score
    alpha = alpha.max(best_score);

    if best_score >= beta {
        return (None, best_score);
    }

    moves.sort_by(|a, b| compare_moves(&None, &options.history[options.position.current_player as usize], a, b));

    for m in moves {
        options.position.apply(&m);
        // We invert alpha and beta since the next player expects scores
        // according to his perspective
        let (_, score) = quiescent(options, depth + 1, -beta, -alpha);
        options.position.undo(&m);

        // Scores are returned from the next player's perspective, so we have to
        // invert them
        let score = -score;

        if score > best_score {
            best_move = Some(m);
            best_score = score;

            // Return earlier if the score is better than the worst the
            // minimizing player can do
            if score >= beta {
                // Update the history table according to the history heuristics
                let from = best_move.as_ref().unwrap().origin.index();
                let to = best_move.as_ref().unwrap().destination.index();
                options.history[options.position.current_player as usize][from][to] += 1;

                return (best_move, score);
            }

            alpha = alpha.max(score);
        }
    }

    (best_move, best_score)
}

fn compare_moves(best_move: &Option<Move>, history: &[[u32; 64]; 64], a: &Move, b: &Move) -> Ordering {
    if let Some(m) = best_move {
        if m.origin == a.origin && m.destination == a.destination {
            return Ordering::Less;
        }
        if m.origin == b.origin && m.destination == b.destination {
            return Ordering::Greater;
        }
    }

    if let Some(ordering) = a.partial_cmp(b) {
        return ordering;
    }

    let a_from = a.origin.index();
    let a_to = a.destination.index();
    let a_heuristic = history[a_from][a_to];

    let b_from = b.origin.index();
    let b_to = b.destination.index();
    let b_heuristic = history[b_from][b_to];

    return b_heuristic.cmp(&a_heuristic);
}
