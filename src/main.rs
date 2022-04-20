use crate::qlearning::piles_et_action::{Paramètres, Piles, NB_DE_PILE};
use num_format::{Locale, ToFormattedString};
use std::{io, time::Instant};
pub mod qlearning;
extern crate num_cpus;

fn main() {
    let mut piles = Piles([0, 0, 0, 0, 0, 0, 0, 0]);

    demande_piles(&mut piles);
    let nb_partie = demande_nb_partie(1_000);
    let k = demande_k(0.01);
    let nb_coeur = num_cpus::get();
    let nb_modèle = demande_nb_modèle(nb_coeur, 125);

       

    let paramètres = Paramètres {
        alpha: 0.9,
        gamma: 1.0,
        k,
        récompense: 1.0,
    };

    println!("Vous avez choisi des piles de {}avec un k de {} et {} modèles.", piles, k, nb_modèle * nb_coeur);

    if piles.xor() == 0 {
        println!("Attention, le deuxième joueur devrait gagner!");
    } 
    let avant = Instant::now();

    // let nb_coup = piles.nb_coup();
    let mut min = 100.0;
    let mut max = 0.0;

    for _ in 0..10 {
        let pourcent =
            qlearning::teste_fiabilité(piles, nb_partie, nb_modèle, nb_coeur, paramètres) * 100.0;
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

fn demande_piles(piles: &mut Piles) {
    let mut entrée = String::new();

    println!(
        "Entrez les piles que vous voulez en les séparant par un espace. (maximum {} piles)", NB_DE_PILE
    );
    io::stdin()
        .read_line(&mut entrée)
        .expect("Erreur de lecture de ligne.");

    let iter = entrée.split_whitespace().enumerate();
    for (index, pile) in iter {
        if index < 8 {
            piles[index] = pile.parse().unwrap_or(0);
        }
    }
}

fn demande_nb_partie(défaut: usize) -> usize {
    let mut entrée = String::new();
    println!("Entrez le nombre de partie. (par défaut {})", défaut);

    io::stdin()
        .read_line(&mut entrée)
        .expect("Erreur de lecture de ligne.");

    entrée.trim().parse().unwrap_or(défaut)
}

fn demande_k(défaut: f64) -> f64 {
    let mut entrée = String::new();
    println!(
        "Entrez k, vitesse à laquelle la surestimation disparait, avec un point. (par défaut {})",
        défaut
    );

    io::stdin()
        .read_line(&mut entrée)
        .expect("Erreur de lecture de ligne.");

    entrée.trim().parse().unwrap_or(défaut)
}

fn demande_nb_modèle(nb_coeur: usize, défaut: usize) -> usize {
    let mut entrée = String::new();
    println!(
        "Entrez le nombre de modèle par coeur. (vous avez {} coeur(s), par défaut {} modèles par coeur)", nb_coeur,
        défaut
    );

    io::stdin()
        .read_line(&mut entrée)
        .expect("Erreur de lecture de ligne.");

    entrée.trim().parse().unwrap_or(défaut)
}