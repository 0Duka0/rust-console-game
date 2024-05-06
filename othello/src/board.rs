use std::cmp::Ordering;

use crate::r#enum::{Color};


macro_rules! line {
    ($start:expr, $data:expr, $shift:ident, $n:expr) => {
        {
            let mut result = $data & $shift($start, $n);
            result |= $data & $shift(result, $n);
            result |= $data & $shift(result, $n);
            result |= $data & $shift(result, $n);
            result |= $data & $shift(result, $n);
            result |= $data & $shift(result, $n);
            result
        }
    }
}

#[inline]
const fn shiftL(a: u64, b: u32) -> u64 {
    a << b
}

#[inline]
const fn shiftR(a: u64, b: u32) -> u64 {
    a >> b
}

#[derive(Clone, Copy)]
enum Piece {
    Empty,
    Black,
    White,
}

impl Piece {
    fn to_char(&self) -> char {
        match self {
            Piece::Empty => '.',
            Piece::Black => '●',
            Piece::White => '○',
        }
    }
}

pub enum GameResult {
    Winner(Color),
    Draw,
}

#[allow(dead_code)]
pub struct Board {
    black: u64,
    white: u64,
    vBlack: i64,
    vWhite: i64,
    turns: usize,
}

#[allow(dead_code)]
impl Board {
    const LR_EDGE_MASK: u64 = 0x7e7e7e7e7e7e7e7e;
    const TB_EDGE_MASK: u64 = 0x00FFFFFFFFFFFF00;
    const LTRB_EDGE_MASK: u64 = 0x007e7e7e7e7e7e00;
    const MOVE_POSITION_MASK: u64 = 0x03F566ED27179461;
    const SHIFT_MASK_LIST: [(u32, u64); 4] = [
        (1, Board::LR_EDGE_MASK),
        (8, Board::TB_EDGE_MASK),
        (7, Board::LTRB_EDGE_MASK),
        (9, Board::LTRB_EDGE_MASK),
    ];
    const POSITION_TABLE: [u8; 64] = [
        0,1,59,2,60,40,54,3,61,32,49,41,55,19,35,4,
        62,52,30,33,50,12,14,42,56,16,27,20,36,23,44,5,
        63,58,39,53,31,48,18,34,51,29,11,13,15,26,22,43,
        57,38,47,17,28,10,25,21,37,46,9,24,45,8,7,6
    ];
    const POSITION_VALUE_TABLE: [[i64; 8]; 8] = [
        [120, -20, 20,  5,  5, 20, -20, 120],
        [-20, -40, -5, -5, -5, -5, -40, -20],
        [ 20,  -5, 15,  3,  3, 15,  -5,  20],
        [  5,  -5,  3,  3,  3,  3,  -5,   5],
        [  5,  -5,  3,  3,  3,  3,  -5,   5],
        [ 20,  -5, 15,  3,  3, 15,  -5,  20],
        [-20, -40, -5, -5, -5, -5, -40, -20],
        [120, -20, 20,  5,  5, 20, -20, 120],
    ];

    pub fn new() -> Self {
        let black = 0x0000000810000000;
        let white = 0x0000001008000000;
        let (vBlack, vWhite) = (6, 6);
        let n_moves = 0;
        Board { black, white, vBlack, vWhite, turns: n_moves }
    }

    pub fn turn(&self) -> Color {
        match self.turns % 2 {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!(),
        }
    }

    pub fn turns(&self) -> usize {
        self.turns
    }

    pub fn values(&self) -> (i64, i64) {
        (self.vBlack, self.vWhite)
    }

    pub fn end(&self) -> bool {
        (self.white ^ self.black).count_ones() == 64
    }

    pub fn pass(&self) -> Board {
        Board {
            black: self.black,
            white: self.white,
            vBlack: self.vBlack,
            vWhite: self.vWhite,
            turns: self.turns+1
        }
    }

    pub fn show(&self) {
        println!("  ABCDEFGH");
        for i in 0..8 {
            let mut row = String::new();
            for j in 0..8 {
                let pos = 1 << (8*(7-i)+7-j);
                let piece = if (self.white & pos) == pos {
                    Piece::White
                } else if (self.black & pos) == pos {
                    Piece::Black
                } else {
                    Piece::Empty
                };
                row.push(piece.to_char());
            }
            println!("{} {}", i+1, row);
        }
    }

    pub fn split_moves(&self, n: u64) -> Vec<u64> {
        let mut moves = Vec::<u64>::new();
        let mut memo = n;
        while memo != 0 {
            let y = memo & !memo.wrapping_sub(1);
            let index = (y.wrapping_mul(Board::MOVE_POSITION_MASK)) >> 58;
            let nShift = Board::POSITION_TABLE[index as usize];
            moves.push(1u64 << nShift);
            memo ^= y;
        }
        moves
    }

    pub fn legalMoves(&self) -> u64 {
        #[inline]
        const fn calc(tp: u64, ntp: u64, mask: u64, shift: u32) -> u64 {
            let l = line!(tp, ntp & mask, shiftL, shift);
            let r = line!(tp, ntp & mask, shiftR, shift);
            shiftL(l, shift) | shiftR(r, shift)
        }

        let players = [self.black, self.white];
        let tp = players[self.turns % 2];
        let ntp = players[(self.turns+1) % 2];
        let blankBoard = !(tp | ntp);
        let mut possible = 0;
        for (shift, mask) in Board::SHIFT_MASK_LIST {
            possible |= calc(tp, ntp, mask, shift);
        }
        possible & blankBoard
    }

    pub fn reverse(&self, position: u64) -> Self {
        #[inline]
        const fn calc(tp: u64, ntp: u64, position: u64, mask: u64, shift: u32) -> u64 {
            let mask = ntp & mask;
            let l1 = line!(position, mask, shiftL, shift);
            let r1 = line!(tp, mask, shiftR, shift);
            let r2 = line!(position, mask, shiftR, shift);
            let l2 = line!(tp, mask, shiftL, shift);
            (l1 & r1) | (r2 & l2)
        }

        let players = [self.black, self.white];
        let values = [self.vBlack, self.vWhite];
        let tp = players[self.turns % 2];
        let ntp = players[(self.turns+1) % 2];
    
        let mut target = 0u64;
        for (shift, mask) in Board::SHIFT_MASK_LIST {
            target |= calc(tp, ntp, position, mask, shift);
        }
        let newTp = tp ^ position ^ target;
        let newNtp = ntp ^ target;
        let mut vTp = values[self.turns % 2];
        let mut vNtp = values[(self.turns+1) % 2];
        let y = position & !position.wrapping_sub(1);
        let index = (y.wrapping_mul(Board::MOVE_POSITION_MASK)) >> 58;
        let nShift = Board::POSITION_TABLE[index as usize];
        vTp += Board::POSITION_VALUE_TABLE[(7-nShift/8) as usize][(nShift%8) as usize];
        while target != 0 {
            let y = target & !target.wrapping_sub(1);
            let index = (y.wrapping_mul(Board::MOVE_POSITION_MASK)) >> 58;
            let nShift = Board::POSITION_TABLE[index as usize];
            let i = (7 - nShift / 8) as usize;
            let j = (nShift % 8) as usize;
            vTp += Board::POSITION_VALUE_TABLE[i][j];
            vNtp -= Board::POSITION_VALUE_TABLE[i][j];
            target ^= y;
        }
        let newPlayers = [newTp, newNtp];
        let newValues = [vTp, vNtp];
        let black = newPlayers[self.turns % 2];
        let white = newPlayers[(self.turns+1) % 2];
        let vBlack = newValues[self.turns % 2];
        let vWhite = newValues[(self.turns+1) % 2];
        Board { black, white, vBlack, vWhite, turns: self.turns + 1 }
    }

    pub fn judge(&self) -> GameResult {
        let whiteCount = self.white.count_ones();
        let blackCount = self.black.count_ones();
        
        match whiteCount.cmp(&blackCount) {
            Ordering::Equal => GameResult::Draw,
            Ordering::Greater => GameResult::Winner(Color::White),
            Ordering::Less => GameResult::Winner(Color::Black),
        }
    }

    fn debug(board: u64) {
        for i in (0..64).rev() {
            print!("{}", (board & (1u64 << i)) >> i);
            if (64 - i) % 8 == 0 {
                println!("");
            }
        }
    }
}