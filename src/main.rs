use crate::qlearning::{Paramètres, Piles};
use num_format::{Locale, ToFormattedString};
use std::time::Instant;
pub mod qlearning;

fn main() {
    let piles = Piles([8, 7, 6, 5, 0, 0, 0, 0]);

    if piles.xor() == 0 {
        println!("Le deuxième joueur devrait gagner.");
    } else {
        println!("Le premier joueur devrait gagner.");
    }

    let nb_partie = 500_000;
    let nb_modèle = 10;
    let nb_travailleur = 8;
    let paramètres = Paramètres {
        alpha: 0.9,
        gamma: 1.0,
        beta: 17.0,
        récompense: 1.0,
        punition: -1.0,
    };

    let avant = Instant::now();
    let pourcent = piles.teste_fiabilité(nb_partie, nb_modèle, nb_travailleur, paramètres);
    let chrono = avant.elapsed().as_millis().to_formatted_string(&Locale::fr_CA);

    let modèles = (nb_modèle * nb_travailleur).to_formatted_string(&Locale::fr_CA);
    let nb_partie = nb_partie.to_formatted_string(&Locale::fr_CA);

    let coup = piles.nb_coup();
    println!(
        "{:.2}% sur {} modèles entrainés avec {} partie(s), {} coups nécessaires, {} ms",
        pourcent * 100.0,
        modèles,
        nb_partie,
        coup,
        chrono
    );
}
