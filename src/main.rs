use crate::qlearning::{Paramètres, Piles};
use std::time::Instant;

pub mod qlearning;

fn main() {
    let piles = Piles([5, 6, 0, 0, 0, 0, 0, 0]);

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
    let nb_modele_par_travailleur = 125;
    let nb_travailleur = 8;

    let avant = Instant::now();
    let pourcent_victoire = piles.teste_fiabilité(
        10_000,
        nb_modele_par_travailleur,
        nb_travailleur,
        paramètres,
    );
    let chrono = avant.elapsed().as_millis();
    let difficulté = piles.nb_coup();
    println!(
        "{:.2}% sur {} modèles entrainés, {} coups nécessaires, {} ms",
        pourcent_victoire * 100.0,
        nb_modele_par_travailleur * nb_travailleur,
        difficulté,
        chrono
    );
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
