use crate::board::Board;
use crate::defs::{FEN_START_POSITION, Side};
use crate::movegen::{MoveGenerator, uci};
use crate::search;

pub fn message_loop() {
    let mut board = Board::new();
    // set to the standard start position by default
    let _ = board.fen_read(None);
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = input.trim();

        let options = command.split_whitespace().collect::<Vec<&str>>();
        match options.as_slice() {
            ["uci"] => uci(),
            ["ucinewgame"] => reset(&mut board),
            ["isready"] => println!("readyok"),
            ["position", options @ ..] => position(&mut board, options),
            ["go", options @ ..] => go(&mut board, options),
            ["quit"] => std::process::exit(0),

            _ => eprintln!("Unknown command: '{}'", command.trim_end()),
        }
    }
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

    while !options.is_empty() {
        match options {
            ["infinite"] => {
                todo!()
            }

            ["wtime", time @ ..] => {
                if board.game_state.side_to_move == 0 {
                    info.time = time[0].parse().unwrap();
                }
            }

            ["btime", time @ ..] => {
                if board.game_state.side_to_move == 1 {
                    info.time = time[0].parse().unwrap();
                }
            }

            ["winc", increment @ ..] => {
                if board.game_state.side_to_move == 0 {
                    info.increment = increment[0].parse().unwrap();
                }
            }

            ["binc", increment @ ..] => {
                if board.game_state.side_to_move == 1 {
                    info.increment = increment[0].parse().unwrap();
                }
            }

            ["depth", depth @ ..] => {
                info.depth = depth[0].parse().unwrap();

                let move_generator = MoveGenerator::new();
                let score= search::search_position(board, &info, &move_generator);
                println!("{}", score);
            }

            _ => {}
        }
    }
}

fn reset(board: &mut Board) {
    *board = Board::new();
    board.fen_read(None);
}
