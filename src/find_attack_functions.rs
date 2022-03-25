use crate::board::*;
use crate::constants::*;

pub fn does_w_kn_attack(board: &Board, position: usize) -> bool {
    if CKNM17[position] {
        if board.board[position + 17] == KN_W {
            return true;
        }
    }
    if CKNM15[position] {
        if board.board[position + 15] == KN_W {
            return true;
        }
    }
    if CKNM10[position] {
        if board.board[position + 10] == KN_W {
            return true;
        }
    }
    if CKNM6[position] {
        if board.board[position + 6] == KN_W {
            return true;
        }
    }
    if CKNMN17[position] {
        if board.board[position - 17] == KN_W {
            return true;
        }
    }
    if CKNMN15[position] {
        if board.board[position - 15] == KN_W {
            return true;
        }
    }
    if CKNMN10[position] {
        if board.board[position - 10] == KN_W {
            return true;
        }
    }
    if CKNMN6[position] {
        if board.board[position - 6] == KN_W {
            return true;
        }
    }
    false
}

pub fn does_w_r_attack(board: &Board, position: usize) -> bool {
    for i in 0..7 {
        if CRM8[position + i * 8] {
            if board.board[position + 8 + i * 8] == R_W || board.board[position + 8 + i * 8] == Q_W
            {
                return true;
            } else if board.board[position + 8 + i * 8] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CRM1[position + i] {
            if board.board[position + 1 + i] == R_W || board.board[position + 1 + i] == Q_W {
                return true;
            } else if board.board[position + 1 + i] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CRMN8[position - i * 8] {
            if board.board[position - 8 - i * 8] == R_W || board.board[position - 8 - i * 8] == Q_W
            {
                return true;
            } else if board.board[position - 8 - i * 8] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CRMN8[position - i] {
            if board.board[position - 1 - i] == R_W || board.board[position - 1 - i] == Q_W {
                return true;
            } else if board.board[position - 1 - i] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    false
}

pub fn does_w_b_attack(board: &Board, position: usize) -> bool {
    for i in 0..7 {
        if CBM9[position + i * 9] {
            if board.board[position + 9 + i * 9] == B_W || board.board[position + 9 + i * 9] == Q_W
            {
                return true;
            } else if board.board[position + 9 + i * 9] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CBM7[position + i * 7] {
            if board.board[position + 7 + i * 7] == B_W || board.board[position + 7 + i * 7] == Q_W
            {
                return true;
            } else if board.board[position + 7 + i * 7] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CBMN9[position - i * 9] {
            if board.board[position - 9 - i * 9] == B_W || board.board[position - 9 - i * 9] == Q_W
            {
                return true;
            } else if board.board[position - 9 - i * 9] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CBMN7[position - i * 7] {
            if board.board[position - 7 - i * 7] == B_W || board.board[position - 7 - i * 7] == Q_W
            {
                return true;
            } else if board.board[position - 7 - i * 7] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    false
}

pub fn does_w_k_attack(board: &Board, position: usize) -> bool {
    if CBM9[position] {
        if board.board[position + 9] == K_W {
            return true;
        }
    }
    if CBM7[position] {
        if board.board[position + 7] == K_W {
            return true;
        }
    }
    if CBMN9[position] {
        if board.board[position - 9] == K_W {
            return true;
        }
    }
    if CBMN7[position] {
        if board.board[position - 7] == K_W {
            return true;
        }
    }
    if CRM8[position] {
        if board.board[position + 8] == K_W {
            return true;
        }
    }
    if CRM1[position] {
        if board.board[position + 1] == K_W {
            return true;
        }
    }
    if CRMN8[position] {
        if board.board[position - 8] == K_W {
            return true;
        }
    }
    if CRMN1[position] {
        if board.board[position - 1] == K_W {
            return true;
        }
    }
    false
}

pub fn does_w_p_attack(board: &Board, position: usize) -> bool {
    if CBMN9[position] {
        if board.board[position - 9] == P_W {
            return true;
        }
    }
    if CBMN7[position] {
        if board.board[position - 7] == P_W {
            return true;
        }
    }
    false
}

pub fn does_b_kn_attack(board: &Board, position: usize) -> bool {
    if CKNM17[position] {
        if board.board[position + 17] == KN_B {
            return true;
        }
    }
    if CKNM15[position] {
        if board.board[position + 15] == KN_B {
            return true;
        }
    }
    if CKNM10[position] {
        if board.board[position + 10] == KN_B {
            return true;
        }
    }
    if CKNM6[position] {
        if board.board[position + 6] == KN_B {
            return true;
        }
    }
    if CKNMN17[position] {
        if board.board[position - 17] == KN_B {
            return true;
        }
    }
    if CKNMN15[position] {
        if board.board[position - 15] == KN_B {
            return true;
        }
    }
    if CKNMN10[position] {
        if board.board[position - 10] == KN_B {
            return true;
        }
    }
    if CKNMN6[position] {
        if board.board[position - 6] == KN_B {
            return true;
        }
    }
    false
}
pub fn does_b_r_attack(board: &Board, position: usize) -> bool {
    for i in 0..7 {
        if CRM8[position + i * 8] {
            if board.board[position + 8 + i * 8] == R_B || board.board[position + 8 + i * 8] == Q_B
            {
                return true;
            } else if board.board[position + 8 + i * 8] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CRM1[position + i] {
            if board.board[position + 1 + i] == R_B || board.board[position + 1 + i] == Q_B {
                return true;
            } else if board.board[position + 1 + i] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CRMN8[position - i * 8] {
            if board.board[position - 8 - i * 8] == R_B || board.board[position - 8 - i * 8] == Q_B
            {
                return true;
            } else if board.board[position - 8 - i * 8] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CRMN8[position - i] {
            if board.board[position - 1 - i] == R_B || board.board[position - 1 - i] == Q_B {
                return true;
            } else if board.board[position - 1 - i] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    false
}
pub fn does_b_b_attack(board: &Board, position: usize) -> bool {
    for i in 0..7 {
        if CBM9[position + i * 9] {
            if board.board[position + 9 + i * 9] == B_B || board.board[position + 9 + i * 9] == Q_B
            {
                return true;
            } else if board.board[position + 9 + i * 9] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CBM7[position + i * 7] {
            if board.board[position + 7 + i * 7] == B_B || board.board[position + 7 + i * 7] == Q_B
            {
                return true;
            } else if board.board[position + 7 + i * 7] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CBMN9[position - i * 9] {
            if board.board[position - 9 - i * 9] == B_B || board.board[position - 9 - i * 9] == Q_B
            {
                return true;
            } else if board.board[position - 9 - i * 9] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    for i in 0..7 {
        if CBMN7[position - i * 7] {
            if board.board[position - 7 - i * 7] == B_B || board.board[position - 7 - i * 7] == Q_B
            {
                return true;
            } else if board.board[position - 7 - i * 7] == 0 {
            } else {
                break;
            }
        } else {
            break;
        }
    }
    false
}
pub fn does_b_k_attack(board: &Board, position: usize) -> bool {
    if CBM9[position] {
        if board.board[position + 9] == K_B {
            return true;
        }
    }
    if CBM7[position] {
        if board.board[position + 7] == K_B {
            return true;
        }
    }
    if CBMN9[position] {
        if board.board[position - 9] == K_B {
            return true;
        }
    }
    if CBMN7[position] {
        if board.board[position - 7] == K_B {
            return true;
        }
    }
    if CRM8[position] {
        if board.board[position + 8] == K_B {
            return true;
        }
    }
    if CRM1[position] {
        if board.board[position + 1] == K_B {
            return true;
        }
    }
    if CRMN8[position] {
        if board.board[position - 8] == K_B {
            return true;
        }
    }
    if CRMN1[position] {
        if board.board[position - 1] == K_B {
            return true;
        }
    }
    false
}
pub fn does_b_p_attack(board: &Board, position: usize) -> bool {
    if CBM9[position] {
        if board.board[position + 9] == P_B {
            return true;
        }
    }
    if CBM7[position] {
        if board.board[position + 7] == P_B {
            return true;
        }
    }
    false
}
