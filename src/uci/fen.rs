use crate::core::model::*;

use std::collections::HashMap;
use std::fmt::Display;

pub const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse(fen: &str) -> Option<State> {
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
    let castlings: Vec<Piece> = match words[2] {
        "-" => Vec::new(),
        _ => words[2].chars().map(parse_castling_piece).collect::<Option<Vec<Piece>>>()?
    };
    let en_passant = match words[3] {
        "-" => None,
        _ => Some(parse_square(words[3])?)
    };
    let halfmoves = words[4].parse::<u32>().ok()?;

    Some(State {
        board,
        curent_player: active_color,
        castlings,
        en_passant,
        halfmoves
    })
}

pub fn parse_with_moves(fen: &str, moves: &[&str]) -> Option<State> {
    let mut position = parse(fen)?;

    for s in moves {
        let m = position.moves().into_iter().find(|m| m.to_string() == *s)?;
        position.apply(&m);
    }

    Some(position)
}

fn parse_board(fen: &str) -> Option<Board> {
    let mut i = 0;
    let mut j = 0;
    let mut board = HashMap::new();

    for c in fen.chars() {
        if c.is_numeric() {
            j += c.to_digit(10).unwrap() as i32;
        } else if c == '/' {
            i += 1;
            j = 0;
        } else {
            let piece = parse_piece(c)?;
            board.insert(Position::new(i, j), piece);
            j += 1;
        }
    }

    Some(board)
}

fn parse_piece(fen: char) -> Option<Piece> {
    match fen {
        'P' => Some(Piece {t: PieceType::Pawn, color: Color::White}),
        'N' => Some(Piece {t: PieceType::Knight, color: Color::White}),
        'B' => Some(Piece {t: PieceType::Bishop, color: Color::White}),
        'R' => Some(Piece {t: PieceType::Rook, color: Color::White}),
        'Q' => Some(Piece {t: PieceType::Queen, color: Color::White}),
        'K' => Some(Piece {t: PieceType::King, color: Color::White}),
        'p' => Some(Piece {t: PieceType::Pawn, color: Color::Black}),
        'n' => Some(Piece {t: PieceType::Knight, color: Color::Black}),
        'b' => Some(Piece {t: PieceType::Bishop, color: Color::Black}),
        'r' => Some(Piece {t: PieceType::Rook, color: Color::Black}),
        'q' => Some(Piece {t: PieceType::Queen, color: Color::Black}),
        'k' => Some(Piece {t: PieceType::King, color: Color::Black}),
        _ => None
    }
}

fn parse_castling_piece(fen: char) -> Option<Piece> {
    match fen {
        'Q' => Some(Piece {t: PieceType::Queen, color: Color::White}),
        'K' => Some(Piece {t: PieceType::King, color: Color::White}),
        'q' => Some(Piece {t: PieceType::Queen, color: Color::Black}),
        'k' => Some(Piece {t: PieceType::King, color: Color::Black}),
        _ => None
    }
}

fn parse_square(fen: &str) -> Option<Position> {
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
        8 - value
    } else {
        return None;
    };

    return Some(Position::new(row, column))
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.promotion {
            Some(p) => write!(f, "{}{}{}", self.origin, self.destination, p),
            None => write!(f, "{}{}", self.origin, self.destination)
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (b'a' + self.column as u8) as char, 8 - self.row)
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = self.t.to_string();

        match self.color {
            Color::White => write!(f, "{}", t),
            Color::Black => write!(f, "{}", t.to_uppercase())
        }
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
