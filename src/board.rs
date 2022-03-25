
#[derive(Copy, Clone)]
pub struct Board {
    pub board: [u8; 64],
    pub turn: bool,
    pub crights: [bool; 4],
    pub epts: usize,
    pub pin: [bool; 64],
    pub halfmove: u8,
    pub fullmove: u8,
    pub attackwhite: [u8; 64],
    pub attackblack: [u8; 64],
    pub checkmate: bool,
}
impl Default for Board {
    fn default() -> Board {
        Board {
            board: [0; 64],
            turn: true,
            crights: [false; 4],
            epts: 100,
            pin: [false; 64],
            halfmove: 0,
            fullmove: 1,
            attackwhite: [0; 64],
            attackblack: [0; 64],
            checkmate: false,
        }
    }
}
