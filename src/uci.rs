use crate::board::*;
use std::collections::HashMap;

pub fn uci_id() {
    let name = "Rustychess";
    let author = "Linus Gustafsson";
    println!("id name {}\nid author {}", name, author);
    log::info!("out --> id name {}\nid author {}", name, author);
}
pub fn initialize_uci() {
    println!("uciok");
    log::info!("out --> uciok");
}
pub fn initialize_engine() {
    println!("readyok");
    log::info!("out --> readyok");
}
pub fn uci_new_game() {}

pub fn uci_make_moves(
    my_board: &mut Board,
    text: &str,
    standard_to_rustychess: &HashMap<&str, usize>,
) {
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
