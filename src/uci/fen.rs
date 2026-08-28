use crate::core::model::*;
use std::fmt::Display;

pub const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse(fen: &str) -> Option<Position> {
    let words: Vec<&str> = fen.split_whitespace().collect();
    if words.len() != 6 {
        return None
    }

    let board = parse_board(words[0])?;
    let active_color = match words[1] {
        "w" => Color::White,
        "b" => Color::Black,
        _ => return None
    };
    let castling = words[2].chars().filter_map(parse_castling_piece).fold(Castling::empty(), |acc, r| acc | r);
    let en_passant = match words[3] {
        "-" => None,
        _ => Some(parse_square(words[3])?)
    };
    let halfmoves = words[4].parse::<u8>().ok()?;

    Some(Position {
        board,
        ended: false,
        current_player: active_color,
        castling,
        en_passant,
        halfmoves
    })
}

pub fn parse_with_moves(fen: &str, moves: &[&str]) -> Option<Position> {
    let mut position = parse(fen)?;

    for s in moves {
        let m = position.moves().into_iter().find(|m| m.to_string() == *s)?;
        position.apply(&m);
    }

    Some(position)
}

fn parse_board(fen: &str) -> Option<Board> {
    let mut position = 54;
    let mut board = Board::new();

    for c in fen.chars() {
        if c.is_numeric() {
            position += c.to_digit(10).unwrap() as usize;
        } else if c == '/' {
            position -= 8;
        } else {
            if !parse_piece(&mut board, position, c) {
                return None
            }
            position += 1;
        }
    }

    Some(board)
}

fn parse_piece(board: &mut Board, position: usize, fen: char) -> bool {
    if fen.is_uppercase() {
        board.colors[0] |= 1 << position;
    } else {
        board.colors[1] |= 1 << position;
    }

    match fen.to_ascii_uppercase() {
        'P' => board.pieces[0] |= 1 << position,
        'N' => board.pieces[1] |= 1 << position,
        'B' => board.pieces[2] |= 1 << position,
        'R' => board.pieces[3] |= 1 << position,
        'Q' => board.pieces[4] |= 1 << position,
        'K' => board.pieces[5] |= 1 << position,
        _ => return false
    }

    true
}

fn parse_castling_piece(fen: char) -> Option<Castling> {
    match fen {
        'Q' => Some(Castling::WhiteQueen),
        'K' => Some(Castling::WhiteKing),
        'q' => Some(Castling::BlackQueen),
        'k' => Some(Castling::BlackKing),
        _ => None
    }
}

fn parse_square(fen: &str) -> Option<usize> {
    if fen.len() != 2 {
        return None
    }

    let column = match fen.chars().nth(0).unwrap() {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return None
    };
    let row_char = fen.chars().nth(1).unwrap();
    let row = if row_char.is_numeric() {
        let value = row_char.to_digit(10).unwrap() as i32;
        if value == 0 || value > 8 {
            return None;
        }
        value - 1
    } else {
        return None;
    };

    return Some((row * 8 + column) as usize)
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "move")
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        write!(f, "{}", character)
    }
}
