/*
 * Copyright (c) Joseph Prichard 2022.
 */

use std::process::exit;
use std::sync::LazyLock;
use crate::agent::{AgentConfig, OthelloAgent};
use crate::board::OthelloBoard;
use crate::errors::{ParseResult, ParseError};
use crate::tile::Tile;

const MAX_LEVEL: usize = 6;

pub struct CommandHandler {
    agents: Vec<Option<OthelloAgent>>,
    configs: Vec<AgentConfig>,
    current_board: OthelloBoard,
}

impl CommandHandler  {
    pub fn new() -> Self {
        let mut agents = Vec::new();
        for _ in 0..MAX_LEVEL {
            agents.push(None);
        }

        let configs = vec![
            AgentConfig::new(2),
            AgentConfig::new(3),
            AgentConfig::new(5),
            AgentConfig::new(7),
            AgentConfig::new(10),
            AgentConfig::new(15),
        ];
        Self { agents, configs, current_board: OthelloBoard::new() }
    }

    fn get_optional_agent(&mut self, valid_level: usize) -> &mut Option<OthelloAgent>  {
        self.agents.get_mut(valid_level - 1)
            .expect(&format!("Fatal error: couldn't access agent Level {}", valid_level))
    }

    // function that will lazily generate agents only when needed
    fn get_agent(&mut self, valid_level: usize) -> &mut OthelloAgent {
        let config = self.configs[valid_level - 1];
        // get the agent for the validated level
        let agent = self.get_optional_agent(valid_level);
        match agent {
            None => {
                // create an agent and return a mutable reference to it if none exists
                *agent = Some(OthelloAgent::new(config));
                agent.as_mut().unwrap()
            }
            // just return a mutable reference to the agent if it exists
            Some(agent) => agent
        }
    }

    pub fn handle_line(&mut self, line: &str) {
        // handle the command and write back the data
        match self.handle_command(line) {
            Ok(result) => println!("{}", result),
            Err(err) =>  println!("error {}", err.to_string())
        }
    }

    fn handle_command(&mut self, command_str: &str) -> ParseResult<String> {
        let tokens = command_str.split(" ").collect::<Vec<&str>>();
 
        if tokens.is_empty() {
            return Err(ParseError::new("Must contain command name"))
        }
        let name = tokens[0];
        let args = &tokens[1..tokens.len()];
        let result = match name {
            "quit" => Self::handle_quit(),
            "view" => self.handle_view(),
            "move" => self.handle_move(args)?,
            "moves" => self.handle_moves(args)?,
            "profile" => self.handle_profile(args)?,
            "best" => self.handle_best_command(args)?,
            "ranked" => self.handle_ranked_command(args)?,
            _ => {
                return Err(ParseError::new("Unknown command name"))
            }
        };
        Ok(result)
    }

    fn handle_quit() -> ! {
        eprintln!("Quit engine");
        exit(1)
    }

    fn handle_view(&self) -> String {
        self.current_board.to_notation()
    }

    fn handle_move(&mut self, args: &[&str]) -> ParseResult<String> {
        if args.len() < 1 {
            return Err(ParseError::new("Needs at least 1 args"))
        }

        let mov = Tile::from_str(args[0])?;
        let (board, using_curr) = match args.get(1) {
            Some(str) => (OthelloBoard::from_notation(str)?, false),
            None => (self.current_board, true), // copy out for safety
        };
        
        // check if the tile is a valid move or not
        if !board.find_current_moves_as_vec().contains(&mov) {
            return Err(ParseError::new("Not a valid move"))
        }

        let new_board = board.make_move(mov);
        if using_curr {
            self.current_board = new_board
        }

        let result = format!("tile {}", new_board.to_notation());
        Ok(result)
    }

    fn handle_moves(&self, args: &[&str]) -> ParseResult<String> {
        let board = match args.get(0) {
            Some(str) => OthelloBoard::from_notation(str)?,
            None => self.current_board, // copy out for convenience
        };

        // construct a moves output as a space-sep string
        let mut moves_str = String::from("moves ");
        board.find_current_moves(|mov| {
            moves_str.push_str(&mov.to_string());
            moves_str.push(' ')
        });
        Ok(moves_str)
    }

    fn parse_level(level_str: &str) -> ParseResult<usize> {
        let level = match level_str.parse::<usize>() {
            Ok(level) => level,
            Err(..) => {
                return Err(ParseError::new("Level must be an integer"))
            }
        };
        if level < 1 || level > MAX_LEVEL {
            static ERR_MSG: LazyLock<String> = std::sync::LazyLock::new(|| format!("Level must be between 1 and {}", MAX_LEVEL));
            return Err(ParseError::new(ERR_MSG.as_str()))
        }
        Ok(level)
    }

    fn handle_profile(&mut self, args: &[&str]) -> ParseResult<String> {
        if args.len() < 2 {
            return Err(ParseError::new("Needs at least 2 args"))
        }
        let level = Self::parse_level(args[1])?;
        match args[0] {
            "log" => {
                let agent = self.get_agent(level);
                eprintln!("Logging runs for agent Level {}", level);
                agent.profiler.log_runs();
                Ok(String::from("Logged runs data to stderr"))
            },
            "dump" => {
                let agent = self.get_agent(level);
                eprintln!("Dumping cache data for agent Level {}", level);
                agent.cache.dump();
                Ok(String::from("Dumped cache data to stderr"))
            },
            "drop" => {
                *self.get_optional_agent(level) = None;
                Ok(String::from(&format!("Dropped agent the Level {}", level)))
            }
            _ => Err(ParseError::new("Profile flag must be dump or drop"))
        }
    }

    fn extract_agent_args(&self, args: &[&str]) -> ParseResult<(usize, OthelloBoard)> {
        if args.len() < 1 {
            return Err(ParseError::new("Needs at least 1 args"))
        }

        let level = Self::parse_level(args[0])?;
        let board = match args.get(1) {
            Some(str) => OthelloBoard::from_notation(str)?,
            None => self.current_board, // copy out for convenience
        };
        Ok((level, board))
    }

    fn handle_best_command(&mut self, args: &[&str]) -> ParseResult<String> {
        let (level, board) = self.extract_agent_args(args)?;

        let best_tile = self.get_agent(level).find_best_move(&board);
        let result = match best_tile {
            Some(tile) => format!("tile {}", tile.to_string()),
            None => String::from("notile"),
        };
        Ok(result)
    }

    fn handle_ranked_command(&mut self, args: &[&str]) -> ParseResult<String> {
        let (level, board) = self.extract_agent_args(args)?;
        
        let ranked_tiles = self.get_agent(level).find_ranked_moves(&board);
       
        // add the ranked tiles to a space-sep string as a response
        let mut tiles_str = String::from("tiles ");
        for r in ranked_tiles.iter() {
            tiles_str.push_str(&r.tile.to_string());
            tiles_str.push(' ');
        }
        Ok(tiles_str)
    }
}