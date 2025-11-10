use crate::{board::Board, evaluate::evaluate_position, movegen::MoveGenerator, movelist::MoveList};
use std::io::Write;

pub struct SearchInfo {
    pub depth: i8,
    pub time: u32,
    pub increment: u32,
}

impl SearchInfo {
    pub fn new() -> Self {
        SearchInfo { depth: -1, time: 0, increment: 0 }
    }
}

pub fn alpha_beta(
    board: &mut Board,
    depth: i8,
    mut alpha: i32,
    beta: i32,
    info: &SearchInfo,
    move_generator: &MoveGenerator,
) -> i32 {
    
    if depth == 0 {
        return evaluate_position(board);
    }
    let mut best_value = -99999;

    let mut list = MoveList::new();
    move_generator.generate_all_moves(board, &mut list);

    for i in 0..list.len() {
        let m = list.get_move(i);
        // Only proceed if the move is legal. make() returns false for illegal moves
        // and already handles unmaking them internally, so we only call unmake()
        // for legal moves to maintain balanced push/pop operations on the history stack.
        if board.make(m, move_generator) {
            let score = -alpha_beta(board, depth - 1, -beta, -alpha, info, move_generator);
            board.unmake();

            if score > best_value {
                best_value = score;
            }
            if score > alpha {
                alpha = score;
            }
            if score >= beta {
                return best_value
            }
        }
    }
    best_value
}

pub fn search_position(board: &mut Board, info: &SearchInfo, move_generator: &MoveGenerator) {
    let start_time = std::time::Instant::now();
    let search_time = info.time / 20 + info.increment / 2;
    for curr_depth in 1..=info.depth {
        // Use curr_depth instead of info.depth to implement iterative deepening correctly.
        // This ensures each iteration searches to the appropriate depth level.
        let best_score = alpha_beta(board, curr_depth, -100000, 100000, info, move_generator);
    
        println!("info depth {} score {}", curr_depth, best_score);
        let _ = std::io::stdout().flush();
        let current_time = std::time::Instant::now();
        if current_time.duration_since(start_time).as_millis() as u32 >= search_time {
            break;
        }
    //return best_score
    }
}