# OthelloAI
Minimax implementation of an Othello AI.

This is a rewrite of my Java OthelloBot engine for the purpose of learning Rust.

## CLI

Send input through STDIN pipe and recv output through STDOUT pipe. Any logging or errors are sent through STDERR.

Moves are given in the standard letter + number othello noitation.
ex: `a5` means col 1, row 5.

Boards are given in a format similar to FEN.
ex: `8E/8E/8E/3EBW3E/3EWB3E/8E/8E/8E/B` would be the start state for a given othello board.

Board arguments are optional and default to using a global board if not provided.

`$ quit`

Close the bot and wipe any active state/caches.

`$ move <move> <board?>`

Make a move on the board with the given move.

`$ view`

View the current board state that can be interacted with using defaulted commands.

`$ moves <board?>`

Retrieve the legal moves on the board.

`$ profile log <level>`

View logs for the engine operations that have been run for a given agent level.

`$ profile dump <level>`

View the current state of the cache for a given agent leel.

`$ profile drop <level>`

Drop all caches and telemetries for a given engine level.

`$ best <level> <board?>`

Find the "best" move that can be made for the board according to the engine at a given level.

`$ ranked <level> <board?>`

Find all legal moves for the board and rank them by how "good" the engine at a given level thinks they are.