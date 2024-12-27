/*
 * Copyright (c) Joseph Prichard 2022.
 */

use std::io;
use std::io::BufRead;
use crate::command::CommandHandler;

mod board;
mod agent;
mod tile;
mod cache;
mod hasher;
mod eval;
mod profile;
mod command;
mod errors;

pub fn main() {
    eprintln!("Started the engine");

    let mut handler = CommandHandler::new();
    for line in io::stdin().lock().lines() {
        match line {
            Ok(line) => handler.handle_line(&line),
            Err(err) => {
                eprintln!("Error occurred while accepting stdin {}", err)
            }
        }
    }
}
