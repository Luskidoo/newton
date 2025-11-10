use crate::board::Board;
use crate::defs::{FEN_START_POSITION, MAX_DEPTH};
use crate::movegen::{MoveGenerator, uci};
use crate::search;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn message_loop() {
    let board = Arc::new(Mutex::new(Board::new()));
    // set to the standard start position by default
    {
        let mut b = board.lock().unwrap();
        let _ = b.fen_read(None);
    }

    // Spawn a thread to listen for UCI commands
    let board_clone = Arc::clone(&board);
    let input_thread = thread::spawn(move || {
        loop {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let command = input.trim();

            let options = command.split_whitespace().collect::<Vec<&str>>();
            match options.as_slice() {
                ["uci"] => {
                    uci();
                }
                ["ucinewgame"] => {
                    let mut b = board_clone.lock().unwrap();
                    reset(&mut *b);
                }
                ["isready"] => println!("readyok"),
                ["position", pos_options @ ..] => {
                    let mut b = board_clone.lock().unwrap();
                    position(&mut *b, pos_options);
                }
                ["go", go_options @ ..] => {
                    // Parse search options and capture the side-to-move under a short lock.
                    // We deliberately do NOT hold the lock during the whole search so the
                    // input thread can still receive commands (eg. "quit") while searching.
                    let side_to_move = {
                        let b = board_clone.lock().unwrap();
                        b.game_state.side_to_move
                    };

                    let mut info = search::SearchInfo::new();
                    let mut i = 0;
                    while i < go_options.len() {
                        match go_options[i] {
                            "infinite" => {
                                // Implement infinite search if needed
                            }
                            "wtime" => {
                                if i + 1 < go_options.len() && side_to_move == 0 {
                                    info.time = go_options[i + 1].parse().unwrap_or(0);
                                    i += 1;
                                }
                            }
                            "btime" => {
                                if i + 1 < go_options.len() && side_to_move == 1 {
                                    info.time = go_options[i + 1].parse().unwrap_or(0);
                                    i += 1;
                                }
                            }
                            "winc" => {
                                if i + 1 < go_options.len() && side_to_move == 0 {
                                    info.increment = go_options[i + 1].parse().unwrap_or(0);
                                    i += 1;
                                }
                            }
                            "binc" => {
                                if i + 1 < go_options.len() && side_to_move == 1 {
                                    info.increment = go_options[i + 1].parse().unwrap_or(0);
                                    i += 1;
                                }
                            }
                            "depth" => {
                                if i + 1 < go_options.len() {
                                    info.depth = go_options[i + 1].parse().unwrap_or(0);
                                    i += 1;
                                }
                            }
                            _ => {}
                        }
                        i += 1;
                    }

                    // If no depth was specified, set a default depth
                    if info.depth <= 0 {
                        info.depth = MAX_DEPTH;
                    }

                    // Spawn a dedicated search thread. It will lock the board for the
                    // duration of the search. The input thread remains free to handle
                    // commands such as "quit" (which calls process::exit and terminates
                    // the whole process immediately).
                    let board_for_search = Arc::clone(&board_clone);
                    thread::spawn(move || {
                        let mut b = board_for_search.lock().unwrap();
                        let move_generator = MoveGenerator::new();
                        let _ = search::search_position(&mut *b, &info, &move_generator);
                    });
                }
                ["quit"] => {
                    std::process::exit(0);
                }

                _ => eprintln!("Unknown command: '{}'", command.trim_end()),
            }
        }
    });

    // Wait for the input thread (it runs indefinitely until quit)
    let _ = input_thread.join();
}

fn uci() {
    println!("id name Newton {}", env!("CARGO_PKG_VERSION"));
    println!("id author Luskidoo");
    println!("uciok");
}

fn position(board: &mut Board, options: &[&str]) {
    // The `options` slice contains everything after the word "position".
    // Valid forms:
    //  - position startpos
    //  - position startpos moves e2e4 e7e5 ...
    //  - position fen <fen string> [moves ...]
    if options.is_empty() {
        return;
    }

    match options[0] {
        "startpos" => {
            let _ = board.fen_read(Some(FEN_START_POSITION));
            // Find the "moves" token and apply any moves after it
            if let Some(moves_idx) = options.iter().position(|&x| x == "moves") {
                let moves = &options[moves_idx + 1..];
                apply_moves(board, moves);
            }
        }

        "fen" => {
            // The FEN itself can contain spaces; consume tokens until we hit
            // an optional "moves" token (which begins the move list).
            let rest = &options[1..];
            let mut fen_end = rest.len();
            for (i, &tok) in rest.iter().enumerate() {
                if tok == "moves" {
                    fen_end = i;
                    break;
                }
            }

            // Join fen parts with spaces to reconstruct the full FEN string.
            let fen_str = rest[..fen_end].join(" ");
            let _ = board.fen_read(Some(&fen_str));

            // If there are moves after the FEN, apply them
            if fen_end < rest.len() && rest[fen_end] == "moves" {
                let moves = &rest[fen_end + 1..];
                apply_moves(board, moves);
            }
        }

        _ => {
            eprintln!("Unknown position command: {:?}", options);
        }
    }
    board.print_board();
}

fn apply_moves(board: &mut Board, moves: &[&str]) {
    let mg = MoveGenerator::new(); // Create once for all moves
    for &move_str in moves {
        if let Some(m) = uci::parse_uci_move(board, move_str) {
            let _ = board.make(m, &mg);
        }
    }
}

fn go(board: &mut Board, options: &[&str]) {
    let mut info = search::SearchInfo::new();

    let mut i = 0;
    while i < options.len() {
        match options[i] {
            "infinite" => {
                // Implement infinite search if needed
                // todo!()
            }
            "wtime" => {
                if i + 1 < options.len() && board.game_state.side_to_move == 0 {
                    info.time = options[i + 1].parse().unwrap_or(0);
                    i += 1;
                }
            }
            "btime" => {
                if i + 1 < options.len() && board.game_state.side_to_move == 1 {
                    info.time = options[i + 1].parse().unwrap_or(0);
                    i += 1;
                }
            }
            "winc" => {
                if i + 1 < options.len() && board.game_state.side_to_move == 0 {
                    info.increment = options[i + 1].parse().unwrap_or(0);
                    i += 1;
                }
            }
            "binc" => {
                if i + 1 < options.len() && board.game_state.side_to_move == 1 {
                    info.increment = options[i + 1].parse().unwrap_or(0);
                    i += 1;
                }
            }
            "depth" => {
                if i + 1 < options.len() {
                    info.depth = options[i + 1].parse().unwrap_or(0);
                    let move_generator = MoveGenerator::new();
                    let _ = search::search_position(board, &info, &move_generator);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
}

fn reset(board: &mut Board) {
    *board = Board::new();
    let _ = board.fen_read(None);
}
