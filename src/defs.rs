use std::fmt;

use crate::bitboard::*;

pub type Side = usize;
pub type Piece = usize;
pub struct Pieces;
impl Pieces {
    pub const KING: Piece = 0;
    pub const QUEEN: Piece = 1;
    pub const ROOK: Piece = 2;
    pub const BISHOP: Piece = 3;
    pub const KNIGHT: Piece = 4;
    pub const PAWN: Piece = 5;
    pub const NONE: Piece = 6;
}

#[derive(Copy, Clone, PartialEq)]
pub struct Sides;
impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const BOTH: Side = 2;
}

pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const FEN_KIWIPETE_POSITION: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
pub const MAX_DEPTH: i8 = 64;
pub struct NrOf;
impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const CASTLING_PERMISSIONS: usize = 16; // 0-15
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
}

pub struct Castling;
impl Castling {
    pub const WK: BitBoard = BitBoard(1);
    pub const WQ: BitBoard = BitBoard(2);
    pub const BK: BitBoard = BitBoard(4);
    pub const BQ: BitBoard = BitBoard(8);
    pub const ALL: BitBoard = BitBoard(15);
}

#[derive(PartialEq, PartialOrd, Clone, Eq)]
pub struct Square(pub usize);
impl Square {
    // White side squares that are important for castling
    pub const A1: Self = Square(0);
    pub const B1: Self = Square(1);
    pub const C1: Self = Square(2);
    pub const D1: Self = Square(3);
    pub const E1: Self = Square(4);
    pub const F1: Self = Square(5);
    pub const G1: Self = Square(6);
    pub const H1: Self = Square(7);

    // Black side squares that are important for castling
    pub const A8: Self = Square(56);
    pub const B8: Self = Square(57);
    pub const C8: Self = Square(58);
    pub const D8: Self = Square(59);
    pub const E8: Self = Square(60);
    pub const F8: Self = Square(61);
    pub const G8: Self = Square(62);
    pub const H8: Self = Square(63);

    // White EP-squares start/end
    pub const A3: Self = Square(16);
    pub const H3: Self = Square(23);

    // Black EP-squares start/end
    pub const A6: Self = Square(40);
    pub const H6: Self = Square(47);

    pub fn to_bb(&self) -> BitBoard {
        BitBoard(1 << self.0)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Files;
impl Files {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const G: usize = 6;
    pub const H: usize = 7;
}

pub struct Ranks;
impl Ranks {
    pub const R1: usize = 0;
    pub const R2: usize = 1;
    pub const R4: usize = 3;
    pub const R5: usize = 4;
    pub const R7: usize = 6;
    pub const R8: usize = 7;
}

pub const MAX_MOVE_RULE: u8 = 100; // 50/75 move rule
pub const MAX_GAME_MOVES: usize = 2048;
pub const MAX_POSITION_MOVES: usize = 255;

#[rustfmt::skip]
pub const SQUARE_NAME: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"
];

pub const PIECE_CHAR_SMALL: [&str; NrOf::PIECE_TYPES + 1] = ["k", "q", "r", "b", "n", "p", ""];

pub static ROOK_MAGICS: [BitBoard; 64] = [
    BitBoard(0x2280008130400221),
    BitBoard(0x0140006000100041),
    BitBoard(0x4100140841002000),
    BitBoard(0x0100041000200900),
    BitBoard(0x0a80024800040080),
    BitBoard(0x09000803004e0400),
    BitBoard(0x0400008201043018),
    BitBoard(0xc200082040920104),
    BitBoard(0x0204800080400628),
    BitBoard(0x4200402000401000),
    BitBoard(0x0812002200104080),
    BitBoard(0x0101000820100101),
    BitBoard(0x00c0800800800400),
    BitBoard(0x0882001002010408),
    BitBoard(0x0002000408018200),
    BitBoard(0x4401002042008100),
    BitBoard(0x2018208000400880),
    BitBoard(0x8040048040802000),
    BitBoard(0x20b0002000280402),
    BitBoard(0xc450008080080010),
    BitBoard(0x2210850010080100),
    BitBoard(0x0000808004000200),
    BitBoard(0x8088040018624130),
    BitBoard(0x0300020028804904),
    BitBoard(0x0800400580008020),
    BitBoard(0x0500200040100040),
    BitBoard(0x0410200080801000),
    BitBoard(0x050021010010000c),
    BitBoard(0x1002002200081004),
    BitBoard(0x0000040080800200),
    BitBoard(0x0002002200080441),
    BitBoard(0x6134803080004100),
    BitBoard(0x0080002000400040),
    BitBoard(0x0030201008400044),
    BitBoard(0x20308a1000802000),
    BitBoard(0x0460800800801002),
    BitBoard(0x0028010025000850),
    BitBoard(0xaa8200140e008810),
    BitBoard(0x0001000421008200),
    BitBoard(0x0000800141800300),
    BitBoard(0x1000802040008010),
    BitBoard(0x40c0100800242000),
    BitBoard(0x0410080024002000),
    BitBoard(0x0808100100090020),
    BitBoard(0x0000080004008080),
    BitBoard(0x0004000402008080),
    BitBoard(0x0050020110040008),
    BitBoard(0x000e808041220014),
    BitBoard(0x2c091442a0800100),
    BitBoard(0x4004482208810600),
    BitBoard(0x0310200010008080),
    BitBoard(0x0030010020081100),
    BitBoard(0x4000110004080100),
    BitBoard(0x0002000280040080),
    BitBoard(0x00c2000801040200),
    BitBoard(0x0800800100006080),
    BitBoard(0x1000104081082202),
    BitBoard(0xc001088020124202),
    BitBoard(0x10200a0022118042),
    BitBoard(0x0900042100100009),
    BitBoard(0x4c21000800100205),
    BitBoard(0x0001000802040001),
    BitBoard(0x0832000400c82102),
    BitBoard(0x1200104881040822),
];

pub static BISHOP_MAGICS: [BitBoard; 64] = [
    BitBoard(0x0022200802890048),
    BitBoard(0x0491240110420000),
    BitBoard(0x2016041112000010),
    BitBoard(0x0020a10040214104),
    BitBoard(0x8084042085000001),
    BitBoard(0x00012820900000a0),
    BitBoard(0x200088c820300004),
    BitBoard(0xb002004400841010),
    BitBoard(0x0010400862040040),
    BitBoard(0x0002048804ad0208),
    BitBoard(0x0001098803010100),
    BitBoard(0x2000444400840000),
    BitBoard(0x4000020210000000),
    BitBoard(0x0a00060924a00082),
    BitBoard(0x10000410a2282000),
    BitBoard(0x8800010121012019),
    BitBoard(0x0041240802040402),
    BitBoard(0x0420888401020229),
    BitBoard(0x0081005004002442),
    BitBoard(0x441800040c20a81a),
    BitBoard(0x0001000820080422),
    BitBoard(0x0806204202100240),
    BitBoard(0x010040510108204a),
    BitBoard(0x0420800214440200),
    BitBoard(0x0020047020840404),
    BitBoard(0x0104202004014408),
    BitBoard(0x00003000020c01c0),
    BitBoard(0x001c004044010042),
    BitBoard(0x2001001011004000),
    BitBoard(0xc004024018080a08),
    BitBoard(0x2041012214008800),
    BitBoard(0x0000420001090110),
    BitBoard(0x040c20200028c200),
    BitBoard(0x0101040302202810),
    BitBoard(0x0001140a03040804),
    BitBoard(0x0200020080080080),
    BitBoard(0x0242080410060200),
    BitBoard(0x0004102080024800),
    BitBoard(0x0110850040630428),
    BitBoard(0x2548204040508202),
    BitBoard(0x0214100446241000),
    BitBoard(0x0013080804023240),
    BitBoard(0x10000c0044008800),
    BitBoard(0x800106020421ba00),
    BitBoard(0x4800181010400c00),
    BitBoard(0x0040082104084040),
    BitBoard(0x01040102420a0400),
    BitBoard(0x0410821e00280040),
    BitBoard(0x0414040109080000),
    BitBoard(0x0104840402020019),
    BitBoard(0x0000020442480000),
    BitBoard(0x0000202442020080),
    BitBoard(0xc00c412020411000),
    BitBoard(0x1000418408208300),
    BitBoard(0x8009a21444040000),
    BitBoard(0x0420011131090800),
    BitBoard(0x0002042402480400),
    BitBoard(0x4040620114029200),
    BitBoard(0x8300400144044405),
    BitBoard(0x0402910280208800),
    BitBoard(0x00008808e0024400),
    BitBoard(0x0003900810100088),
    BitBoard(0x0250210204284280),
    BitBoard(0x0028820c18020010),
];
