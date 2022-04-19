use crate::qlearning::piles_et_action::{Paramètres, Piles};
use num_format::{Locale, ToFormattedString};
use std::time::Instant;
pub mod qlearning;

fn main() {
    let piles = Piles([1, 4, 5, 7, 0, 0, 0, 0]);

    if piles.xor() == 0 {
        println!("Attention, le deuxième joueur devrait gagner.");
    }

    let nb_modèle = 25;
    let nb_travailleur = 8;

    let paramètres = Paramètres {
        alpha: 0.9,
        gamma: 1.0,
        k: 0.001,
        récompense: 1.0,
        punition: -1.0,
    };

    let avant = Instant::now();

    // let nb_coup = piles.nb_coup();
    let nb_partie = 20_000;
    let mut min = 100.0;
    let mut max = 0.0;

    for _ in 0..10 {
        let pourcent =
            qlearning::teste_fiabilité(piles, nb_partie, nb_modèle, nb_travailleur, paramètres)
                * 100.0;
        if pourcent > max {
            max = pourcent;
        } else if pourcent < min {
            min = pourcent;
        }
    }

    let chrono = avant
        .elapsed()
        .as_millis()
        .to_formatted_string(&Locale::fr_CA);

    let milieu = (max + min) / 2.0;
    let incertitude = (max - min) / 2.0;
    let nb_partie = nb_partie.to_formatted_string(&Locale::fr_CA);

    println!(
        "{:.2}±{:.2}% avec {} parties en {} ms",
        milieu, incertitude, nb_partie, chrono
    );
}

// 29 nb_coups nécessaires pour Piles([8, 7, 6, 5, 4, 3, 2, 1]) à 2 000 000 parties
// 60% pour modèle DBS-Qlearning t
// 55% pour modèle DBS-Qlearning t²
// 30% pour modèle Qlearning classique
