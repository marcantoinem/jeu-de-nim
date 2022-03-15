use crate::qlearning::{Paramètres, Piles};

// use std::time::Instant;

mod qlearning;

fn main() {
    let piles = Piles([5, 4, 3, 0]);

    if piles.xor() == 0 {
        println!("Le deuxième joueur devrait gagner.");
    } else {
        println!("Le premier joueur devrait gagner.");
    }

    let mut paramètres = Paramètres {
        alpha: 0.85,
        gamma: 0.94,
        beta: 12.5,
        récompense: 1.0,
        punition: -1.0,
    };

    for alpha in 6..=10 {
        for gamma in 6..=10 {
            for beta in 10..=14 {
                paramètres.alpha = alpha as f64 / 10.0;
                paramètres.gamma = gamma as f64 / 10.0;
                paramètres.beta = beta as f64 ;
                let difficulté = piles.difficulté();
                let pourcent_victoire = piles.teste_fiabilité(50_000, 1000, 4, paramètres);
                println!(
                    "{:.2}%, {}, {}, {}, {}",
                    pourcent_victoire * 100.0,
                    difficulté,
                    paramètres.alpha,
                    paramètres.gamma,
                    paramètres.beta
                );
            }
        }
    }

    let difficulté = piles.difficulté();
    let pourcent_victoire = piles.teste_fiabilité(500_000, 125, 8, paramètres);
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
