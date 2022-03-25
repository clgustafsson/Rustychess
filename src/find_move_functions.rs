use crate::board::*;
use crate::constants::*;

pub fn find_w_r_moves(board: &Board, moves: &mut [i32; 512], movesindex: &mut usize, x: usize) {
    for i in 0..7 {
        if CRM8[x + i * 8] {
            if board.board[x + 8 + i * 8] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                *movesindex += 2
            } else if board.board[x + 8 + i * 8] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                *movesindex += 2;
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
            if board.board[x + 1 + i] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 1 + i as i32;
                *movesindex += 2
            } else if board.board[x + 1 + i] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 1 + i as i32;
                *movesindex += 2;
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
            if board.board[x - 8 - i * 8] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                *movesindex += 2
            } else if board.board[x - 8 - i * 8] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                *movesindex += 2;
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
            if board.board[x - 1 - i] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 1 - i as i32;
                *movesindex += 2
            } else if board.board[x - 1 - i] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 1 - i as i32;
                *movesindex += 2;
                break;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

pub fn find_w_b_moves(board: &Board, moves: &mut [i32; 512], movesindex: &mut usize, x: usize) {
    for i in 0..7 {
        if CBM9[x + i * 9] {
            if board.board[x + 9 + i * 9] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                *movesindex += 2
            } else if board.board[x + 9 + i * 9] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                *movesindex += 2;
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
            if board.board[x + 7 + i * 7] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                *movesindex += 2
            } else if board.board[x + 7 + i * 7] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                *movesindex += 2;
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
            if board.board[x - 9 - i * 9] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                *movesindex += 2
            } else if board.board[x - 9 - i * 9] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                *movesindex += 2;
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
            if board.board[x - 7 - i * 7] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                *movesindex += 2
            } else if board.board[x - 7 - i * 7] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                *movesindex += 2;
                break;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}
pub fn find_w_kn_moves(board: &Board, moves: &mut [i32; 512], movesindex: &mut usize, x: usize) {
    if CKNM17[x] {
        if board.board[x + 17] == 0 || board.board[x + 17] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 17;
            *movesindex += 2
        }
    }
    if CKNM15[x] {
        if board.board[x + 15] == 0 || board.board[x + 15] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 15;
            *movesindex += 2
        }
    }
    if CKNM10[x] {
        if board.board[x + 10] == 0 || board.board[x + 10] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 10;
            *movesindex += 2
        }
    }
    if CKNM6[x] {
        if board.board[x + 6] == 0 || board.board[x + 6] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 6;
            *movesindex += 2
        }
    }
    if CKNMN17[x] {
        if board.board[x - 17] == 0 || board.board[x - 17] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 17;
            *movesindex += 2
        }
    }
    if CKNMN15[x] {
        if board.board[x - 15] == 0 || board.board[x - 15] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 15;
            *movesindex += 2
        }
    }
    if CKNMN10[x] {
        if board.board[x - 10] == 0 || board.board[x - 10] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 10;
            *movesindex += 2
        }
    }
    if CKNMN6[x] {
        if board.board[x - 6] == 0 || board.board[x - 6] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 6;
            *movesindex += 2
        }
    }
}

pub fn find_w_k_moves(board: &Board, moves: &mut [i32; 512], movesindex: &mut usize, x: usize) {
    if CBM9[x] && board.does_b_attack(x + 9) == false {
        if board.board[x + 9] == 0 || board.board[x + 9] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 9;
            *movesindex += 2
        }
    }
    if CBM7[x] && board.does_b_attack(x + 7) == false {
        if board.board[x + 7] == 0 || board.board[x + 7] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 7;
            *movesindex += 2
        }
    }
    if CBMN9[x] && board.does_b_attack(x - 9) == false {
        if board.board[x - 9] == 0 || board.board[x - 9] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 9;
            *movesindex += 2
        }
    }
    if CBMN7[x] && board.does_b_attack(x - 7) == false {
        if board.board[x - 7] == 0 || board.board[x - 7] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 7;
            *movesindex += 2
        }
    }
    if CRM8[x] && board.does_b_attack(x + 8) == false {
        if board.board[x + 8] == 0 || board.board[x + 8] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 8;
            *movesindex += 2
        }
    }
    if CRM1[x] && board.does_b_attack(x + 1) == false {
        if board.board[x + 1] == 0 || board.board[x + 1] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 1;
            *movesindex += 2
        }
    }
    if CRMN8[x] && board.does_b_attack(x - 8) == false {
        if board.board[x - 8] == 0 || board.board[x - 8] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 8;
            *movesindex += 2
        }
    }
    if CRMN1[x] && board.does_b_attack(x - 1) == false {
        if board.board[x - 1] == 0 || board.board[x - 1] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 1;
            *movesindex += 2
        }
    }
    if board.crights[0]
        && board.does_b_attack(61) == false
        && board.board[61] == 0
        && board.board[62] == 0
    {
        moves[*movesindex] = x as i32;
        moves[*movesindex + 1] = x as i32 + 2;
        *movesindex += 2
    }
    if board.crights[1]
        && board.does_b_attack(59) == false
        && board.board[59] == 0
        && board.board[58] == 0
        && board.board[57] == 0
    {
        moves[*movesindex] = x as i32;
        moves[*movesindex + 1] = x as i32 - 2;
        *movesindex += 2
    }
}

pub fn find_w_p_moves(board: &Board, moves: &mut [i32; 512], movesindex: &mut usize, x: usize) {
    if CPMN16[x] {
        if board.board[x - 8] == 0 && board.board[x - 16] == 0 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 16;
            *movesindex += 2
        }
    }
    if CPM16[x] == false && board.board[x - 8] == 0 {
        moves[*movesindex] = x as i32;
        moves[*movesindex + 1] = x as i32 - 8;
        *movesindex += 2
    }
    if CPM16[x] == false && CBMN9[x] {
        if board.board[x - 9] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 9;
            *movesindex += 2
        }
    }
    if CPM16[x] == false && CBMN7[x] {
        if board.board[x - 7] > OTHER_COLOR_PIECE {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 7;
            *movesindex += 2
        }
    }
    if CBMN9[x] {
        if x - 9 == board.epts {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 9;
            *movesindex += 2
        }
    }
    if CBMN7[x] {
        if x - 7 == board.epts {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 7;
            *movesindex += 2
        }
    }
    if CPM16[x] {
        if board.board[x - 8] == 0 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 8 + 400;
            *movesindex += 2
        }
        if CBMN9[x] {
            if board.board[x - 9] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 9 + 400;
                *movesindex += 2
            }
        }
        if CBMN7[x] {
            if board.board[x - 7] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 7 + 400;
                *movesindex += 2
            }
        }
        if board.board[x - 8] == 0 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 8 + 300;
            *movesindex += 2
        }
        if CBMN9[x] {
            if board.board[x - 9] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 9 + 300;
                *movesindex += 2
            }
        }
        if CBMN7[x] {
            if board.board[x - 7] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 7 + 300;
                *movesindex += 2
            }
        }
        if board.board[x - 8] == 0 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 8 + 200;
            *movesindex += 2
        }
        if CBMN9[x] {
            if board.board[x - 9] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 9 + 200;
                *movesindex += 2
            }
        }
        if CBMN7[x] {
            if board.board[x - 7] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 7 + 200;
                *movesindex += 2
            }
        }
        if board.board[x - 8] == 0 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 8 + 100;
            *movesindex += 2
        }
        if CBMN9[x] {
            if board.board[x - 9] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 9 + 100;
                *movesindex += 2
            }
        }
        if CBMN7[x] {
            if board.board[x - 7] > OTHER_COLOR_PIECE {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 7 + 100;
                *movesindex += 2
            }
        }
    }
}

pub fn find_b_r_moves(board: &Board, moves: &mut [i32; 512], movesindex: &mut usize, x: usize) {
    for i in 0..7 {
        if CRM8[x + i * 8] {
            if board.board[x + 8 + i * 8] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                *movesindex += 2
            } else if board.board[x + 8 + i * 8] < OTHER_COLOR_PIECE
                && board.board[x + 8 + i * 8] > 65
            {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 8 + 8 * i as i32;
                *movesindex += 2;
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
            if board.board[x + 1 + i] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 1 + i as i32;
                *movesindex += 2
            } else if board.board[x + 1 + i] < OTHER_COLOR_PIECE && board.board[x + 1 + i] > 65 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 1 + i as i32;
                *movesindex += 2;
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
            if board.board[x - 8 - i * 8] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                *movesindex += 2
            } else if board.board[x - 8 - i * 8] < OTHER_COLOR_PIECE
                && board.board[x - 8 - i * 8] > 65
            {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 8 - 8 * i as i32;
                *movesindex += 2;
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
            if board.board[x - 1 - i] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 1 - i as i32;
                *movesindex += 2
            } else if board.board[x - 1 - i] < OTHER_COLOR_PIECE && board.board[x - 1 - i] > 65 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 1 - i as i32;
                *movesindex += 2;
                break;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

pub fn find_b_b_moves(board: &Board, moves: &mut [i32; 512], movesindex: &mut usize, x: usize) {
    for i in 0..7 {
        if CBM9[x + i * 9] {
            if board.board[x + 9 + i * 9] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                *movesindex += 2
            } else if board.board[x + 9 + i * 9] < OTHER_COLOR_PIECE
                && board.board[x + 9 + i * 9] > 65
            {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 9 + 9 * i as i32;
                *movesindex += 2;
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
            if board.board[x + 7 + i * 7] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                *movesindex += 2
            } else if board.board[x + 7 + i * 7] < OTHER_COLOR_PIECE
                && board.board[x + 7 + i * 7] > 65
            {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 7 + 7 * i as i32;
                *movesindex += 2;
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
            if board.board[x - 9 - i * 9] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                *movesindex += 2
            } else if board.board[x - 9 - i * 9] < OTHER_COLOR_PIECE
                && board.board[x - 9 - i * 9] > 65
            {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 9 - 9 * i as i32;
                *movesindex += 2;
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
            if board.board[x - 7 - i * 7] == 0 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                *movesindex += 2
            } else if board.board[x - 7 - i * 7] < OTHER_COLOR_PIECE
                && board.board[x - 7 - i * 7] > 65
            {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 - 7 - 7 * i as i32;
                *movesindex += 2;
                break;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

pub fn find_b_kn_moves(board: &Board, moves: &mut [i32; 512], movesindex: &mut usize, x: usize) {
    if CKNM17[x] {
        if board.board[x + 17] == 0
            || board.board[x + 17] < OTHER_COLOR_PIECE && board.board[x + 17] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 17;
            *movesindex += 2
        }
    }
    if CKNM15[x] {
        if board.board[x + 15] == 0
            || board.board[x + 15] < OTHER_COLOR_PIECE && board.board[x + 15] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 15;
            *movesindex += 2
        }
    }
    if CKNM10[x] {
        if board.board[x + 10] == 0
            || board.board[x + 10] < OTHER_COLOR_PIECE && board.board[x + 10] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 10;
            *movesindex += 2
        }
    }
    if CKNM6[x] {
        if board.board[x + 6] == 0
            || board.board[x + 6] < OTHER_COLOR_PIECE && board.board[x + 6] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 6;
            *movesindex += 2
        }
    }
    if CKNMN17[x] {
        if board.board[x - 17] == 0
            || board.board[x - 17] < OTHER_COLOR_PIECE && board.board[x - 17] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 17;
            *movesindex += 2
        }
    }
    if CKNMN15[x] {
        if board.board[x - 15] == 0
            || board.board[x - 15] < OTHER_COLOR_PIECE && board.board[x - 15] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 15;
            *movesindex += 2
        }
    }
    if CKNMN10[x] {
        if board.board[x - 10] == 0
            || board.board[x - 10] < OTHER_COLOR_PIECE && board.board[x - 10] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 10;
            *movesindex += 2
        }
    }
    if CKNMN6[x] {
        if board.board[x - 6] == 0
            || board.board[x - 6] < OTHER_COLOR_PIECE && board.board[x - 6] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 6;
            *movesindex += 2
        }
    }
}

pub fn find_b_k_moves(board: &Board, moves: &mut [i32; 512], movesindex: &mut usize, x: usize) {
    if CBM9[x] && board.does_w_attack(x + 9) == false {
        if board.board[x + 9] == 0
            || board.board[x + 9] < OTHER_COLOR_PIECE && board.board[x + 9] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 9;
            *movesindex += 2
        }
    }
    if CBM7[x] && board.does_w_attack(x + 7) == false {
        if board.board[x + 7] == 0
            || board.board[x + 7] < OTHER_COLOR_PIECE && board.board[x + 7] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 7;
            *movesindex += 2
        }
    }
    if CBMN9[x] && board.does_w_attack(x - 9) == false {
        if board.board[x - 9] == 0
            || board.board[x - 9] < OTHER_COLOR_PIECE && board.board[x - 9] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 9;
            *movesindex += 2
        }
    }
    if CBMN7[x] && board.does_w_attack(x - 7) == false {
        if board.board[x - 7] == 0
            || board.board[x - 7] < OTHER_COLOR_PIECE && board.board[x - 7] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 7;
            *movesindex += 2
        }
    }
    if CRM8[x] && board.does_w_attack(x + 8) == false {
        if board.board[x + 8] == 0
            || board.board[x + 8] < OTHER_COLOR_PIECE && board.board[x + 8] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 8;
            *movesindex += 2
        }
    }
    if CRM1[x] && board.does_w_attack(x + 1) == false {
        if board.board[x + 1] == 0
            || board.board[x + 1] < OTHER_COLOR_PIECE && board.board[x + 1] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 1;
            *movesindex += 2
        }
    }
    if CRMN8[x] && board.does_w_attack(x - 8) == false {
        if board.board[x - 8] == 0
            || board.board[x - 8] < OTHER_COLOR_PIECE && board.board[x - 8] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 8;
            *movesindex += 2
        }
    }
    if CRMN1[x] && board.does_w_attack(x - 1) == false {
        if board.board[x - 1] == 0
            || board.board[x - 1] < OTHER_COLOR_PIECE && board.board[x - 1] > 65
        {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 - 1;
            *movesindex += 2
        }
    }
    if board.crights[2]
        && board.does_w_attack(5) == false
        && board.board[5] == 0
        && board.board[6] == 0
    {
        moves[*movesindex] = x as i32;
        moves[*movesindex + 1] = x as i32 + 2;
        *movesindex += 2
    }
    if board.crights[3]
        && board.does_w_attack(3) == false
        && board.board[3] == 0
        && board.board[2] == 0
        && board.board[1] == 0
    {
        moves[*movesindex] = x as i32;
        moves[*movesindex + 1] = x as i32 - 2;
        *movesindex += 2
    }
}

pub fn find_b_p_moves(board: &Board, moves: &mut [i32; 512], movesindex: &mut usize, x: usize) {
    if CPM16[x] {
        if board.board[x + 8] == 0 && board.board[x + 16] == 0 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 16;
            *movesindex += 2
        }
    }
    if CPMN16[x] == false && board.board[x + 8] == 0 {
        moves[*movesindex] = x as i32;
        moves[*movesindex + 1] = x as i32 + 8;
        *movesindex += 2
    }
    if CPMN16[x] == false && CBM9[x] {
        if board.board[x + 9] < OTHER_COLOR_PIECE && board.board[x + 9] > 65 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 9;
            *movesindex += 2
        }
    }
    if CPMN16[x] == false && CBM7[x] {
        if board.board[x + 7] < OTHER_COLOR_PIECE && board.board[x + 7] > 65 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 7;
            *movesindex += 2
        }
    }
    if CBM9[x] {
        if x + 9 == board.epts {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 9;
            *movesindex += 2
        }
    }
    if CBM7[x] {
        if x + 7 == board.epts {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 7;
            *movesindex += 2
        }
    }
    if CPMN16[x] {
        if board.board[x + 8] == 0 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 8 + 400;
            *movesindex += 2
        }
        if CBM9[x] {
            if board.board[x + 9] < OTHER_COLOR_PIECE && board.board[x + 9] > 65 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 9 + 400;
                *movesindex += 2
            }
        }
        if CBM7[x] {
            if board.board[x + 7] < OTHER_COLOR_PIECE && board.board[x + 7] > 65 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 7 + 400;
                *movesindex += 2
            }
        }
        if board.board[x + 8] == 0 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 8 + 300;
            *movesindex += 2
        }
        if CBM9[x] {
            if board.board[x + 9] < OTHER_COLOR_PIECE && board.board[x + 9] > 65 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 9 + 300;
                *movesindex += 2
            }
        }
        if CBM7[x] {
            if board.board[x + 7] < OTHER_COLOR_PIECE && board.board[x + 7] > 65 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 7 + 300;
                *movesindex += 2
            }
        }
        if board.board[x + 8] == 0 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 8 + 200;
            *movesindex += 2
        }
        if CBM9[x] {
            if board.board[x + 9] < OTHER_COLOR_PIECE && board.board[x + 9] > 65 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 9 + 200;
                *movesindex += 2
            }
        }
        if CBM7[x] {
            if board.board[x + 7] < OTHER_COLOR_PIECE && board.board[x + 7] > 65 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 7 + 200;
                *movesindex += 2
            }
        }
        if board.board[x + 8] == 0 {
            moves[*movesindex] = x as i32;
            moves[*movesindex + 1] = x as i32 + 8 + 100;
            *movesindex += 2
        }
        if CBM9[x] {
            if board.board[x + 9] < OTHER_COLOR_PIECE && board.board[x + 9] > 65 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 9 + 100;
                *movesindex += 2
            }
        }
        if CBM7[x] {
            if board.board[x + 7] < OTHER_COLOR_PIECE && board.board[x + 7] > 65 {
                moves[*movesindex] = x as i32;
                moves[*movesindex + 1] = x as i32 + 7 + 100;
                *movesindex += 2
            }
        }
    }
}
