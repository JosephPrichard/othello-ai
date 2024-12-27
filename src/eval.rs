/*
 * Copyright (c) Joseph Prichard 2022.
 */

use crate::board::{BLACK, OthelloBoard, WHITE};
use crate::tile::{Tile, TILES};

const CORNERS: [[i8; 2]; 4] = [[0, 0], [0, 7], [7, 0], [7, 7]];
const XC_SQUARES: [[i8; 2]; 12] = [
    [1, 1], [1, 6], [6, 1], [6, 6], [0, 1], [0, 6],
    [7, 1], [7, 6], [1, 0], [1, 7], [6, 0], [6, 7],
];

fn find_parity_heuristic(board: &OthelloBoard) -> f32 {
    let mut white_score = 0f32;
    let mut black_score = 0f32;
    for tile in TILES {
        let color = board.get_tile(tile);
        if color == WHITE {
            white_score += 1f32;
        }
        if color == BLACK {
            black_score += 1f32;
        }
    }
    (black_score - white_score) / (black_score + white_score)
}

fn find_corner_heuristic(board: &OthelloBoard) -> f32 {
    let mut white_corners = 0f32;
    let mut black_corners = 0f32;
    for corner in CORNERS {
        let color = board.get_tile(Tile::new(corner[0], corner[1]));
        if color == WHITE {
            white_corners += 1f32;
        }
        if color == BLACK {
            black_corners += 1f32;
        }
    }
    if black_corners + white_corners != 0f32 {
        (black_corners - white_corners) / (black_corners + white_corners)
    } else {
        0f32
    }
}

fn find_xc_square_heuristic(board: &OthelloBoard) -> f32 {
    let mut white_squares = 0f32;
    let mut black_squares = 0f32;
    for square in XC_SQUARES {
        let color = board.get_tile(Tile::new(square[0], square[1]));
        if color == WHITE {
            white_squares += 1f32;
        }
        if color == BLACK {
            black_squares += 1f32;
        }
    }
    if white_squares + black_squares != 0f32 {
        // having more x or c squares is bad
        (white_squares - black_squares) / (black_squares + white_squares)
    } else {
        0f32
    }
}

fn find_mobility_heuristic(board: &OthelloBoard) -> f32 {
    let white_moves = board.count_potential_moves(WHITE) as f32;
    let black_moves = board.count_potential_moves(BLACK) as f32;
    if white_moves + black_moves == 0f32 {
        (black_moves - white_moves) / (black_moves + white_moves)
    } else {
        0f32
    }
}

fn find_stability_heuristic(board: &OthelloBoard) -> f32 {
    0f32
}

pub fn find_heuristic(board: &OthelloBoard) -> f32 {
    50f32 * find_parity_heuristic(board)
        + 100f32 * find_corner_heuristic(board)
        + 100f32 * find_mobility_heuristic(board)
        + 50f32 * find_xc_square_heuristic(board)
        + 100f32 * find_stability_heuristic(board)
}