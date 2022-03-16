use crate::qlearning::{Paramètres, Piles};

// use std::time::Instant;

mod qlearning;

fn main() {
    let piles = Piles([2, 19, 0, 0, 0, 0, 0, 0]);

    if piles.xor() == 0 {
        println!("Le deuxième joueur devrait gagner.");
    } else {
        println!("Le premier joueur devrait gagner.");
    }

    let paramètres = Paramètres {
        alpha: 0.9,
        gamma: 1.0,
        beta: 17.0,
        récompense: 1.0,
        punition: -1.0,
    };

    let difficulté = piles.nb_coup();
    let pourcent_victoire = piles.teste_fiabilité(100_000, 25, 8, paramètres);
    println!("{:.2}%, {}", pourcent_victoire * 100.0, difficulté);
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
