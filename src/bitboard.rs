use crate::defs::Square;
use std::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not, Shl, ShlAssign,
    Shr, Sub,
};

#[derive(Copy, Clone, PartialEq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0u64);
    pub const RANK4: BitBoard = BitBoard(0x00000000FF000000);
    pub const RANK5: BitBoard = BitBoard(0x000000FF00000000);
    pub const A_FILE: BitBoard = BitBoard(0x101010101010101);
    pub const B_FILE: BitBoard = BitBoard(0x202020202020202);
    pub const G_FILE: BitBoard = BitBoard(0x4040404040404040);
    pub const H_FILE: BitBoard = BitBoard(0x8080808080808080);
    pub const AB_FILE: BitBoard = BitBoard(BitBoard::A_FILE.0 | BitBoard::B_FILE.0);
    pub const GH_FILE: BitBoard = BitBoard(BitBoard::G_FILE.0 | BitBoard::H_FILE.0);
    pub const NOT_RANK_1: BitBoard = BitBoard(0xffffffffffffff00);
    pub const NOT_RANK_2: BitBoard = BitBoard(0xffffffffffff00ff);
    pub const NOT_RANK_7: BitBoard = BitBoard(0xff00ffffffffffff);
    pub const NOT_RANK_8: BitBoard = BitBoard(0xffffffffffffff);

    pub fn set_bit(self, x: BitBoard) -> BitBoard {
        self | x
    }

    pub fn south_one(self) -> Self {
        self >> BitBoard(8)
    }

    pub fn north_one(self) -> Self {
        self << BitBoard(8)
    }

    pub fn pop_count(self) -> u32 {
        self.0.count_ones()
    }

    pub fn east_one(self) -> Self {
        self << BitBoard(1) & !Self::A_FILE
    }

    pub fn west_one(self) -> Self {
        self >> BitBoard(1) & !Self::H_FILE
    }

    pub fn next(bitboard: &mut BitBoard) -> Square {
        let square: u64 = bitboard.0.trailing_zeros() as u64;
        //println!("Bitboard before {:?}", bitboard);
        *bitboard ^= BitBoard(1u64 << square);
        //println!("Bitboard after {:?}", bitboard);
        Square(square as usize)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl Shl for BitBoard {
    type Output = BitBoard;
    fn shl(self, rhs: Self) -> Self {
        BitBoard(self.0 << rhs.0)
    }
}

impl ShlAssign for BitBoard {
    #[inline]
    fn shl_assign(&mut self, rhs: Self) {
        self.0 <<= rhs.0
    }
}

// #![feature(const_trait_impl)]
// impl ~const Shl for BitBoard {
//     type Output = BitBoard;
//     fn shl(self, rhs: Self) -> Self{
//         BitBoard(self.0 << rhs.0)
//     }
// }

impl Shr for BitBoard {
    type Output = BitBoard;
    fn shr(self, rhs: Self) -> Self {
        BitBoard(self.0 >> rhs.0)
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;
    fn bitand(self, rhs: Self) -> Self {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Not for BitBoard {
    type Output = BitBoard;
    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

impl Add for BitBoard {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        BitBoard(self.0 + rhs.0)
    }
}

impl Mul for BitBoard {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        BitBoard(self.0 * rhs.0)
    }
}

impl Sub for BitBoard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        BitBoard(self.0 - rhs.0)
    }
}

impl std::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BitBoard(0x{:016x})", self.0)
    }
}

impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = rank * 8 + file;
                write!(f, "{} ", if self.0 & (1 << sq) != 0 { "1" } else { "0" })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
