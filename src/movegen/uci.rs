use crate::defs::{Square, Pieces};
use crate::movegen::bit_move::Move;
use crate::board::Board;

pub fn parse_uci_move(board: &Board, move_str: &str) -> Option<Move> {
    if move_str.len() < 4 || move_str.len() > 5 {
        return None;
    }
    
    // Parse source and destination squares
    let from_file = (move_str.chars().nth(0)? as u8 - b'a') as u8;
    let from_rank = move_str.chars().nth(1)? as u8 - b'1';
    let to_file = (move_str.chars().nth(2)? as u8 - b'a') as u8;
    let to_rank = move_str.chars().nth(3)? as u8 - b'1';
    
    if from_file > 7 || from_rank > 7 || to_file > 7 || to_rank > 7 {
        return None;
    }
    
    let from_sq = Square((from_rank * 8 + from_file) as usize);
    let to_sq = Square((to_rank * 8 + to_file) as usize);
    
    // Get the piece type at the source square
    let piece_type = board.piece_list[from_sq.0];
    if piece_type == Pieces::NONE {
        return None;  // No piece at source square
    }
    
    // Verify piece belongs to the side to move
    let side_to_move = board.game_state.side_to_move;
    if (board.pieces[side_to_move][piece_type] & from_sq.to_bb()).0 == 0 {
        return None;  // Piece doesn't belong to side to move
    }
    
    // Store the validated piece type
    let piece = piece_type;
    
    // Create move data
    // Format must match Move::new / Move decoding in movegen. Use same shifts
    let mut move_data = piece; // PIECE at shift 0
    move_data |= (from_sq.0 & 0x3F) << 3; // FROM_SQ shift is 3
    move_data |= (to_sq.0 & 0x3F) << 9;   // TO_SQ shift is 9

    // CAPTURE: always encode; if destination empty, encode Pieces::NONE
    let capture_piece = if board.piece_list[to_sq.0] != Pieces::NONE {
        board.piece_list[to_sq.0]
    } else {
        Pieces::NONE
    };
    move_data |= (capture_piece & 0x7) << 15; // CAPTURE shift is 15

    // PROMOTION: encode promotion piece if specified, otherwise encode NONE
    if move_str.len() == 5 {
        let promotion_char = move_str.chars().nth(4)?;
        let promotion_piece = match promotion_char {
            'q' => Some(Pieces::QUEEN),
            'r' => Some(Pieces::ROOK),
            'b' => Some(Pieces::BISHOP),
            'n' => Some(Pieces::KNIGHT),
            _ => None,
        }?;

        move_data |= (promotion_piece & 0x7) << 18; // PROMOTION shift is 18
    } else {
        move_data |= (Pieces::NONE & 0x7) << 18;
    }
    
    Some(Move::new(move_data))
}