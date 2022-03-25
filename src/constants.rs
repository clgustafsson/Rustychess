pub const FEN_STARTPOSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const POSITION_CONVERTER_RUSTYCHESS: [usize; 128] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 100, 101, 102, 103, 104, 105, 106, 107,
    200, 201, 202, 203, 204, 205, 206, 207, 300, 301, 302, 303, 304, 305, 306, 307, 400, 401, 402,
    403, 404, 405, 406, 407, 156, 157, 158, 159, 160, 161, 162, 163, 256, 257, 258, 259, 260, 261,
    262, 263, 356, 357, 358, 359, 360, 361, 362, 363, 456, 457, 458, 459, 460, 461, 462, 463,
];
pub const POSTION_CONVERTER_STANDARD: [&str; 128] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a8n", "b8n", "c8n", "d8n", "e8n", "f8n", "g8n", "h8n", "a8b", "b8b", "c8b", "d8b", "e8b",
    "f8b", "g8b", "h8b", "a8r", "b8r", "c8r", "d8r", "e8r", "f8r", "g8r", "h8r", "a8q", "b8q",
    "c8q", "d8q", "e8q", "f8q", "g8q", "h8q", "a1n", "b1n", "c1n", "d1n", "e1n", "f1n", "g1n",
    "h1n", "a1b", "b1b", "c1b", "d1b", "e1b", "f1b", "g1b", "h1b", "a1r", "b1r", "c1r", "d1r",
    "e1r", "f1r", "g1r", "h1r", "a1q", "b1q", "c1q", "d1q", "e1q", "f1q", "g1q", "h1q",
];

//ascii numbers for pieces in format <Piece>_<Color>
pub const R_W: u8 = 82;
pub const KN_W: u8 = 78;
pub const B_W: u8 = 66;
pub const Q_W: u8 = 81;
pub const K_W: u8 = 75;
pub const P_W: u8 = 80;
pub const R_B: u8 = 114;
pub const KN_B: u8 = 110;
pub const B_B: u8 = 98;
pub const Q_B: u8 = 113;
pub const K_B: u8 = 107;
pub const P_B: u8 = 112;

pub const EMPTY_PLACEHOLDER_NUM: i32 = 1000;

pub const OTHER_COLOR_PIECE: u8 = 83;

pub const DISTANCE_TO_EDGE: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 2, 2, 2, 2, 1, 0, 0, 1, 2, 3, 3, 2, 1, 0,
    0, 1, 2, 3, 3, 2, 1, 0, 0, 1, 2, 2, 2, 2, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

//format for consts below
// Can<Piece>Move<Distance>   CKNMN17 -> C KN M N17 -> Can Knight Move - 17

pub const CKNM17: [bool; 64] = [
    true, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
    false, true, true, true, true, true, true, true, false, true, true, true, true, true, true,
    true, false, true, true, true, true, true, true, true, false, true, true, true, true, true,
    true, true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false,
];

pub const CKNM15: [bool; 64] = [
    false, true, true, true, true, true, true, true, false, true, true, true, true, true, true,
    true, false, true, true, true, true, true, true, true, false, true, true, true, true, true,
    true, true, false, true, true, true, true, true, true, true, false, true, true, true, true,
    true, true, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false,
];

pub const CKNM10: [bool; 64] = [
    true, true, true, true, true, true, false, false, true, true, true, true, true, true, false,
    false, true, true, true, true, true, true, false, false, true, true, true, true, true, true,
    false, false, true, true, true, true, true, true, false, false, true, true, true, true, true,
    true, false, false, true, true, true, true, true, true, false, false, false, false, false,
    false, false, false, false, false,
];

pub const CKNM6: [bool; 64] = [
    false, false, true, true, true, true, true, true, false, false, true, true, true, true, true,
    true, false, false, true, true, true, true, true, true, false, false, true, true, true, true,
    true, true, false, false, true, true, true, true, true, true, false, false, true, true, true,
    true, true, true, false, false, true, true, true, true, true, true, false, false, false, false,
    false, false, false, false,
];

pub const CKNMN17: [bool; 64] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, true, true, true, true, true, true, true, false, true, true, true,
    true, true, true, true, false, true, true, true, true, true, true, true, false, true, true,
    true, true, true, true, true, false, true, true, true, true, true, true, true, false, true,
    true, true, true, true, true, true,
];

pub const CKNMN15: [bool; 64] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, true, true, true, true, true, true, true, false, true, true, true, true,
    true, true, true, false, true, true, true, true, true, true, true, false, true, true, true,
    true, true, true, true, false, true, true, true, true, true, true, true, false, true, true,
    true, true, true, true, true, false,
];

pub const CKNMN10: [bool; 64] = [
    false, false, false, false, false, false, false, false, false, false, true, true, true, true,
    true, true, false, false, true, true, true, true, true, true, false, false, true, true, true,
    true, true, true, false, false, true, true, true, true, true, true, false, false, true, true,
    true, true, true, true, false, false, true, true, true, true, true, true, false, false, true,
    true, true, true, true, true,
];

pub const CKNMN6: [bool; 64] = [
    false, false, false, false, false, false, false, false, true, true, true, true, true, true,
    false, false, true, true, true, true, true, true, false, false, true, true, true, true, true,
    true, false, false, true, true, true, true, true, true, false, false, true, true, true, true,
    true, true, false, false, true, true, true, true, true, true, false, false, true, true, true,
    true, true, true, false, false,
];

pub const CRM8: [bool; 64] = [
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, false, false, false, false, false, false,
    false, false,
];

pub const CRM1: [bool; 64] = [
    true, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
    false, true, true, true, true, true, true, true, false, true, true, true, true, true, true,
    true, false, true, true, true, true, true, true, true, false, true, true, true, true, true,
    true, true, false, true, true, true, true, true, true, true, false, true, true, true, true,
    true, true, true, false,
];

pub const CRMN8: [bool; 64] = [
    false, false, false, false, false, false, false, false, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true,
];

pub const CRMN1: [bool; 64] = [
    false, true, true, true, true, true, true, true, false, true, true, true, true, true, true,
    true, false, true, true, true, true, true, true, true, false, true, true, true, true, true,
    true, true, false, true, true, true, true, true, true, true, false, true, true, true, true,
    true, true, true, false, true, true, true, true, true, true, true, false, true, true, true,
    true, true, true, true,
];

pub const CBM9: [bool; 64] = [
    true, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
    false, true, true, true, true, true, true, true, false, true, true, true, true, true, true,
    true, false, true, true, true, true, true, true, true, false, true, true, true, true, true,
    true, true, false, true, true, true, true, true, true, true, false, false, false, false, false,
    false, false, false, false,
];

pub const CBM7: [bool; 64] = [
    false, true, true, true, true, true, true, true, false, true, true, true, true, true, true,
    true, false, true, true, true, true, true, true, true, false, true, true, true, true, true,
    true, true, false, true, true, true, true, true, true, true, false, true, true, true, true,
    true, true, true, false, true, true, true, true, true, true, true, false, false, false, false,
    false, false, false, false,
];

pub const CBMN9: [bool; 64] = [
    false, false, false, false, false, false, false, false, false, true, true, true, true, true,
    true, true, false, true, true, true, true, true, true, true, false, true, true, true, true,
    true, true, true, false, true, true, true, true, true, true, true, false, true, true, true,
    true, true, true, true, false, true, true, true, true, true, true, true, false, true, true,
    true, true, true, true, true,
];

pub const CBMN7: [bool; 64] = [
    false, false, false, false, false, false, false, false, true, true, true, true, true, true,
    true, false, true, true, true, true, true, true, true, false, true, true, true, true, true,
    true, true, false, true, true, true, true, true, true, true, false, true, true, true, true,
    true, true, true, false, true, true, true, true, true, true, true, false, true, true, true,
    true, true, true, true, false,
];

pub const CPM16: [bool; 64] = [
    false, false, false, false, false, false, false, false, true, true, true, true, true, true,
    true, true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false,
];

pub const CPMN16: [bool; 64] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, true, true, true, true,
    true, true, true, false, false, false, false, false, false, false, false,
];
