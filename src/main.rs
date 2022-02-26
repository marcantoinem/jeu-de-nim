use crate::qlearning::Piles;

mod qlearning;
fn main() {
    let piles = Piles([3, 5]);
    let _hashmap = qlearning::train(&piles, 10000);
    println!("Le jeu de Nim.");
}

//  001 = 1 = 2⁰
//  010 = 2 = 2¹
//  100 = 4 = 2²
// 1000 = 8 = 2³
// 2⁰ + 2¹ = 3
// 1111
// 2³*1 2²*1 2¹*1 2⁰*1
// 2⁴-1
// u8 = 8 emplacements
