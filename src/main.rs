use crate::qlearning::Piles;

// use std::time::Instant;

mod qlearning;

fn main() {
    let piles = Piles([4, 8, 0, 0]);

    if piles.xor() == 0 {
        println!("Le deuxième joueur devrait gagner.");
    } else {
        println!("Le premier joueur devrait gagner.");
    }

    let pourcent_victoire = piles.teste_fiabilité(10000, 10000, 8);
    println!("{}", pourcent_victoire);
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
