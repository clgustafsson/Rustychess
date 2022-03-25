use crate::board::*;
use crate::constants::*;

pub fn promote(board: &mut Board, from: usize, to: &mut usize) {
    if *to >= 400 {
        *to -= 400;
        if board.turn {
            board.board[from] = Q_W;
        } else {
            board.board[from] = Q_B;
        }
    } else if *to >= 300 {
        *to -= 300;
        if board.turn {
            board.board[from] = R_W;
        } else {
            board.board[from] = R_B;
        }
    } else if *to >= 200 {
        *to -= 200;
        if board.turn {
            board.board[from] = B_W;
        } else {
            board.board[from] = B_B;
        }
    } else if *to >= 100 {
        *to -= 100;
        if board.turn {
            board.board[from] = KN_W;
        } else {
            board.board[from] = KN_B;
        }
    }
}
pub fn update_move_info(
    board: &mut Board,
    from: usize,
    to: usize,
    is_kingmove: &mut bool,
    is_kcastle: &mut bool,
    is_qcastle: &mut bool,
    is_doublepawnmove: &mut bool,
    is_capture: &mut bool,
    is_pawnmove: &mut bool,
    is_enpassant: &mut bool,
) {
    if board.board[from] == K_W || board.board[from] == K_B {
        *is_kingmove = true;
        if to as i8 - from as i8 == 2 {
            *is_kcastle = true;
        }
        if from as i8 - to as i8 == 2 {
            *is_qcastle = true;
        }
    }
    if board.board[from] == P_W && from - to == 16 || board.board[from] == P_B && to - from == 16 {
        *is_doublepawnmove = true;
    }
    if board.board[to] != 0 {
        *is_capture = true
    }
    if board.board[from] == P_W || board.board[from] == P_B {
        *is_pawnmove = true;
        if to == board.epts {
            *is_enpassant = true;
        }
    }
    if board.board[to] == K_W || board.board[to] == K_B {
        board.checkmate = true
    }
}
pub fn update_board(
    board: &mut Board,
    from: usize,
    to: usize,
    is_kcastle: bool,
    is_qcastle: bool,
    is_enpassant: bool,
) {
    if is_kcastle {
        board.board[from + 1] = board.board[to + 1];
        board.board[to + 1] = 0;
    }
    if is_qcastle {
        board.board[from - 1] = board.board[to - 2];
        board.board[to - 2] = 0;
    }
    if is_enpassant {
        if board.turn {
            board.board[to + 8] = 0;
        } else {
            board.board[to - 8] = 0;
        }
    }
    board.board[to] = board.board[from];
    board.board[from] = 0;
}
pub fn update_turn(board: &mut Board) {
    if board.turn {
        board.turn = false
    } else {
        board.turn = true;
        board.fullmove += 1
    }
}
pub fn update_halfmove(board: &mut Board, is_capture: bool, is_pawnmove: bool) {
    if is_capture == false && is_pawnmove == false {
        board.halfmove += 1
    }
}
pub fn update_epts(board: &mut Board, from: usize, to: usize, is_doublepawnmove: bool) {
    if is_doublepawnmove {
        let epts: usize = from + to;
        board.epts = epts / 2
    } else {
        board.epts = 100
    }
}
pub fn update_crights(board: &mut Board, from: usize, to: usize, is_kingmove: bool) {
    if is_kingmove {
        if board.turn {
            board.crights[0] = false;
            board.crights[1] = false;
        } else {
            board.crights[2] = false;
            board.crights[3] = false;
        }
    }
    if from == 63 || to == 63 {
        board.crights[0] = false;
    }
    if from == 56 || to == 56 {
        board.crights[1] = false;
    }
    if from == 7 || to == 7 {
        board.crights[2] = false;
    }
    if from == 0 || to == 0 {
        board.crights[3] = false;
    }
}
