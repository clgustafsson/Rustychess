#![allow(dead_code)]
#![allow(unused_variables)]

use crate::board::b::*;
use crate::board_consts::bc::*;
use log::LevelFilter;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::process;

mod board;
mod board_consts;

const FEN_STARTPOSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

const POSITION_CONVERTER_RUSTYCHESS: [usize; 128] = [
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
  27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
  51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 100, 101, 102, 103, 104, 105, 106, 107, 200,
  201, 202, 203, 204, 205, 206, 207, 300, 301, 302, 303, 304, 305, 306, 307, 400, 401, 402, 403,
  404, 405, 406, 407, 156, 157, 158, 159, 160, 161, 162, 163, 256, 257, 258, 259, 260, 261, 262,
  263, 356, 357, 358, 359, 360, 361, 362, 363, 456, 457, 458, 459, 460, 461, 462, 463,
];
const POSTION_CONVERTER_STANDARD: [&str; 128] = [
  "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
  "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
  "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
  "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
  "a8n", "b8n", "c8n", "d8n", "e8n", "f8n", "g8n", "h8n", "a8b", "b8b", "c8b", "d8b", "e8b", "f8b",
  "g8b", "h8b", "a8r", "b8r", "c8r", "d8r", "e8r", "f8r", "g8r", "h8r", "a8q", "b8q", "c8q", "d8q",
  "e8q", "f8q", "g8q", "h8q", "a1n", "b1n", "c1n", "d1n", "e1n", "f1n", "g1n", "h1n", "a1b", "b1b",
  "c1b", "d1b", "e1b", "f1b", "g1b", "h1b", "a1r", "b1r", "c1r", "d1r", "e1r", "f1r", "g1r", "h1r",
  "a1q", "b1q", "c1q", "d1q", "e1q", "f1q", "g1q", "h1q",
];

//ascii numbers for pieces in format <Piece>_<Color>
const R_W: u8 = 82;
const KN_W: u8 = 78;
const B_W: u8 = 66;
const Q_W: u8 = 81;
const K_W: u8 = 75;
const P_W: u8 = 80;
const R_B: u8 = 114;
const KN_B: u8 = 110;
const B_B: u8 = 98;
const Q_B: u8 = 113;
const K_B: u8 = 107;
const P_B: u8 = 112;

const OTHER_COLOR_PIECE: u8 = 83;

const EMPTY_PLACEHOLDER_NUM: i32 = 1000;

impl Board {
  fn load_fen(&mut self, fen: &str) {
    let mut i: usize = 0;
    let mut fen_i: usize = 0;
    for x in fen.as_bytes() {
      if x == &32 {
        fen_i += 1
      } else {
        match fen_i {
          0 => {
            //board
            if x > &48 && x < &57 {
              let new_num = x - 48; //If x is a 'number then this corresponds to that number
              i += new_num as usize;
            }
            if x > &65 && x < &90 || x > &97 && x < &122 {
              self.board[i] = *x;
              i += 1;
            }
          }
          1 => {
            //turn
            if x == &119 {
              self.turn = true
            } else {
              self.turn = false
            }
          }
          2 => {
            //castling rights
            match x {
              75 => self.crights[0] = true,
              81 => self.crights[1] = true,
              107 => self.crights[2] = true,
              113 => self.crights[3] = true,
              _ => (),
            }
          }
          3 => {
            //en passant target square
            if x > &96 {
              //filter out numbers and '-'
              if self.turn {
                let wepts = x - Q_W;
                self.epts = wepts as usize
              }
              //if whites turn epts will always be on 6th rank
              else {
                let bepts = x - 57;
                self.epts = bepts as usize
              }
            } //if blacks turn epts will always be on 3rd rank
          }
          4 => {
            //halfmove clock
            if self.halfmove == 0 {
              self.halfmove = x - 48
            } else {
              self.halfmove = self.halfmove * 10 + x - 48
            } //stringlike addition with number
          }
          5 => {
            //fullmove
            if self.fullmove == 1 {
              self.fullmove = x - 48
            } else {
              self.fullmove = self.fullmove * 10 + x - 48
            } //stringlike addition with number
          }
          _ => (),
        }
      }
    }
  }
  fn make_move(&mut self, from: usize, mut to: usize) {
    let mut is_capture: bool = false;
    let mut is_pawnmove: bool = false;
    let mut is_doublepawnmove: bool = false;
    let mut is_enpassant: bool = false;
    let mut is_kingmove: bool = false;
    let mut is_kcastle: bool = false;
    let mut is_qcastle: bool = false;
    if to >= 400 {
      to -= 400;
      if self.turn {
        self.board[from] = Q_W;
      } else {
        self.board[from] = Q_B;
      }
    } else if to >= 300 {
      to -= 300;
      if self.turn {
        self.board[from] = R_W;
      } else {
        self.board[from] = R_B;
      }
    } else if to >= 200 {
      to -= 200;
      if self.turn {
        self.board[from] = B_W;
      } else {
        self.board[from] = B_B;
      }
    } else if to >= 100 {
      to -= 100;
      if self.turn {
        self.board[from] = KN_W;
      } else {
        self.board[from] = KN_B;
      }
    }
    if self.board[from] == K_W || self.board[from] == K_B {
      is_kingmove = true;
      if to as i8 - from as i8 == 2 {
        is_kcastle = true;
      }
      if from as i8 - to as i8 == 2 {
        is_qcastle = true;
      }
    }

    if self.board[from] == P_W && from - to == 16 || self.board[from] == P_B && to - from == 16 {
      is_doublepawnmove = true;
    }
    if self.board[to] != 0 {
      is_capture = true
    }
    if self.board[from] == P_W || self.board[from] == P_B {
      is_pawnmove = true;
      if to == self.epts {
        is_enpassant = true;
      }
    }
    if self.board[to] == K_W || self.board[to] == K_B {
      self.checkmate = true
    }
    //update board
    if is_kcastle {
      self.board[from + 1] = self.board[to + 1];
      self.board[to + 1] = 0;
    }
    if is_qcastle {
      self.board[from - 1] = self.board[to - 2];
      self.board[to - 2] = 0;
    }
    if is_enpassant {
      if self.turn {
        self.board[to + 8] = 0;
      } else {
        self.board[to - 8] = 0;
      }
    }
    self.board[to] = self.board[from];
    self.board[from] = 0;
    //update turn and fullmove
    if self.turn {
      self.turn = false
    } else {
      self.turn = true;
      self.fullmove += 1
    }
    //update halfmove
    if is_capture == false && is_pawnmove == false {
      self.halfmove += 1
    }
    //update epts
    if is_doublepawnmove {
      let epts: usize = from + to;
      self.epts = epts / 2
    } else {
      self.epts = 100
    }
    //update crights
    if is_kingmove {
      if self.turn {
        self.crights[0] = false;
        self.crights[1] = false;
      } else {
        self.crights[2] = false;
        self.crights[3] = false;
      }
    }
    if from == 63 || to == 63 {
      self.crights[0] = false;
    }
    if from == 56 || to == 56 {
      self.crights[1] = false;
    }
    if from == 7 || to == 7 {
      self.crights[2] = false;
    }
    if from == 0 || to == 0 {
      self.crights[3] = false;
    }
  }
  fn value(&self) -> i32 {
    let mut eval: i32 = 0;
    for position in 0..64 {
      match self.board[position] {
        R_B => {
          eval -= 50;
          eval -= R_VALUE_MAP[63 - position];
        }
        KN_B => {
          eval -= 30;
          eval -= N_VALUE_MAP[63 - position];
        }
        B_B => {
          eval -= 30;
          eval -= B_VALUE_MAP[63 - position];
        }
        Q_B => {
          eval -= 90;
          eval -= Q_VALUE_MAP[63 - position];
        }
        K_B => {
          eval -= 1000;
          eval -= K_VALUE_MAP[63 - position];
        }
        P_B => {
          eval -= 10;
          eval -= P_VALUE_MAP[63 - position];
        }
        R_W => {
          eval += 50;
          eval += R_VALUE_MAP[position];
        }
        KN_W => {
          eval += 30;
          eval += N_VALUE_MAP[position];
        }
        B_W => {
          eval += 30;
          eval += B_VALUE_MAP[position];
        }
        Q_W => {
          eval += 90;
          eval += Q_VALUE_MAP[position];
        }
        K_W => {
          eval += 1000;
          eval += K_VALUE_MAP[position];
        }
        P_W => {
          eval += 10;
          eval += P_VALUE_MAP[position];
        }
        _ => (),
      }
    }
    eval
  }
  fn does_w_attack(&self, position: usize) -> bool {
    if CKNM17[position] {
      if self.board[position + 17] == KN_W {
        return true;
      }
    }
    if CKNM15[position] {
      if self.board[position + 15] == KN_W {
        return true;
      }
    }
    if CKNM10[position] {
      if self.board[position + 10] == KN_W {
        return true;
      }
    }
    if CKNM6[position] {
      if self.board[position + 6] == KN_W {
        return true;
      }
    }
    if CKNMN17[position] {
      if self.board[position - 17] == KN_W {
        return true;
      }
    }
    if CKNMN15[position] {
      if self.board[position - 15] == KN_W {
        return true;
      }
    }
    if CKNMN10[position] {
      if self.board[position - 10] == KN_W {
        return true;
      }
    }
    if CKNMN6[position] {
      if self.board[position - 6] == KN_W {
        return true;
      }
    }
    for i in 0..7 {
      if CRM8[position + i * 8] {
        if self.board[position + 8 + i * 8] == R_W || self.board[position + 8 + i * 8] == Q_W {
          return true;
        } else if self.board[position + 8 + i * 8] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CRM1[position + i] {
        if self.board[position + 1 + i] == R_W || self.board[position + 1 + i] == Q_W {
          return true;
        } else if self.board[position + 1 + i] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CRMN8[position - i * 8] {
        if self.board[position - 8 - i * 8] == R_W || self.board[position - 8 - i * 8] == Q_W {
          return true;
        } else if self.board[position - 8 - i * 8] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CRMN8[position - i] {
        if self.board[position - 1 - i] == R_W || self.board[position - 1 - i] == Q_W {
          return true;
        } else if self.board[position - 1 - i] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CBM9[position + i * 9] {
        if self.board[position + 9 + i * 9] == B_W || self.board[position + 9 + i * 9] == Q_W {
          return true;
        } else if self.board[position + 9 + i * 9] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CBM7[position + i * 7] {
        if self.board[position + 7 + i * 7] == B_W || self.board[position + 7 + i * 7] == Q_W {
          return true;
        } else if self.board[position + 7 + i * 7] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CBMN9[position - i * 9] {
        if self.board[position - 9 - i * 9] == B_W || self.board[position - 9 - i * 9] == Q_W {
          return true;
        } else if self.board[position - 9 - i * 9] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CBMN7[position - i * 7] {
        if self.board[position - 7 - i * 7] == B_W || self.board[position - 7 - i * 7] == Q_W {
          return true;
        } else if self.board[position - 7 - i * 7] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    if CBM9[position] {
      if self.board[position + 9] == K_W {
        return true;
      }
    }
    if CBM7[position] {
      if self.board[position + 7] == K_W {
        return true;
      }
    }
    if CBMN9[position] {
      if self.board[position - 9] == K_W {
        return true;
      }
    }
    if CBMN7[position] {
      if self.board[position - 7] == K_W {
        return true;
      }
    }
    if CRM8[position] {
      if self.board[position + 8] == K_W {
        return true;
      }
    }
    if CRM1[position] {
      if self.board[position + 1] == K_W {
        return true;
      }
    }
    if CRMN8[position] {
      if self.board[position - 8] == K_W {
        return true;
      }
    }
    if CRMN1[position] {
      if self.board[position - 1] == K_W {
        return true;
      }
    }
    if CBMN9[position] {
      if self.board[position - 9] == P_W {
        return true;
      }
    }
    if CBMN7[position] {
      if self.board[position - 7] == P_W {
        return true;
      }
    }
    false
  }
  fn does_b_attack(&self, position: usize) -> bool {
    if CKNM17[position] {
      if self.board[position + 17] == KN_B {
        return true;
      }
    }
    if CKNM15[position] {
      if self.board[position + 15] == KN_B {
        return true;
      }
    }
    if CKNM10[position] {
      if self.board[position + 10] == KN_B {
        return true;
      }
    }
    if CKNM6[position] {
      if self.board[position + 6] == KN_B {
        return true;
      }
    }
    if CKNMN17[position] {
      if self.board[position - 17] == KN_B {
        return true;
      }
    }
    if CKNMN15[position] {
      if self.board[position - 15] == KN_B {
        return true;
      }
    }
    if CKNMN10[position] {
      if self.board[position - 10] == KN_B {
        return true;
      }
    }
    if CKNMN6[position] {
      if self.board[position - 6] == KN_B {
        return true;
      }
    }
    for i in 0..7 {
      if CRM8[position + i * 8] {
        if self.board[position + 8 + i * 8] == R_B || self.board[position + 8 + i * 8] == Q_B {
          return true;
        } else if self.board[position + 8 + i * 8] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CRM1[position + i] {
        if self.board[position + 1 + i] == R_B || self.board[position + 1 + i] == Q_B {
          return true;
        } else if self.board[position + 1 + i] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CRMN8[position - i * 8] {
        if self.board[position - 8 - i * 8] == R_B || self.board[position - 8 - i * 8] == Q_B {
          return true;
        } else if self.board[position - 8 - i * 8] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CRMN8[position - i] {
        if self.board[position - 1 - i] == R_B || self.board[position - 1 - i] == Q_B {
          return true;
        } else if self.board[position - 1 - i] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CBM9[position + i * 9] {
        if self.board[position + 9 + i * 9] == B_B || self.board[position + 9 + i * 9] == Q_B {
          return true;
        } else if self.board[position + 9 + i * 9] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CBM7[position + i * 7] {
        if self.board[position + 7 + i * 7] == B_B || self.board[position + 7 + i * 7] == Q_B {
          return true;
        } else if self.board[position + 7 + i * 7] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CBMN9[position - i * 9] {
        if self.board[position - 9 - i * 9] == B_B || self.board[position - 9 - i * 9] == Q_B {
          return true;
        } else if self.board[position - 9 - i * 9] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    for i in 0..7 {
      if CBMN7[position - i * 7] {
        if self.board[position - 7 - i * 7] == B_B || self.board[position - 7 - i * 7] == Q_B {
          return true;
        } else if self.board[position - 7 - i * 7] == 0 {
        } else {
          break;
        }
      } else {
        break;
      }
    }
    if CBM9[position] {
      if self.board[position + 9] == K_B {
        return true;
      }
    }
    if CBM7[position] {
      if self.board[position + 7] == K_B {
        return true;
      }
    }
    if CBMN9[position] {
      if self.board[position - 9] == K_B {
        return true;
      }
    }
    if CBMN7[position] {
      if self.board[position - 7] == K_B {
        return true;
      }
    }
    if CRM8[position] {
      if self.board[position + 8] == K_B {
        return true;
      }
    }
    if CRM1[position] {
      if self.board[position + 1] == K_B {
        return true;
      }
    }
    if CRMN8[position] {
      if self.board[position - 8] == K_B {
        return true;
      }
    }
    if CRMN1[position] {
      if self.board[position - 1] == K_B {
        return true;
      }
    }
    if CBM9[position] {
      if self.board[position + 9] == P_B {
        return true;
      }
    }
    if CBM7[position] {
      if self.board[position + 7] == P_B {
        return true;
      }
    }
    false
  }
  fn find_moves(&self) -> [i32; 512] {
    let mut moves: [i32; 512] = [EMPTY_PLACEHOLDER_NUM; 512];
    if self.checkmate {
      return moves;
    }
    let mut movesindex: usize = 0;
    if self.turn {
      // is it whites turn
      for x in 0..64 {
        let piece = self.board[x];

        match piece {
          R_W => {
            for i in 0..7 {
              if CRM8[x + i * 8] {
                if self.board[x + 8 + i * 8] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                  movesindex += 2
                } else if self.board[x + 8 + i * 8] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRM1[x + i] {
                if self.board[x + 1 + i] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 1 + i as i32;
                  movesindex += 2
                } else if self.board[x + 1 + i] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 1 + i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRMN8[x - i * 8] {
                if self.board[x - 8 - i * 8] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                  movesindex += 2
                } else if self.board[x - 8 - i * 8] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRMN1[x - i] {
                if self.board[x - 1 - i] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 1 - i as i32;
                  movesindex += 2
                } else if self.board[x - 1 - i] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 1 - i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
          }
          B_W => {
            for i in 0..7 {
              if CBM9[x + i * 9] {
                if self.board[x + 9 + i * 9] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                  movesindex += 2
                } else if self.board[x + 9 + i * 9] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBM7[x + i * 7] {
                if self.board[x + 7 + i * 7] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                  movesindex += 2
                } else if self.board[x + 7 + i * 7] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBMN9[x - i * 9] {
                if self.board[x - 9 - i * 9] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                  movesindex += 2
                } else if self.board[x - 9 - i * 9] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBMN7[x - i * 7] {
                if self.board[x - 7 - i * 7] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                  movesindex += 2
                } else if self.board[x - 7 - i * 7] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
          }
          Q_W => {
            for i in 0..7 {
              if CRM8[x + i * 8] {
                if self.board[x + 8 + i * 8] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                  movesindex += 2
                } else if self.board[x + 8 + i * 8] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRM1[x + i] {
                if self.board[x + 1 + i] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 1 + i as i32;
                  movesindex += 2
                } else if self.board[x + 1 + i] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 1 + i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRMN8[x - i * 8] {
                if self.board[x - 8 - i * 8] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                  movesindex += 2
                } else if self.board[x - 8 - i * 8] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRMN1[x - i] {
                if self.board[x - 1 - i] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 1 - i as i32;
                  movesindex += 2
                } else if self.board[x - 1 - i] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 1 - i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBM9[x + i * 9] {
                if self.board[x + 9 + i * 9] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                  movesindex += 2
                } else if self.board[x + 9 + i * 9] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBM7[x + i * 7] {
                if self.board[x + 7 + i * 7] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                  movesindex += 2
                } else if self.board[x + 7 + i * 7] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBMN9[x - i * 9] {
                if self.board[x - 9 - i * 9] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                  movesindex += 2
                } else if self.board[x - 9 - i * 9] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBMN7[x - i * 7] {
                if self.board[x - 7 - i * 7] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                  movesindex += 2
                } else if self.board[x - 7 - i * 7] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
          }
          KN_W => {
            if CKNM17[x] {
              if self.board[x + 17] == 0 || self.board[x + 17] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 17;
                movesindex += 2
              }
            }
            if CKNM15[x] {
              if self.board[x + 15] == 0 || self.board[x + 15] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 15;
                movesindex += 2
              }
            }
            if CKNM10[x] {
              if self.board[x + 10] == 0 || self.board[x + 10] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 10;
                movesindex += 2
              }
            }
            if CKNM6[x] {
              if self.board[x + 6] == 0 || self.board[x + 6] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 6;
                movesindex += 2
              }
            }
            if CKNMN17[x] {
              if self.board[x - 17] == 0 || self.board[x - 17] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 17;
                movesindex += 2
              }
            }
            if CKNMN15[x] {
              if self.board[x - 15] == 0 || self.board[x - 15] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 15;
                movesindex += 2
              }
            }
            if CKNMN10[x] {
              if self.board[x - 10] == 0 || self.board[x - 10] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 10;
                movesindex += 2
              }
            }
            if CKNMN6[x] {
              if self.board[x - 6] == 0 || self.board[x - 6] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 6;
                movesindex += 2
              }
            }
          }
          K_W => {
            if CBM9[x] && self.does_b_attack(x + 9) == false {
              if self.board[x + 9] == 0 || self.board[x + 9] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 9;
                movesindex += 2
              }
            }
            if CBM7[x] && self.does_b_attack(x + 7) == false {
              if self.board[x + 7] == 0 || self.board[x + 7] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 7;
                movesindex += 2
              }
            }
            if CBMN9[x] && self.does_b_attack(x - 9) == false {
              if self.board[x - 9] == 0 || self.board[x - 9] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 9;
                movesindex += 2
              }
            }
            if CBMN7[x] && self.does_b_attack(x - 7) == false {
              if self.board[x - 7] == 0 || self.board[x - 7] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 7;
                movesindex += 2
              }
            }
            if CRM8[x] && self.does_b_attack(x + 8) == false {
              if self.board[x + 8] == 0 || self.board[x + 8] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 8;
                movesindex += 2
              }
            }
            if CRM1[x] && self.does_b_attack(x + 1) == false {
              if self.board[x + 1] == 0 || self.board[x + 1] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 1;
                movesindex += 2
              }
            }
            if CRMN8[x] && self.does_b_attack(x - 8) == false {
              if self.board[x - 8] == 0 || self.board[x - 8] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 8;
                movesindex += 2
              }
            }
            if CRMN1[x] && self.does_b_attack(x - 1) == false {
              if self.board[x - 1] == 0 || self.board[x - 1] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 1;
                movesindex += 2
              }
            }
            if self.crights[0]
              && self.does_b_attack(61) == false
              && self.board[61] == 0
              && self.board[62] == 0
            {
              moves[movesindex] = x as i32;
              moves[movesindex + 1] = x as i32 + 2;
              movesindex += 2
            }
            if self.crights[1]
              && self.does_b_attack(59) == false
              && self.board[59] == 0
              && self.board[58] == 0
              && self.board[57] == 0
            {
              moves[movesindex] = x as i32;
              moves[movesindex + 1] = x as i32 - 2;
              movesindex += 2
            }
          }
          P_W => {
            if CPMN16[x] {
              if self.board[x - 8] == 0 && self.board[x - 16] == 0 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 16;
                movesindex += 2
              }
            }
            if CPM16[x] == false && self.board[x - 8] == 0 {
              moves[movesindex] = x as i32;
              moves[movesindex + 1] = x as i32 - 8;
              movesindex += 2
            }
            if CPM16[x] == false && CBMN9[x] {
              if self.board[x - 9] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 9;
                movesindex += 2
              }
            }
            if CPM16[x] == false && CBMN7[x] {
              if self.board[x - 7] > OTHER_COLOR_PIECE {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 7;
                movesindex += 2
              }
            }
            if CBMN9[x] {
              if x - 9 == self.epts {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 9;
                movesindex += 2
              }
            }
            if CBMN7[x] {
              if x - 7 == self.epts {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 7;
                movesindex += 2
              }
            }
            if CPM16[x] {
              if self.board[x - 8] == 0 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 8 + 400;
                movesindex += 2
              }
              if CBMN9[x] {
                if self.board[x - 9] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 + 400;
                  movesindex += 2
                }
              }
              if CBMN7[x] {
                if self.board[x - 7] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 + 400;
                  movesindex += 2
                }
              }
              if self.board[x - 8] == 0 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 8 + 300;
                movesindex += 2
              }
              if CBMN9[x] {
                if self.board[x - 9] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 + 300;
                  movesindex += 2
                }
              }
              if CBMN7[x] {
                if self.board[x - 7] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 + 300;
                  movesindex += 2
                }
              }
              if self.board[x - 8] == 0 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 8 + 200;
                movesindex += 2
              }
              if CBMN9[x] {
                if self.board[x - 9] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 + 200;
                  movesindex += 2
                }
              }
              if CBMN7[x] {
                if self.board[x - 7] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 + 200;
                  movesindex += 2
                }
              }
              if self.board[x - 8] == 0 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 8 + 100;
                movesindex += 2
              }
              if CBMN9[x] {
                if self.board[x - 9] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 + 100;
                  movesindex += 2
                }
              }
              if CBMN7[x] {
                if self.board[x - 7] > OTHER_COLOR_PIECE {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 + 100;
                  movesindex += 2
                }
              }
            }
          }
          _ => (),
        }
      }
    } else {
      // it is blacks turn
      for x in 0..64 {
        let piece = self.board[x];

        match piece {
          R_B => {
            for i in 0..7 {
              if CRM8[x + i * 8] {
                if self.board[x + 8 + i * 8] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                  movesindex += 2
                } else if self.board[x + 8 + i * 8] < OTHER_COLOR_PIECE
                  && self.board[x + 8 + i * 8] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRM1[x + i] {
                if self.board[x + 1 + i] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 1 + i as i32;
                  movesindex += 2
                } else if self.board[x + 1 + i] < OTHER_COLOR_PIECE && self.board[x + 1 + i] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 1 + i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRMN8[x - i * 8] {
                if self.board[x - 8 - i * 8] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                  movesindex += 2
                } else if self.board[x - 8 - i * 8] < OTHER_COLOR_PIECE
                  && self.board[x - 8 - i * 8] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRMN1[x - i] {
                if self.board[x - 1 - i] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 1 - i as i32;
                  movesindex += 2
                } else if self.board[x - 1 - i] < OTHER_COLOR_PIECE && self.board[x - 1 - i] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 1 - i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
          }
          B_B => {
            for i in 0..7 {
              if CBM9[x + i * 9] {
                if self.board[x + 9 + i * 9] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                  movesindex += 2
                } else if self.board[x + 9 + i * 9] < OTHER_COLOR_PIECE
                  && self.board[x + 9 + i * 9] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBM7[x + i * 7] {
                if self.board[x + 7 + i * 7] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                  movesindex += 2
                } else if self.board[x + 7 + i * 7] < OTHER_COLOR_PIECE
                  && self.board[x + 7 + i * 7] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBMN9[x - i * 9] {
                if self.board[x - 9 - i * 9] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                  movesindex += 2
                } else if self.board[x - 9 - i * 9] < OTHER_COLOR_PIECE
                  && self.board[x - 9 - i * 9] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBMN7[x - i * 7] {
                if self.board[x - 7 - i * 7] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                  movesindex += 2
                } else if self.board[x - 7 - i * 7] < OTHER_COLOR_PIECE
                  && self.board[x - 7 - i * 7] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
          }
          Q_B => {
            for i in 0..7 {
              if CRM8[x + i * 8] {
                if self.board[x + 8 + i * 8] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                  movesindex += 2
                } else if self.board[x + 8 + i * 8] < OTHER_COLOR_PIECE
                  && self.board[x + 8 + i * 8] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRM1[x + i] {
                if self.board[x + 1 + i] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 1 + i as i32;
                  movesindex += 2
                } else if self.board[x + 1 + i] < OTHER_COLOR_PIECE && self.board[x + 1 + i] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 1 + i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRMN8[x - i * 8] {
                if self.board[x - 8 - i * 8] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                  movesindex += 2
                } else if self.board[x - 8 - i * 8] < OTHER_COLOR_PIECE
                  && self.board[x - 8 - i * 8] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CRMN1[x - i] {
                if self.board[x - 1 - i] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 1 - i as i32;
                  movesindex += 2
                } else if self.board[x - 1 - i] < OTHER_COLOR_PIECE && self.board[x - 1 - i] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 1 - i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBM9[x + i * 9] {
                if self.board[x + 9 + i * 9] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                  movesindex += 2
                } else if self.board[x + 9 + i * 9] < OTHER_COLOR_PIECE
                  && self.board[x + 9 + i * 9] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBM7[x + i * 7] {
                if self.board[x + 7 + i * 7] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                  movesindex += 2
                } else if self.board[x + 7 + i * 7] < OTHER_COLOR_PIECE
                  && self.board[x + 7 + i * 7] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBMN9[x - i * 9] {
                if self.board[x - 9 - i * 9] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                  movesindex += 2
                } else if self.board[x - 9 - i * 9] < OTHER_COLOR_PIECE
                  && self.board[x - 9 - i * 9] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
            for i in 0..7 {
              if CBMN7[x - i * 7] {
                if self.board[x - 7 - i * 7] == 0 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                  movesindex += 2
                } else if self.board[x - 7 - i * 7] < OTHER_COLOR_PIECE
                  && self.board[x - 7 - i * 7] > 65
                {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                  movesindex += 2;
                  break;
                } else {
                  break;
                }
              } else {
                break;
              }
            }
          }
          KN_B => {
            if CKNM17[x] {
              if self.board[x + 17] == 0
                || self.board[x + 17] < OTHER_COLOR_PIECE && self.board[x + 17] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 17;
                movesindex += 2
              }
            }
            if CKNM15[x] {
              if self.board[x + 15] == 0
                || self.board[x + 15] < OTHER_COLOR_PIECE && self.board[x + 15] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 15;
                movesindex += 2
              }
            }
            if CKNM10[x] {
              if self.board[x + 10] == 0
                || self.board[x + 10] < OTHER_COLOR_PIECE && self.board[x + 10] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 10;
                movesindex += 2
              }
            }
            if CKNM6[x] {
              if self.board[x + 6] == 0
                || self.board[x + 6] < OTHER_COLOR_PIECE && self.board[x + 6] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 6;
                movesindex += 2
              }
            }
            if CKNMN17[x] {
              if self.board[x - 17] == 0
                || self.board[x - 17] < OTHER_COLOR_PIECE && self.board[x - 17] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 17;
                movesindex += 2
              }
            }
            if CKNMN15[x] {
              if self.board[x - 15] == 0
                || self.board[x - 15] < OTHER_COLOR_PIECE && self.board[x - 15] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 15;
                movesindex += 2
              }
            }
            if CKNMN10[x] {
              if self.board[x - 10] == 0
                || self.board[x - 10] < OTHER_COLOR_PIECE && self.board[x - 10] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 10;
                movesindex += 2
              }
            }
            if CKNMN6[x] {
              if self.board[x - 6] == 0
                || self.board[x - 6] < OTHER_COLOR_PIECE && self.board[x - 6] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 6;
                movesindex += 2
              }
            }
          }
          K_B => {
            if CBM9[x] && self.does_w_attack(x + 9) == false {
              if self.board[x + 9] == 0
                || self.board[x + 9] < OTHER_COLOR_PIECE && self.board[x + 9] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 9;
                movesindex += 2
              }
            }
            if CBM7[x] && self.does_w_attack(x + 7) == false {
              if self.board[x + 7] == 0
                || self.board[x + 7] < OTHER_COLOR_PIECE && self.board[x + 7] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 7;
                movesindex += 2
              }
            }
            if CBMN9[x] && self.does_w_attack(x - 9) == false {
              if self.board[x - 9] == 0
                || self.board[x - 9] < OTHER_COLOR_PIECE && self.board[x - 9] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 9;
                movesindex += 2
              }
            }
            if CBMN7[x] && self.does_w_attack(x - 7) == false {
              if self.board[x - 7] == 0
                || self.board[x - 7] < OTHER_COLOR_PIECE && self.board[x - 7] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 7;
                movesindex += 2
              }
            }
            if CRM8[x] && self.does_w_attack(x + 8) == false {
              if self.board[x + 8] == 0
                || self.board[x + 8] < OTHER_COLOR_PIECE && self.board[x + 8] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 8;
                movesindex += 2
              }
            }
            if CRM1[x] && self.does_w_attack(x + 1) == false {
              if self.board[x + 1] == 0
                || self.board[x + 1] < OTHER_COLOR_PIECE && self.board[x + 1] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 1;
                movesindex += 2
              }
            }
            if CRMN8[x] && self.does_w_attack(x - 8) == false {
              if self.board[x - 8] == 0
                || self.board[x - 8] < OTHER_COLOR_PIECE && self.board[x - 8] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 8;
                movesindex += 2
              }
            }
            if CRMN1[x] && self.does_w_attack(x - 1) == false {
              if self.board[x - 1] == 0
                || self.board[x - 1] < OTHER_COLOR_PIECE && self.board[x - 1] > 65
              {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 - 1;
                movesindex += 2
              }
            }
            if self.crights[2]
              && self.does_w_attack(5) == false
              && self.board[5] == 0
              && self.board[6] == 0
            {
              moves[movesindex] = x as i32;
              moves[movesindex + 1] = x as i32 + 2;
              movesindex += 2
            }
            if self.crights[3]
              && self.does_w_attack(3) == false
              && self.board[3] == 0
              && self.board[2] == 0
              && self.board[1] == 0
            {
              moves[movesindex] = x as i32;
              moves[movesindex + 1] = x as i32 - 2;
              movesindex += 2
            }
          }
          P_B => {
            if CPM16[x] {
              if self.board[x + 8] == 0 && self.board[x + 16] == 0 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 16;
                movesindex += 2
              }
            }
            if CPMN16[x] == false && self.board[x + 8] == 0 {
              moves[movesindex] = x as i32;
              moves[movesindex + 1] = x as i32 + 8;
              movesindex += 2
            }
            if CPMN16[x] == false && CBM9[x] {
              if self.board[x + 9] < OTHER_COLOR_PIECE && self.board[x + 9] > 65 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 9;
                movesindex += 2
              }
            }
            if CPMN16[x] == false && CBM7[x] {
              if self.board[x + 7] < OTHER_COLOR_PIECE && self.board[x + 7] > 65 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 7;
                movesindex += 2
              }
            }
            if CBM9[x] {
              if x + 9 == self.epts {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 9;
                movesindex += 2
              }
            }
            if CBM7[x] {
              if x + 7 == self.epts {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 7;
                movesindex += 2
              }
            }
            if CPMN16[x] {
              if self.board[x + 8] == 0 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 8 + 400;
                movesindex += 2
              }
              if CBM9[x] {
                if self.board[x + 9] < OTHER_COLOR_PIECE && self.board[x + 9] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 400;
                  movesindex += 2
                }
              }
              if CBM7[x] {
                if self.board[x + 7] < OTHER_COLOR_PIECE && self.board[x + 7] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 400;
                  movesindex += 2
                }
              }
              if self.board[x + 8] == 0 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 8 + 300;
                movesindex += 2
              }
              if CBM9[x] {
                if self.board[x + 9] < OTHER_COLOR_PIECE && self.board[x + 9] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 300;
                  movesindex += 2
                }
              }
              if CBM7[x] {
                if self.board[x + 7] < OTHER_COLOR_PIECE && self.board[x + 7] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 300;
                  movesindex += 2
                }
              }
              if self.board[x + 8] == 0 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 8 + 200;
                movesindex += 2
              }
              if CBM9[x] {
                if self.board[x + 9] < OTHER_COLOR_PIECE && self.board[x + 9] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 200;
                  movesindex += 2
                }
              }
              if CBM7[x] {
                if self.board[x + 7] < OTHER_COLOR_PIECE && self.board[x + 7] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 200;
                  movesindex += 2
                }
              }
              if self.board[x + 8] == 0 {
                moves[movesindex] = x as i32;
                moves[movesindex + 1] = x as i32 + 8 + 100;
                movesindex += 2
              }
              if CBM9[x] {
                if self.board[x + 9] < OTHER_COLOR_PIECE && self.board[x + 9] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 9 + 100;
                  movesindex += 2
                }
              }
              if CBM7[x] {
                if self.board[x + 7] < OTHER_COLOR_PIECE && self.board[x + 7] > 65 {
                  moves[movesindex] = x as i32;
                  moves[movesindex + 1] = x as i32 + 7 + 100;
                  movesindex += 2
                }
              }
            }
          }
          _ => (),
        }
      }
    }
    moves
  }
}

#[derive(Copy, Clone)]
struct Node {
  parent_key: i32,
  board: Board,
  value: Option<i32>,
  move1: usize,
  move2: usize,
  depth: usize,
}

impl Node {
  fn make_node_tree(self, searchdepth: usize) -> HashMap<i32, Node> {
    let mut all_nodes = HashMap::new();
    all_nodes.insert(0, self); //this will be the startnode
    for depth in 0..searchdepth {
      let mut node_i: i32 = 0;
      let mut j: i32 = 0;
      loop {
        match all_nodes.get(&(node_i * 10 + depth as i32)) {
          Some(&node) => {
            let mut i: usize = 0;
            let all_moves = node.board.find_moves();
            if all_moves[0] == EMPTY_PLACEHOLDER_NUM {
              //if all_moves[0] == EMPTY_PLACEHOLDER_NUM there are no legal moves, thus a nonexpanding branch is created
              all_nodes.insert(
                j * 10 + depth as i32 + 1,
                Node {
                  parent_key: node_i * 10 + depth as i32,
                  board: node.board,
                  value: None,
                  move1: all_moves[i] as usize,
                  move2: all_moves[i + 1] as usize,
                  depth: depth + 1,
                },
              );
              j += 1;
            }
            while all_moves[i] != EMPTY_PLACEHOLDER_NUM {
              let mut board_copy = node.board;

              board_copy.make_move(all_moves[i] as usize, all_moves[i + 1] as usize);
              all_nodes.insert(
                j * 10 + depth as i32 + 1,
                Node {
                  parent_key: node_i * 10 + depth as i32,
                  board: board_copy,
                  value: None,
                  move1: all_moves[i] as usize,
                  move2: all_moves[i + 1] as usize,
                  depth: depth + 1,
                },
              );

              j += 1;
              i += 2;
            }
          }
          _ => (break),
        }
        node_i += 1;
      }
    }
    all_nodes
  }
  fn search_node_tree(&self, searchdepth: usize, mut all_nodes: HashMap<i32, Node>) {
    let mut rustychess_to_standard = HashMap::new();
    for i in 0..128 {
      rustychess_to_standard.insert(
        POSITION_CONVERTER_RUSTYCHESS[i],
        POSTION_CONVERTER_STANDARD[i],
      );
    }
    for b_depth in 0..searchdepth {
      let depth = searchdepth - b_depth;
      let mut node_i: i32 = 0;
      if depth == 1 {
        break;
      }
      loop {
        match all_nodes.get_mut(&(node_i * 10 + depth as i32)) {
          Some(node) => {
            if node.depth == searchdepth {
              node.value = Some(node.board.value());
            }
            let parent_key = node.parent_key;
            let node_value = node.value;
            if let Some(p_node) = all_nodes.get_mut(&parent_key) {
              if p_node.value == None {
                p_node.value = node_value;
              } else if p_node.board.turn {
                //whites turn =>maximizing player
                if node_value > (p_node.value) {
                  p_node.value = node_value;
                }
              } else if p_node.board.turn == false {
                //blacks turn =>minimizing player
                if node_value < p_node.value {
                  p_node.value = node_value;
                }
              }
            }
          }
          _ => (break),
        }
        node_i += 1;
      }
    }
    let depth: usize = 1;
    let mut node_i: i32 = 0;
    let mut bestmovevalue: i32 = 0;
    let mut bestmove1: usize = EMPTY_PLACEHOLDER_NUM as usize;
    let mut bestmove2: usize = EMPTY_PLACEHOLDER_NUM as usize;
    loop {
      match all_nodes.get(&(node_i * 10 + depth as i32)) {
        Some(&node) => {
          if bestmove1 == EMPTY_PLACEHOLDER_NUM as usize {
            match node.value {
              Some(x) => bestmovevalue = x,
              None => (),
            }
            bestmove1 = node.move1;
            bestmove2 = node.move2;
          } else if node.board.turn == false {
            //blacks turn next turn => whites turn => maximizing player
            if node.value > Some(bestmovevalue) {
              match node.value {
                Some(x) => bestmovevalue = x,
                None => (),
              }
              bestmove1 = node.move1;
              bestmove2 = node.move2;
            }
          } else if node.board.turn {
            //whites turn next turn => blacks turn => minimizing player
            if node.value < Some(bestmovevalue) {
              match node.value {
                Some(x) => bestmovevalue = x,
                None => (),
              }
              bestmove1 = node.move1;
              bestmove2 = node.move2;
            }
          }
        }
        _ => (break),
      }
      node_i += 1;
    }
    println!(
      "bestmove {}{}",
      match rustychess_to_standard.get(&bestmove1) {
        Some(x) => x,
        _ => (""),
      },
      match rustychess_to_standard.get(&bestmove2) {
        Some(x) => x,
        _ => (""),
      }
    );
    log::info!(
      "out --> bestmove {}{}",
      match rustychess_to_standard.get(&bestmove1) {
        Some(x) => x,
        _ => (""),
      },
      match rustychess_to_standard.get(&bestmove2) {
        Some(x) => x,
        _ => (""),
      }
    );
  }
}

fn main() {
  let mut my_board: Board = Board {
    ..Default::default()
  };
  simple_logging::log_to_file("test.log", LevelFilter::Info);
  let mut standard_to_rustychess = HashMap::new();
  for i in 0..128 {
    standard_to_rustychess.insert(
      POSTION_CONVERTER_STANDARD[i],
      POSITION_CONVERTER_RUSTYCHESS[i],
    );
  }

  fn uci_id() {
    let name = "Rustychess";
    let author = "Linus Gustafsson";
    println!("id name {}\nid author {}", name, author);
    log::info!("out --> id name {}\nid author {}", name, author);
  }
  fn initialize_uci() {
    println!("uciok");
    log::info!("out --> uciok");
  }
  fn initialize_engine() {
    println!("readyok");
    log::info!("out --> readyok");
  }
  fn uci_new_game() {}
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    //println!("{}", line.unwrap());
    let text = line.unwrap();
    log::info!("in --> {}", text);
    let first_word = text.split_whitespace().next().unwrap_or("");
    if text == "uci" {
      uci_id();
      initialize_uci();
    }
    if text == "isready" {
      initialize_engine();
    }
    if text == "ucinewgame" {
      my_board = Board {
        ..Default::default()
      };
      uci_new_game();
    }
    if text == "position startpos" {
      my_board.load_fen(FEN_STARTPOSITION);
    }
    if text.starts_with("position startpos moves ") {
      my_board = Board {
        ..Default::default()
      };
      my_board.load_fen(FEN_STARTPOSITION);
      let split = text.split_whitespace();
      let textsplit: Vec<&str> = split.collect();
      for x in 3..textsplit.len() {
        let move1 = &textsplit[x][..2];
        let move2 = &textsplit[x][2..];
        my_board.make_move(
          match standard_to_rustychess.get(&move1) {
            Some(x) => *x,
            _ => (0),
          },
          match standard_to_rustychess.get(&move2) {
            Some(x) => *x,
            _ => (0),
          },
        );
      }
    }
    if text == "quit" {
      process::exit(1);
    }
    if first_word == "go" {
      let start_node: Node = Node {
        parent_key: 0,
        board: my_board,
        value: None,
        move1: 0,
        move2: 0,
        depth: 0,
      };
      let all_nodes = start_node.make_node_tree(4);
      start_node.search_node_tree(4, all_nodes);
    }
  }
}
