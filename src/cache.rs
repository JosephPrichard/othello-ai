/*
 * Copyright (c) Joseph Prichard 2022.
 */

use std::mem;

const CACHE_SIZE: usize = (2i32.pow(12) + 1) as usize;

type CacheLine = [Option<CacheNode>; 2];
type Cache = [CacheLine; CACHE_SIZE];

#[derive(Copy, Clone)]
pub struct CacheNode {
    pub key: i64,
    pub heuristic: f32,
    pub depth: u32,
}

impl CacheNode {
    pub fn new(key: i64, heuristic: f32, depth: u32) -> Self {
        Self { key, heuristic, depth }
    }
}

pub struct TranspositionTable {
    cache: Box<Cache>,
    hits: u32,
    misses: u32,
}

impl TranspositionTable {
    pub fn new() -> Self {
        eprintln!("Cache size: {} bytes", mem::size_of::<Cache>());
        // each cache line has 2 elements, one being "replace by depth" and one being "replace always"
        Self {
            cache: Box::new([[None; 2]; CACHE_SIZE]),
            hits: 0,
            misses: 0,
        }
    }

    pub fn cache_len(&self) -> i64 {
        self.cache.len() as i64
    }

    pub fn put(&mut self, node: CacheNode) {
        let some_node = Some(node);
        let h = node.key % self.cache_len();
        // retrieve cache line
        let cache_line = &mut self.cache[h as usize];
        // check if "replace by depth" is populated
        if let Some(first_node) = cache_line[0] {
            // populated, new node is better so we do replacement
            if node.depth > first_node.depth {
                cache_line[1] = Some(first_node);
                cache_line[0] = some_node;
            } else {
                // new node is worse so it should be sent to "replace always"
                cache_line[1] = some_node;
            }
        } else {
            // not populated, so it can be used
            cache_line[0] = some_node;
        }
    }

    pub fn get(&mut self, key: i64) -> Option<&CacheNode> {
        let h = key % self.cache_len();
        // retrieve cache line
        let cache_line = &self.cache[h as usize];
        // iterate through cache line
        for opt_node in cache_line {
            // if node is in cache line return it
            if let Some(node) = opt_node {
                if node.key == key {
                    self.hits += 1;
                    return Some(node);
                }
            }
        }
        self.misses += 1;
        None
    }

    pub fn clear(&mut self) {
        for cache_line in self.cache.iter_mut() {
             *cache_line = [None; 2];
        }
    }

    pub fn dump(&self) {
        eprintln!("Debug Cache");
        for cache_line in self.cache.iter() {
            let dump_str = &mut String::new();
            match &cache_line[0] {
                Some(node) => {
                    dump_str.push_str(&format!("Slot1 {} {} {} ", node.key, node.heuristic, node.depth))
                },
                None => dump_str.push_str("Slot1 Empty ")
            };
            match &cache_line[1] {
                Some(node) => {
                    dump_str.push_str(&format!("Slot2 {} {} {}", node.key, node.heuristic, node.depth))
                },
                None => dump_str.push_str("Slot2 Empty")
            };
            eprintln!("{}", dump_str);
        }
    }

    pub fn hits(&self) -> u32 {
        self.hits
    }

    pub fn misses(&self) -> u32 {
        self.misses
    }

    pub fn reset_counts(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }
}