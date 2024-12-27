/*
 * Copyright (c) Joseph Prichard 2022.
 */

use std::time::SystemTime;

use smallvec::SmallVec;
use crate::board::OthelloBoard;
use crate::eval;
use crate::hasher::ZHasher;
use crate::cache::{CacheNode, TranspositionTable};
use crate::profile::{Profiler, Run};
use crate::tile::RankedTile;

#[derive(Copy, Clone)]
pub struct AgentConfig {
    max_search_depth: u32,
}

impl AgentConfig {
    pub fn new(max_search_depth: u32) -> Self {
        Self { max_search_depth }
    }
}

pub struct OthelloAgent {
    hasher: ZHasher,
    config: AgentConfig,
    pub cache: TranspositionTable,
    pub profiler: Profiler,
}

impl OthelloAgent {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            config,
            hasher: ZHasher::new(),
            cache: TranspositionTable::new(),
            profiler: Profiler::new(),
        }
    }

    pub fn add_run(&mut self, time_taken: u128) {
        let run = Run::new(
            self.config.max_search_depth, self.cache.hits(),
            self.cache.misses(), time_taken
        );
        self.profiler.add_run(run);
    }

    pub fn find_best_move(&mut self, board: &OthelloBoard) -> Option<RankedTile> {
        let start_time = SystemTime::now();
        self.cache.reset_counts();

        let mut best_move = None;
        let mut best_heuristic = if board.black_move { f32::MIN } else { f32::MAX };

        // call the iterative deepening minimax to calculate the heuristic for each potential move and determine the best one
        board.find_current_moves(|mov| {
            // get the child board for the move and check if it is better than the last one
            let child = board.make_move(mov);
            let heuristic = self.evaluate_base(&child);
            // compare the move to make sure we get the best one
            if board.black_move {
                if heuristic > best_heuristic {
                    best_move = Some(mov);
                    best_heuristic = heuristic;
                }
            } else {
                if heuristic < best_heuristic {
                    best_move = Some(mov);
                    best_heuristic = heuristic;
                }
            }
        });

        let time_taken = SystemTime::now().duration_since(start_time).unwrap().as_millis();
        self.add_run(time_taken);

        RankedTile::from_option(best_move, best_heuristic)
    }

    pub fn find_ranked_moves(&mut self, board: &OthelloBoard) -> Vec<RankedTile> {
        let start_time = SystemTime::now();
        self.cache.reset_counts();

        let mut ranked_tiles = vec![];
        // call the iterative deepening minimax to calculate the heuristic for each potential move
        board.find_current_moves(|mov| {
            // get the child board for the move and check if it is better than the last one
            let child = board.make_move(mov);
            let heuristic = self.evaluate_base(&child);
            ranked_tiles.push(RankedTile::new(mov, heuristic))
        });

        if board.black_move {
            ranked_tiles.sort_by(|a, b| {
                a.heuristic.total_cmp(&b.heuristic)
            });
        } else {
            ranked_tiles.sort_by(|a, b| {
                b.heuristic.total_cmp(&a.heuristic)
            });
        }

        let time_taken = SystemTime::now().duration_since(start_time).unwrap().as_millis();
        self.add_run(time_taken);

        ranked_tiles
    }

    fn evaluate_base(&mut self, board: &OthelloBoard) -> f32 {
        let mut heuristic = 0f32;
        for depth_limit in 1..self.config.max_search_depth - 1 {
            heuristic = self.evaluate(*board, depth_limit, board.black_move, f32::MIN, f32::MAX);
        }
        heuristic
    }

    fn evaluate(&mut self, board: OthelloBoard, depth: u32, maximizer: bool, mut alpha: f32, mut beta: f32) -> f32 {
        // stop when we reach depth floor
        if depth == 0 {
            return eval::find_heuristic(&board);
        }

        // create then populate a vec of children for each move
        let mut children = SmallVec::<[OthelloBoard; 16]>::new();
        board.find_current_moves(|mov| {
            // get the child board for the move and add it to children
            let child = board.make_move(mov);
            children.push(child);
        });

        // cannot expand node's children
        if children.len() == 0 {
            return eval::find_heuristic(&board);
        }

        // check transposition table to see if we have a cache hit
        let hash_key = self.hasher.hash(&board);
        if let Some(node) = self.cache.get(hash_key) {
            if node.depth >= depth {
                return node.heuristic;
            }
        }

        if maximizer {
            // explore best children first for move ordering, find the best moves and return them
            for child in children {
                alpha = alpha.max(self.evaluate(child, depth - 1, false, alpha, beta));
                // prune this branch, it cannot possibly be better than any child found so far
                if alpha >= beta {
                    break;
                }
            }
            let node = CacheNode::new(hash_key, alpha, depth);
            self.cache.put(node);
            alpha
        } else {
            // explore best children first for move ordering, find the best moves and return them
            for child in children {
                beta = beta.min(self.evaluate(child, depth - 1, true, alpha, beta));
                // prune this branch, it cannot possibly be better than any child found so far
                if beta <= alpha {
                    break;
                }
            }
            let node = CacheNode::new(hash_key, beta, depth);
            self.cache.put(node);
            beta
        }
    }
}