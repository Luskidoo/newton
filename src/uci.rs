use crate::board::Board;
use crate::search;

pub fn message_loop() {
    let mut board = Board::new();
    board.fen_read(None);

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
            ["go", options @ ..] => go(options),
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
    while !options.is_empty() {
        match options {
            ["startpos"] => {
                board.fen_read(None);
            }
            ["fen", position @ ..] => {
                board.fen_read(Some(&position.join("")));
            }
            _ => {}
        }
    }
}

fn go(options: &[&str]) {
    let info = search::SearchInfo::new();

    while !options.is_empty() {
        match options {
            ["infinite"] => {
                todo!()
            }

            ["wtime"] => {
                todo!()
            }

            ["btime"] => {
                todo!()
            }

            ["winc"] => {
                todo!()
            }

            ["binc"] => {
                todo!()
            }

            _ => {}
        }
    }
}

fn reset(board: &mut Board) {
    let mut board = Board::new();
    board.fen_read(None);
}
