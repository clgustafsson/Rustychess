pub mod bc {
    pub const DISTANCE_TO_EDGE: [u8; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 2, 2, 2, 2, 1, 0, 0, 1, 2, 3, 3, 2,
        1, 0, 0, 1, 2, 3, 3, 2, 1, 0, 0, 1, 2, 2, 2, 2, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    //format for consts below
    // Can<Piece>Move<Distance>   CKNMN17 -> C KN M N17 -> Can Knight Move - 17

    pub const CKNM17: [bool; 64] = [
        true, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        false, true, true, true, true, true, true, true, false, true, true, true, true, true, true,
        true, false, true, true, true, true, true, true, true, false, true, true, true, true, true,
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false,
    ];

    pub const CKNM15: [bool; 64] = [
        false, true, true, true, true, true, true, true, false, true, true, true, true, true, true,
        true, false, true, true, true, true, true, true, true, false, true, true, true, true, true,
        true, true, false, true, true, true, true, true, true, true, false, true, true, true, true,
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false,
    ];

    pub const CKNM10: [bool; 64] = [
        true, true, true, true, true, true, false, false, true, true, true, true, true, true,
        false, false, true, true, true, true, true, true, false, false, true, true, true, true,
        true, true, false, false, true, true, true, true, true, true, false, false, true, true,
        true, true, true, true, false, false, true, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ];

    pub const CKNM6: [bool; 64] = [
        false, false, true, true, true, true, true, true, false, false, true, true, true, true,
        true, true, false, false, true, true, true, true, true, true, false, false, true, true,
        true, true, true, true, false, false, true, true, true, true, true, true, false, false,
        true, true, true, true, true, true, false, false, true, true, true, true, true, true,
        false, false, false, false, false, false, false, false,
    ];

    pub const CKNMN17: [bool; 64] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true, true, true, true, true, true, true, false, true, true,
        true, true, true, true, true, false, true, true, true, true, true, true, true, false, true,
        true, true, true, true, true, true, false, true, true, true, true, true, true, true, false,
        true, true, true, true, true, true, true,
    ];

    pub const CKNMN15: [bool; 64] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, true, true, true, true, true, true, true, false, true, true, true,
        true, true, true, true, false, true, true, true, true, true, true, true, false, true, true,
        true, true, true, true, true, false, true, true, true, true, true, true, true, false, true,
        true, true, true, true, true, true, false,
    ];

    pub const CKNMN10: [bool; 64] = [
        false, false, false, false, false, false, false, false, false, false, true, true, true,
        true, true, true, false, false, true, true, true, true, true, true, false, false, true,
        true, true, true, true, true, false, false, true, true, true, true, true, true, false,
        false, true, true, true, true, true, true, false, false, true, true, true, true, true,
        true, false, false, true, true, true, true, true, true,
    ];

    pub const CKNMN6: [bool; 64] = [
        false, false, false, false, false, false, false, false, true, true, true, true, true, true,
        false, false, true, true, true, true, true, true, false, false, true, true, true, true,
        true, true, false, false, true, true, true, true, true, true, false, false, true, true,
        true, true, true, true, false, false, true, true, true, true, true, true, false, false,
        true, true, true, true, true, true, false, false,
    ];

    pub const CRM8: [bool; 64] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, false, false, false,
        false, false, false, false, false,
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
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true,
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
        true, true, false, true, true, true, true, true, true, true, false, false, false, false,
        false, false, false, false, false,
    ];

    pub const CBM7: [bool; 64] = [
        false, true, true, true, true, true, true, true, false, true, true, true, true, true, true,
        true, false, true, true, true, true, true, true, true, false, true, true, true, true, true,
        true, true, false, true, true, true, true, true, true, true, false, true, true, true, true,
        true, true, true, false, true, true, true, true, true, true, true, false, false, false,
        false, false, false, false, false,
    ];

    pub const CBMN9: [bool; 64] = [
        false, false, false, false, false, false, false, false, false, true, true, true, true,
        true, true, true, false, true, true, true, true, true, true, true, false, true, true, true,
        true, true, true, true, false, true, true, true, true, true, true, true, false, true, true,
        true, true, true, true, true, false, true, true, true, true, true, true, true, false, true,
        true, true, true, true, true, true,
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
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
    ];

    pub const CPMN16: [bool; 64] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, true, true, true, true,
        true, true, true, true, false, false, false, false, false, false, false, false,
    ];

    pub const K_VALUE_MAP: [i32; 64] = [
        -3, -4, -4, -5, -5, -4, -4, -3, -3, -4, -4, -5, -5, -4, -4, -3, -3, 4, -4, -5, -5, -4, -4,
        -3, -3, -4, -4, -5, -5, -4, -4, -3, -2, -3, -3, -4, -4, -3, -3, -2, -1, -2, -2, -2, -2, -2,
        -2, -1, 2, 2, 0, 0, 0, 0, 2, 2, 2, 3, 1, 0, 0, 1, 3, 2,
    ];

    pub const Q_VALUE_MAP: [i32; 64] = [
        -2, -1, -1, -1, -1, -1, -1, -2, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, 1, 1, 1, 1, 0, -1, -1, 0,
        1, 1, 1, 1, 0, -1, -1, 0, 1, 1, 1, 1, 0, -1, -1, 0, 1, 1, 1, 1, 0, -1, -1, 0, 0, 0, 0, 0,
        0, -1, -2, -1, -1, -1, -1, -1, -1, -2,
    ];

    pub const R_VALUE_MAP: [i32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0,
        0, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1, 0,
        0, 0, 1, 1, 0, 0, 0,
    ];

    pub const B_VALUE_MAP: [i32; 64] = [
        -2, -1, -1, -1, -1, -1, -1, -2, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, 1, 1, 1, 1, 0, -1, -1, 1,
        1, 1, 1, 1, 1, -1, -1, 0, 1, 1, 1, 1, 0, -1, -1, 1, 1, 1, 1, 1, 1, -1, -1, 1, 0, 0, 0, 0,
        1, -1, -2, -1, -1, -1, -1, -1, -1, -2,
    ];

    pub const N_VALUE_MAP: [i32; 64] = [
        -5, -4, -3, -3, -3, -3, -4, -5, -4, -2, 0, 0, 0, 0, -2, -4, -3, 0, 1, 2, 2, 1, 0, -3, -3,
        1, 2, 2, 2, 2, 1, -3, -3, 0, 2, 2, 2, 2, 0, -3, -3, 1, 1, 2, 2, 1, 1, -3, -4, -2, 0, 1, 1,
        0, -2, -4, -5, -4, -3, -3, -3, -3, -4, -5,
    ];

    pub const P_VALUE_MAP: [i32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 2, 3, 3, 2, 1, 1, 1, 1, 1, 3, 3, 1,
        1, 1, 0, 0, 0, 2, 2, 0, 0, 0, 1, -1, -1, 0, 0, -1, -1, 1, 1, 1, 1, -2, -2, 1, 1, 1, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];
}
