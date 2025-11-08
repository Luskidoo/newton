mod bitboard;
mod board;
mod defs;
mod movegen;
mod movelist;
mod perft;
mod uci;

use crate::bitboard::*;
use crate::movegen::MoveGenerator;
use crate::movelist::*;
use board::*;

fn main() {
    let mut board = Board::new();
    let init_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let test_fen = "4k3/8/8/8/n7/8/P7/4K3 w - - 0 1";
    let max_moves = "R6R/3Q4/1Q4Q1/4Q3/2Q4Q/Q4Q2/pp1Q4/kBNN1KB1 w - - 0 1";
    let kiwi_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let pos_4 = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    let pos_3 = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -";
    let promotion = "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1";
    let fen_result = board.fen_read(Some(kiwi_fen));
    let mut list = MoveList::new();
    let move_gen = MoveGenerator::new();
    perft::run(board, 5, move_gen);
}
