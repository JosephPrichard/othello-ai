/*
 * Copyright (c) Joseph Prichard 2022.
 */

use rand::Rng;
use crate::board::OthelloBoard;
use crate::tile::Tile;

pub struct ZHasher {
    table: [[i64; 3]; 64],
}

impl ZHasher {
    pub fn new() -> Self {
        let mut generator = rand::thread_rng();
        let mut hasher = Self {
            table: [[0; 3]; 64]
        };
        for i in 0..64 {
            for j in 0..3 {
                let n = generator.gen_range(i64::MIN..i64::MAX);
                hasher.table[i][j] = if n >= 0 { n } else { -n };
            }
        }
        hasher
    }

    pub fn hash(&self, board: &OthelloBoard) -> i64 {
        let mut hash = 0i64;
        for i in 0..self.table.len() {
            let t = Tile::from_index(i);
            hash = hash ^ self.table[i][board.get_tile(t) as usize];
        }
        hash
    }
}