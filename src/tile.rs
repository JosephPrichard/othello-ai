/*
 * Copyright (c) Joseph Prichard 2022.
 */

use std::fmt;
use crate::errors::{ParseError, ParseResult};

#[derive(Clone, Copy, PartialEq)]
pub struct Tile {
    pub row: i8,
    pub col: i8,
}

impl Tile {
    pub const fn new(row: i8, col: i8) -> Self {
        Self { row, col }
    }

    pub const fn from_index(index: usize) -> Self {
        let row = (index / 8) as i8;
        let col = (index % 8) as i8;
        Self { row, col }
    }

    pub fn from_str(str: &str) -> ParseResult<Self> {
        // check if the tile is the right size
        if str.len() != 2 {
            return Err(ParseError::new("Tile notation must be 2 characters long"))
        }
        let mut chars = str.chars();
        let c1 = chars.next().unwrap_or_default();
        let c2 = chars.next().unwrap_or_default();
        // convert first char into column and convert second char into row
        let col = ((c1 as u8) - ('a' as u8)) as i8;
        let row = c2.to_digit(10).unwrap_or_default() as i8 - 1;
        // check if the each char is within the acceptable range
        if row < 0 || col < 0 || row > 7 || col > 7 {
            return Err(ParseError::new("Tile row col pair must be between a1 and h7"))
        }
        Ok(Self { row, col })
    }

    pub fn in_bounds(&self) -> bool {
        self.row >= 0 && self.col >= 0 && self.row < 8 && self.col < 8
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = (self.col as u8 + ('a' as u8)) as char;
        let r = (self.row + 1).to_string();
        write!(f, "{}{}", c, r)
    }
}

#[derive(Clone, Copy)]
pub struct RankedTile {
    pub tile: Tile,
    pub heuristic: f32,
}

impl RankedTile {
    pub fn new(tile: Tile, heuristic: f32) -> Self {
        Self { tile, heuristic }
    }

    pub fn from_option(tile: Option<Tile>, heuristic: f32) -> Option<Self> {
        if let Some(t) = tile {
            Some(Self::new(t, heuristic))
        } else {
            None
        }
    }
}

impl fmt::Display for RankedTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Disc: {}, Heuristic: {}", self.tile, self.heuristic)
    }
}

pub static TILES: [Tile; 64] = tiles();

pub const fn tiles() -> [Tile; 64] {
    let mut tiles = [Tile { row: 0, col: 0 }; 64];
    let mut i = 0;
    while i < 64 {
        tiles[i] = Tile::from_index(i);
        i += 1;
    }
    tiles
}