use crate::{board::Board, evaluate::evaluate_position, movegen::MoveGenerator, movelist::MoveList};

pub struct SearchInfo {
    pub depth: i8,
    pub time: u8,
    pub increment: u8,
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
        board.make(m, move_generator);
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
    best_value
}

pub fn search_position(board: &mut Board, info: &SearchInfo, move_generator: &MoveGenerator) -> i32 {
    let best_score = alpha_beta(board, info.depth, -100000, 100000, info, move_generator);
    return best_score
}