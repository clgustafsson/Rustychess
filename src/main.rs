#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use crate::board::*;
use crate::constants::*;
use crate::fen::*;
use crate::find_attack_functions::*;
use crate::find_move_functions::*;
use crate::make_move::*;
use crate::uci::*;
use crate::value::*;
use log::LevelFilter;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::process;

mod board;
mod constants;
mod fen;
mod find_attack_functions;
mod find_move_functions;
mod make_move;
mod uci;
mod value;

impl Board {
  fn load_fen(&mut self, fen: &str) {
    let mut i: usize = 0;
    let mut fen_i: usize = 0;
    for x in fen.as_bytes() {
      if x == &32 {
        fen_i += 1
      } else {
        match fen_i {
          0 => load_fen_board(self, &mut i, x),
          1 => load_fen_turn(self, x),
          2 => load_fen_crights(self, x),
          3 => load_fen_epts(self, x),
          4 => load_fen_halfmove(self, x),
          5 => load_fen_fullmove(self, x),
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
    promote(self, from, &mut to);
    update_move_info(
      self,
      from,
      to,
      &mut is_kingmove,
      &mut is_kcastle,
      &mut is_qcastle,
      &mut is_doublepawnmove,
      &mut is_capture,
      &mut is_pawnmove,
      &mut is_enpassant,
    );
    update_board(self, from, to, is_kcastle, is_qcastle, is_enpassant);
    update_turn(self);
    update_halfmove(self, is_capture, is_pawnmove);
    update_epts(self, from, to, is_doublepawnmove);
    update_crights(self, from, to, is_kingmove);
  }
  fn value(&self) -> i32 {
    let mut eval: i32 = 0;
    for position in 0..64 {
      match self.board[position] {
        R_B => value_r_b(&mut eval, position),
        KN_B => value_kn_b(&mut eval, position),
        B_B => value_b_b(&mut eval, position),
        Q_B => value_q_b(&mut eval, position),
        K_B => value_k_b(&mut eval, position),
        P_B => value_p_b(&mut eval, position),
        R_W => value_r_w(&mut eval, position),
        KN_W => value_kn_w(&mut eval, position),
        B_W => value_b_w(&mut eval, position),
        Q_W => value_q_w(&mut eval, position),
        K_W => value_k_w(&mut eval, position),
        P_W => value_p_w(&mut eval, position),
        _ => (),
      }
    }
    eval
  }
  fn does_w_attack(&self, position: usize) -> bool {
    if does_w_kn_attack(self, position) {
      return true;
    }
    if does_w_r_attack(self, position) {
      return true;
    }
    if does_w_b_attack(self, position) {
      return true;
    }
    if does_w_k_attack(self, position) {
      return true;
    }
    if does_w_p_attack(self, position) {
      return true;
    }
    false
  }
  fn does_b_attack(&self, position: usize) -> bool {
    if does_b_kn_attack(self, position) {
      return true;
    }
    if does_b_r_attack(self, position) {
      return true;
    }
    if does_b_b_attack(self, position) {
      return true;
    }
    if does_b_k_attack(self, position) {
      return true;
    }
    if does_b_p_attack(self, position) {
      return true;
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
      // it is whites turn
      for x in 0..64 {
        let piece = self.board[x];
        match piece {
          R_W => find_w_r_moves(self, &mut moves, &mut movesindex, x),
          B_W => find_w_b_moves(self, &mut moves, &mut movesindex, x),
          Q_W => {
            find_w_r_moves(self, &mut moves, &mut movesindex, x);
            find_w_b_moves(self, &mut moves, &mut movesindex, x);
          }
          KN_W => find_w_kn_moves(self, &mut moves, &mut movesindex, x),
          K_W => find_w_k_moves(self, &mut moves, &mut movesindex, x),
          P_W => find_w_p_moves(self, &mut moves, &mut movesindex, x),
          _ => (),
        }
      }
    } else {
      // it is blacks turn
      for x in 0..64 {
        let piece = self.board[x];
        match piece {
          R_B => find_b_r_moves(self, &mut moves, &mut movesindex, x),
          B_B => find_b_b_moves(self, &mut moves, &mut movesindex, x),
          Q_B => {
            find_b_r_moves(self, &mut moves, &mut movesindex, x);
            find_b_b_moves(self, &mut moves, &mut movesindex, x);
          }
          KN_B => find_b_kn_moves(self, &mut moves, &mut movesindex, x),
          K_B => find_b_k_moves(self, &mut moves, &mut movesindex, x),
          P_B => find_b_p_moves(self, &mut moves, &mut movesindex, x),
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
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
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
      uci_make_moves(&mut my_board, &text, &standard_to_rustychess)
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
