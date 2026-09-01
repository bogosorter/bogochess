use crate::core::model::*;
use std::fmt::Display;

pub const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse(fen: &str) -> Option<GameState> {
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

    Some(GameState {
        board,
        ended: false,
        current_player: active_color,
        castling,
        en_passant,
        halfmoves
    })
}

pub fn parse_with_moves(fen: &str, moves: &[&str]) -> Option<GameState> {
    let mut game_state = parse(fen)?;

    for s in moves {
        let m = game_state.pseudo_legal_moves().into_iter().find(|m| m.to_string() == *s)?;
        game_state.apply(&m);
    }

    Some(game_state)
}

fn parse_board(fen: &str) -> Option<Board> {
    let mut position = Square::new(56);
    let mut board = Board::new();

    for c in fen.chars() {
        if c.is_numeric() {
            position = position.shift(c.to_digit(10).unwrap() as i8);
        } else if c == '/' {
            position = position.shift(-16);
        } else {
            if !parse_piece(&mut board, position, c) {
                return None
            }
            position = position.shift(1);
        }
    }

    Some(board)
}

fn parse_piece(board: &mut Board, square: Square, fen: char) -> bool {
    if fen.is_uppercase() {
        board.colors[0] |= square.bitboard();
    } else {
        board.colors[1] |= square.bitboard();
    }

    match fen.to_ascii_uppercase() {
        'P' => board.pieces[PieceType::Pawn] |= square.bitboard(),
        'N' => board.pieces[PieceType::Knight] |= square.bitboard(),
        'B' => board.pieces[PieceType::Bishop] |= square.bitboard(),
        'R' => board.pieces[PieceType::Rook] |= square.bitboard(),
        'Q' => board.pieces[PieceType::Queen] |= square.bitboard(),
        'K' => board.pieces[PieceType::King] |= square.bitboard(),
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

fn parse_square(fen: &str) -> Option<Square> {
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

    return Some(Square::new((row * 8 + column) as usize))
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.promoted() {
            Some(p) => write!(f, "{}{}{}", self.origin(), self.destination(), p),
            None => write!(f, "{}{}", self.origin(), self.destination())
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (b'a' + self.column() as u8) as char, self.row() + 1)
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
