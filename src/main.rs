use crate::qlearning::piles_et_action::{Paramètres, Piles};
use num_format::{Locale, ToFormattedString};
use std::time::Instant;
pub mod qlearning;
extern crate num_cpus;

fn main() {
    let piles = Piles([2, 1, 0, 0, 0, 0, 0, 0]);
    let k = 1.0;
    let nb_coeur = num_cpus::get();
    let nb_partie = 2_000;
    let nb_modèle = 125;

    let paramètres = Paramètres {
        alpha: 0.9,
        gamma: 1.0,
        k,
        récompense: 1.0,
    };

    println!(
        "Vous avez choisi des piles de {}avec un k de {} et {} modèles.",
        piles,
        k,
        nb_modèle * nb_coeur
    );

    if piles.xor() == 0 {
        println!("Attention, le deuxième joueur devrait gagner!");
    }

    let avant = Instant::now();

    let pourcent =
        qlearning::teste_fiabilité(piles, nb_partie, nb_modèle, nb_coeur, paramètres) * 100.0;

    let chrono = avant
        .elapsed()
        .as_millis()
        .to_formatted_string(&Locale::fr_CA);

    let nb_partie = nb_partie.to_formatted_string(&Locale::fr_CA);

    println!(
        "{:.2}% avec {} parties nécessitant en {} ms",
        pourcent, nb_partie, chrono
    );
}
