use crate::board::*;

pub fn load_fen_board(board: &mut Board, i: &mut usize, x: &u8) {
    if x > &48 && x < &57 {
        let new_num = x - 48; //If x is a 'number then this corresponds to that number
        *i += new_num as usize;
    }
    if x > &65 && x < &90 || x > &97 && x < &122 {
        board.board[*i] = *x;
        *i += 1;
    }
}
pub fn load_fen_turn(board: &mut Board, x: &u8) {
    if x == &119 {
        board.turn = true
    } else {
        board.turn = false
    }
}
pub fn load_fen_crights(board: &mut Board, x: &u8) {
    match x {
        75 => board.crights[0] = true,
        81 => board.crights[1] = true,
        107 => board.crights[2] = true,
        113 => board.crights[3] = true,
        _ => (),
    }
}
pub fn load_fen_epts(board: &mut Board, x: &u8) {
    if x > &96 {
        //filter out numbers and '-'
        if board.turn {
            let wepts = x - 81;
            board.epts = wepts as usize
        }
        //if whites turn epts will always be on 6th rank
        else {
            let bepts = x - 57;
            board.epts = bepts as usize
        }
    } //if blacks turn epts will always be on 3rd rank}
}
pub fn load_fen_halfmove(board: &mut Board, x: &u8) {
    //halfmove clock
    if board.halfmove == 0 {
        board.halfmove = x - 48
    } else {
        board.halfmove = board.halfmove * 10 + x - 48
    } //stringlike addition with number
}
pub fn load_fen_fullmove(board: &mut Board, x: &u8) {
    if board.fullmove == 1 {
        board.fullmove = x - 48
    } else {
        board.fullmove = board.fullmove * 10 + x - 48
    } //stringlike addition with number
}
