use crate::qlearning::Piles;
// use std::time::Instant;s

mod qlearning;

// const ALPHA: f32 = 0.9;
const gamma: f32 = 1.01;
const RÉCOMPENSE: f32 = 2.0;

fn main() {
    let piles = Piles([3, 5, 0, 0]);

    if piles.xor() == 0 {
        println!("Le deuxième joueur devrait gagner.");
    } else {
        println!("Le premier joueur devrait gagner.");
    }
    let x = 100;
    // for _repeat in 1..11{
    for ALPHA in 85..=100 {
    //  for _repeat in 1..11{
        let pourcentage_victoire =
            piles.teste_fiabilité(x, 10000000, ALPHA  as f32 / 100.0, gamma, RÉCOMPENSE);
        println!("{}, {}", ALPHA as f32 / 100.0, pourcentage_victoire);
    //  }
    }   
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
