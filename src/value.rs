const K_VALUE_MAP: [i32; 64] = [
    -3, -4, -4, -5, -5, -4, -4, -3, -3, -4, -4, -5, -5, -4, -4, -3, -3, 4, -4, -5, -5, -4, -4, -3,
    -3, -4, -4, -5, -5, -4, -4, -3, -2, -3, -3, -4, -4, -3, -3, -2, -1, -2, -2, -2, -2, -2, -2, -1,
    2, 2, 0, 0, 0, 0, 2, 2, 2, 3, 1, 0, 0, 1, 3, 2,
];

const Q_VALUE_MAP: [i32; 64] = [
    -2, -1, -1, -1, -1, -1, -1, -2, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, 1, 1, 1, 1, 0, -1, -1, 0, 1,
    1, 1, 1, 0, -1, -1, 0, 1, 1, 1, 1, 0, -1, -1, 0, 1, 1, 1, 1, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1,
    -2, -1, -1, -1, -1, -1, -1, -2,
];

const R_VALUE_MAP: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0,
    -1, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 1,
    1, 0, 0, 0,
];

const B_VALUE_MAP: [i32; 64] = [
    -2, -1, -1, -1, -1, -1, -1, -2, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, 1, 1, 1, 1, 0, -1, -1, 1, 1,
    1, 1, 1, 1, -1, -1, 0, 1, 1, 1, 1, 0, -1, -1, 1, 1, 1, 1, 1, 1, -1, -1, 1, 0, 0, 0, 0, 1, -1,
    -2, -1, -1, -1, -1, -1, -1, -2,
];

const N_VALUE_MAP: [i32; 64] = [
    -5, -4, -3, -3, -3, -3, -4, -5, -4, -2, 0, 0, 0, 0, -2, -4, -3, 0, 1, 2, 2, 1, 0, -3, -3, 1, 2,
    2, 2, 2, 1, -3, -3, 0, 2, 2, 2, 2, 0, -3, -3, 1, 1, 2, 2, 1, 1, -3, -4, -2, 0, 1, 1, 0, -2, -4,
    -5, -4, -3, -3, -3, -3, -4, -5,
];

const P_VALUE_MAP: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 2, 3, 3, 2, 1, 1, 1, 1, 1, 3, 3, 1, 1, 1,
    0, 0, 0, 2, 2, 0, 0, 0, 1, -1, -1, 0, 0, -1, -1, 1, 1, 1, 1, -2, -2, 1, 1, 1, 0, 0, 0, 0, 0, 0,
    0, 0,
];

const R_VALUE: i32 = 50;
const B_VALUE: i32 = 30;
const Q_VALUE: i32 = 90;
const K_VALUE: i32 = 1000;
const P_VALUE: i32 = 10;
const KN_VALUE: i32 = 30;

pub fn value_r_b(eval: &mut i32, position: usize) {
    *eval -= R_VALUE;
    *eval -= R_VALUE_MAP[63 - position];
}
pub fn value_kn_b(eval: &mut i32, position: usize) {
    *eval -= KN_VALUE;
    *eval -= N_VALUE_MAP[63 - position];
}
pub fn value_b_b(eval: &mut i32, position: usize) {
    *eval -= B_VALUE;
    *eval -= B_VALUE_MAP[63 - position];
}
pub fn value_q_b(eval: &mut i32, position: usize) {
    *eval -= Q_VALUE;
    *eval -= Q_VALUE_MAP[63 - position];
}
pub fn value_k_b(eval: &mut i32, position: usize) {
    *eval -= K_VALUE;
    *eval -= K_VALUE_MAP[63 - position];
}
pub fn value_p_b(eval: &mut i32, position: usize) {
    *eval -= P_VALUE;
    *eval -= P_VALUE_MAP[63 - position];
}
pub fn value_r_w(eval: &mut i32, position: usize) {
    *eval += R_VALUE;
    *eval += R_VALUE_MAP[position];
}
pub fn value_kn_w(eval: &mut i32, position: usize) {
    *eval += KN_VALUE;
    *eval += N_VALUE_MAP[position];
}
pub fn value_b_w(eval: &mut i32, position: usize) {
    *eval += B_VALUE;
    *eval += B_VALUE_MAP[position];
}
pub fn value_q_w(eval: &mut i32, position: usize) {
    *eval += Q_VALUE;
    *eval += Q_VALUE_MAP[position];
}
pub fn value_k_w(eval: &mut i32, position: usize) {
    *eval += K_VALUE;
    *eval += K_VALUE_MAP[position];
}
pub fn value_p_w(eval: &mut i32, position: usize) {
    *eval += P_VALUE;
    *eval += P_VALUE_MAP[position];
}
