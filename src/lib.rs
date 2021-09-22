use serde::{Deserialize};
use std::{error::Error, fmt::Display, convert::From};
use std::fmt;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

#[derive(Debug)]
pub enum FenError {
    UnexpectedChessChar(char),
}

impl Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UnexpectedChessChar(the_char) => write!(f, "{} is not a valid chess character", the_char),
        }
    }
}

impl Error for FenError {}

lazy_static! {
    static ref PIECEMAP: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('♙', 'P');
        m.insert('♖', 'R');
        m.insert('♘', 'N');
        m.insert('♗', 'B');
        m.insert('♕', 'Q');
        m.insert('♔', 'K');
        m.insert('♟', 'p');
        m.insert('♜', 'r');
        m.insert('♞', 'n');
        m.insert('♝', 'b');
        m.insert('♛', 'q');
        m.insert('♚', 'k');
        m.insert(' ', ' ');
        m
    };
}

#[derive(Deserialize)]
struct Editor {
    white_to_start: bool,
    white_king_side_castle: bool,
    white_queen_side_castle: bool,
    black_king_side_castle: bool,
    black_queen_side_castle: bool,
    squares: Vec<char>
}

struct Fen {
    squares: String,
    white_to_start: bool,
    white_king_side_castle: bool,
    white_queen_side_castle: bool,
    black_king_side_castle: bool,
    black_queen_side_castle: bool,
}

impl Fen {
    fn editor_chars_to_fen_squares(editor: &Editor) -> String {
        let fen_chars_result: Result<Vec<char>, _> = editor.squares
            .iter()
            .map(|chess_char| Self::chess_char_to_fen_char(chess_char))
            .collect();
        let fen_chars = fen_chars_result.expect("Error getting fen characters");
        let fen_ranks: Vec<String> = fen_chars.chunks(8)
            .map(|fen_rank| Self::abbreviated_fen_rank(fen_rank) )
            .collect();
        fen_ranks.join("/")
    }

    fn chess_char_to_fen_char(chess_char: &char) -> Result<char, Box<dyn Error>> {
        PIECEMAP.get(chess_char).map(|x|x.clone()).ok_or(Box::new(FenError::UnexpectedChessChar(*chess_char)) as Box<dyn Error + Send>) 
    }
    
    fn abbreviated_fen_rank(rank: &[char]) -> String {
        let mut abb_fen_rank = String::new();
        let mut empty_square_count = 0;
        for fen_char in rank {
            if *fen_char == ' ' {
                empty_square_count += 1;
            } else {
                if empty_square_count > 0 {
                    abb_fen_rank.push(std::char::from_digit(empty_square_count, 10).unwrap());
                    empty_square_count = 0;
                }
                abb_fen_rank.push(fen_char.clone());
            }
        }
    
        if empty_square_count > 0 {
            abb_fen_rank.push(std::char::from_digit(empty_square_count, 10).unwrap());
        }
    
        abb_fen_rank
    }

    fn starting_color(&self) -> String {
        if self.white_to_start {String::from("w")} else {String::from("b")} 
    }

    fn castling_rights(&self) -> String {
        let mut rights = String::new();
        if self.white_king_side_castle {rights.push('K')}
        if self.white_queen_side_castle {rights.push('Q')}
        if self.black_king_side_castle {rights.push('k')}
        if self.black_queen_side_castle {rights.push('q')}
        rights
    }
}

impl From<Editor> for Fen {
    fn from(editor: Editor) -> Self {
        Self {
            squares: Self::editor_chars_to_fen_squares(&editor),
            white_to_start: editor.white_to_start,
            white_king_side_castle: editor.white_king_side_castle,
            white_queen_side_castle: editor.white_queen_side_castle,
            black_king_side_castle: editor.black_king_side_castle,
            black_queen_side_castle: editor.black_queen_side_castle
        }
    }
}

impl Display for Fen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} - 0 1", self.squares, self.starting_color(), self.castling_rights())
    }
}

fn get_fen(editor_str: &str) -> Result<String, Box<dyn Error>> {
    let editor: Editor = serde_json::from_str(editor_str)?;
    let fen: Fen = editor.into();
    Ok(fen.to_string())
}

#[wasm_bindgen]
pub fn get_fen_wasm(editor_json: &str) -> String {
    get_fen(editor_json).expect("Conversion to fen failed")
}