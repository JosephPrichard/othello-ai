/*
 * Copyright (c) Joseph Prichard 2022.
 */

use std::fmt;
use crate::errors::{ParseResult, ParseError};
use crate::tile::{Tile, TILES};

pub const EMPTY: u8 = 0;
pub const WHITE: u8 = 1;
pub const BLACK: u8 = 2;
const DIRECTIONS: [[i8; 2]; 8] = [[0, 1], [0, -1], [1, 0], [-1, 0], [-1, -1], [-1, 1], [1, -1], [1, 1]];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OthelloBoard {
    board: i128,
    pub black_move: bool,
}

impl OthelloBoard {
    pub fn new() -> Self {
        let mut board = Self { board: 0, black_move: true };
        board.set_tile(Tile::new(3, 3), WHITE);
        board.set_tile(Tile::new(3, 4), BLACK);
        board.set_tile(Tile::new(4, 3), BLACK);
        board.set_tile(Tile::new(4, 4), WHITE);
        board
    }

    pub fn set_tile(&mut self, tile: Tile, color: u8) {
        let p = (tile.row * 8 + tile.col) * 2;
        let clear_mask = !(1 << p) & !(1 << (p + 1));
        self.board &= clear_mask;
        self.board |= (color as i128) << p;
    }

    pub fn get_tile(&self, tile: Tile) -> u8 {
        let mask = (1 << 2) - 1;
        let p = (tile.row * 8 + tile.col) * 2;
        (mask & (self.board >> p)) as u8
    }

    pub fn find_current_moves(&self, on_move: impl FnMut(Tile)) {
        let color = if self.black_move { BLACK } else { WHITE };
        self.find_potential_moves(color, on_move)
    }

    pub fn find_potential_moves(&self, color: u8, mut on_move: impl FnMut(Tile)) {
        let opposite_color = if color == BLACK { WHITE } else { BLACK };

        // check each disc for potential flanks
        for disc in TILES.into_iter() {
            // skip if the color does not match
            if self.get_tile(disc) != color {
                continue;
            }
            // check each direction from disc for potential flank
            for direction in DIRECTIONS {
                let mut tile = Tile::new(disc.row + direction[0], disc.col + direction[1]);

                // iterate from disc to next opposite color
                let mut count = 0;
                while tile.in_bounds() {
                    if self.get_tile(tile) != opposite_color {
                        break;
                    }
                    tile.row += direction[0];
                    tile.col += direction[1];
                    count += 1;
                }
                // add move to potential moves list assuming
                // we flank at least once disc, the tile is in bounds and is empty
                if count > 0 && tile.in_bounds() && self.get_tile(tile) == EMPTY {
                    // invoke move event
                    on_move(tile);
                }
            }
        }
    }

    pub fn make_move(&self, mov: Tile) -> OthelloBoard {
        // copies the current board to a new child board
        let mut board = *self;

        let opposite_color = if board.black_move { WHITE } else { BLACK };
        let current_color = if board.black_move { BLACK } else { WHITE };

        board.black_move = !board.black_move;
        board.set_tile(mov, current_color);

        // check each direction of new disc position
        for direction in DIRECTIONS {
            let initial_tile = Tile::new(mov.row + direction[0], mov.col + direction[1]);
            let mut tile = Tile::new(initial_tile.row, initial_tile.col);

            let mut flank = false;

            // iterate from disc until first potential flank
            while tile.in_bounds() {
                if board.get_tile(tile) == current_color {
                    flank = true;
                    break;
                } else if board.get_tile(tile) == EMPTY {
                    break;
                }
                tile.row += direction[0];
                tile.col += direction[1];
            }

            if !flank {
                continue;
            }

            tile.row = initial_tile.row;
            tile.col = initial_tile.col;

            // flip each disc to opposite color to flank, update disc counts
            while tile.in_bounds() {
                if board.get_tile(tile) != opposite_color {
                    break;
                }

                board.set_tile(tile, current_color);

                tile.row += direction[0];
                tile.col += direction[1];
            }
        }

        board
    }

    pub fn find_current_moves_as_vec(&self) -> Vec<Tile> {
        let mut moves = vec![];
        self.find_current_moves(|mov| {
            moves.push(mov)
        });
        moves
    }

    pub fn count_potential_moves(&self, color: u8) -> usize {
        let mut count = 0;
        self.find_potential_moves(color, |_| count += 1);
        count
    }

    pub fn get_symbol(&self, tile: Tile) -> char {
        match self.get_tile(tile) {
            1 => 'B',
            2 => 'W',
            _ => 'E'
        }
    }

    pub fn set_symbol(&mut self, tile: Tile, sym: char) -> ParseResult<()> {
        let byte = match sym {
            'E' => 0u8,
            'B' => 1u8,
            'W' => 2u8,
            _ => {
                return Err(ParseError::new("Tile symbol must be E, B or W"))
            }
        };
        self.set_tile(tile, byte);
        Ok(())
    }

    pub fn set_turn(&mut self, sym: char) -> ParseResult<()> {
        self.black_move = match sym {
            'B' => true,
            'W' => false,
            _ => {
                return Err(ParseError::new("Turn must be B or W"))
            }
        };
        Ok(())
    }

    pub fn from_notation(str: &str) -> ParseResult<Self> {
        let mut board = OthelloBoard::new();
        let mut row = 0;
        let mut col = 0;
        let mut count = 1;
        for c in str.chars() {
            if row > 7 {
                board.set_turn(c)?;
                break;
            }
            if c == '/' {
                // slash means we need to go to the next row
                row += 1;
                col = 0;
            } else {
                match c.to_digit(10) {
                    // digit indicates the count of the next sym to output
                    Some(digit) => count = digit as i8,
                    // otherwise output the current sym, count number of times and reset the count
                    None => {
                        if count + col > 8 {
                            return Err(ParseError::new("Cannot have more than 8 cols per row"))
                        }
                        // write the counted number of symbols and go to the next col
                        for _ in 0..count {
                            board.set_symbol(Tile::new(row, col), c)?;
                            col += 1;
                        }
                        count = 1;
                    }
                }
            }
        }
        Ok(board)
    }

    pub fn to_notation(&self) -> String {
        let mut tiles_str = String::with_capacity(66);
        let mut count = 0;
        let mut sym = self.get_symbol(Tile::from_index(0));
        // iterating over each tile
        for tile in TILES {
            let new_sym = self.get_symbol(tile);
            if new_sym != sym {
                // add the counted chars
                if count > 1 {
                    tiles_str.push_str(&count.to_string());
                }
                if count > 0 {
                    tiles_str.push(sym);
                }
                // reset the count with the new sym
                sym = new_sym;
                count = 1;
            } else {
                // encountered same symbol, so increment count
                count += 1;
            }
            // at the end of the row
            if tile.col == 7 {
                // add the counted chars
                if count > 1 {
                    tiles_str.push_str(&count.to_string());
                }
                if count > 0 {
                    tiles_str.push(sym);
                }
                // reset the count with the new sym
                sym = new_sym;
                count = 0;
                // add the slash at the end of the row
                tiles_str.push('/');
            }
        }
        tiles_str.push(if self.black_move { 'B' } else { 'W' });
        tiles_str
    }
}

impl fmt::Display for OthelloBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_str = String::from("");
        // add space for better board indentation
        board_str.push_str("  ");
        // add each column header as letter
        for i in 0u8..8 {
            board_str.push((('a' as u8) + i) as char);
            board_str.push(' ');
        }
        board_str.push('\n');
        // add each matrix element in board with row header
        for row in 0..8 {
            board_str.push_str(&(row + 1).to_string());
            board_str.push(' ');
            for col in 0..8 {
                board_str.push_str(&self.get_tile(Tile::new(row, col)).to_string());
                board_str.push(' ');
            }
            board_str.push('\n');
        }
        write!(f, "{}", board_str)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::OthelloBoard;

    #[test]
    fn test_to_notation() {
        let board = OthelloBoard { board: 1495472766589663741892773636151968256, black_move: true };
        let notation = "4EW3E/3EWBW2E/BE5WE/E2B3W2E/2E2BW3E/E2BWB3E/3EWEB2E/2EWEB3E/B";
        let other_notation = board.to_notation();

        assert_eq!(notation, other_notation);
    }

    #[test]
    fn test_from_notation() {
        let board = OthelloBoard { board: 1495472766589663741892773636151968256, black_move: true };
        let notation = "4EW3E/3EWBW2E/BE5WE/E2B3W2E/2E2BW3E/E2BWB3E/3EWEB2E/2EWEB3E/B";
        let other_board = OthelloBoard::from_notation(&notation).unwrap();

        eprintln!("{}\n{}", board, other_board);

        assert_eq!(board, other_board);
    }
}