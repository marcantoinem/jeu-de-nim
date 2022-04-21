use crate::app::qlearning::piles_et_action::{Action, Paramètres, Piles};
use fxhash::FxHashMap;
use std::thread;

pub mod piles_et_action;

const MINIMUM: f64 = 0.001;
const MAXIMUM: f64 = 40.0;
const BETA_MAX: f64 = 16.0;

pub fn entraine(piles: &Piles, nb_partie: usize, p: Paramètres) -> FxHashMap<Piles, Action> {
    let mut hashmap = piles.genere_hashmap();
    for nb in 0..nb_partie {
        let beta = p.k * (nb * nb) as f64;

        let mut piles = *piles;
        let mut partie = vec![];

        let win = loop {
            if piles.zero_partout() {
                // Victoire deuxième joueur
                break false;
            }

            let action_prise = piles.cherche_action(&hashmap);
            partie.push((piles, *action_prise));
            piles = action_prise.future_piles(piles);

            if piles.zero_partout() {
                // Victoire premier joueur
                break true;
            }

            let action_prise = piles.cherche_action(&hashmap);
            piles = action_prise.future_piles(piles);
        };

        partie.reverse();

        let mut qualité_max = if win { 1.0 } else { -1.0 };

        for (piles, action_prise) in partie {
            let mut piles = piles;
            piles.trie_croissant();
            let entrée = hashmap.entry(piles).or_default();
            let qualité = entrée.entry(action_prise).or_default();

            if win {
                *qualité =
                    (1.0 - p.alpha) * *qualité + p.alpha * (p.récompense + p.gamma * qualité_max);
            } else {
                *qualité =
                    (1.0 - p.alpha) * *qualité + p.alpha * (-p.récompense + p.gamma * qualité_max);
            }

            if *qualité < MINIMUM {
                *qualité = MINIMUM
            } else if *qualité > MAXIMUM {
                *qualité = MAXIMUM;
            }

            qualité_max = qualité_maximale_régularisée(entrée, beta);
            // qualité_max = qualité_maximale(entrée);
        }
    }
    nettoyer_hashmap(hashmap)
}

pub fn _entraine_affiche(piles: &Piles, nb_partie: usize, p: Paramètres) {
    let mut hashmap = piles.genere_hashmap();

    for nb in 0..nb_partie {
        println!("Partie {}:", nb + 1);
        let mut piles = *piles;
        let mut partie = vec![];
        let win = loop {
            if piles.zero_partout() {
                // Victoire deuxième joueur
                break false;
            }

            let action_prise = piles.cherche_action(&hashmap);
            partie.push((piles, *action_prise));
            piles = action_prise.future_piles(piles);

            if piles.zero_partout() {
                // Victoire premier joueur
                break true;
            }

            let action_prise = piles.cherche_action(&hashmap);
            piles = action_prise.future_piles(piles);
        };

        partie.reverse();

        let mut qualité_max = 1.0;

        for (piles, action_prise) in partie {
            let mut piles = piles;

            piles.trie_croissant();

            let entrée = hashmap.entry(piles).or_default();

            let qualité = entrée.entry(action_prise).or_default();

            if win {
                *qualité =
                    (1.0 - p.alpha) * *qualité + p.alpha * (p.récompense + p.gamma * qualité_max);
            } else {
                *qualité =
                    (1.0 - p.alpha) * *qualité + p.alpha * (-p.récompense + p.gamma * qualité_max);
            }

            if *qualité < MINIMUM {
                *qualité = MINIMUM
            } else if *qualité > MAXIMUM {
                *qualité = MAXIMUM;
            }

            println!(
                "Piles: {}, Action: {}, Qualité: {:.3}",
                piles,
                action_prise.future_piles(piles),
                *qualité
            );

            let entrée = hashmap.get(&piles).unwrap();
            qualité_max = qualité_maximale(entrée);
        }
    }
}

fn qualité_maximale_régularisée(liste_action: &FxHashMap<Action, f64>, beta: f64) -> f64 {
    if liste_action.is_empty() {
        return 1.0;
    }
    if beta > BETA_MAX {
        return qualité_maximale(liste_action);
    }
    // Source : https://www.ijcai.org/proceedings/2020/0276.pdf
    let mut dbs: f64 = 0.0;
    let mut somme: f64 = 0.0;

    for action_qualité in liste_action {
        somme += (beta * action_qualité.1).exp();
    }

    for action_qualité in liste_action {
        dbs += (beta * action_qualité.1).exp() * action_qualité.1 / somme;
    }

    dbs as f64
}

fn qualité_maximale(liste_action: &FxHashMap<Action, f64>) -> f64 {
    if liste_action.is_empty() {
        return 1.0;
    }
    let mut iterator = liste_action.into_iter();
    let mut meilleure_action = iterator.next().unwrap();
    for action_qualité in iterator {
        if action_qualité.1 > meilleure_action.1 {
            meilleure_action = action_qualité;
        }
    }
    *meilleure_action.1
}

fn action_qualité_maximale(liste_action: FxHashMap<Action, f64>) -> Action {
    if liste_action.is_empty() {
        return Action {
            pile: 0,
            nb_enleve: 0,
        };
    }
    let mut iterator = liste_action.into_iter();
    let mut meilleure_action = iterator.next().unwrap();
    for action_qualité in iterator {
        if action_qualité.1 > meilleure_action.1 {
            meilleure_action = action_qualité;
        }
    }
    meilleure_action.0
}

fn nettoyer_hashmap(hashmap: FxHashMap<Piles, FxHashMap<Action, f64>>) -> FxHashMap<Piles, Action> {
    let mut hashmap_nettoyé = FxHashMap::default();
    for (pile, liste_action) in hashmap {
        hashmap_nettoyé.insert(pile, action_qualité_maximale(liste_action));
    }
    hashmap_nettoyé
}

pub fn victoire_parfaite(piles_originales: Piles, hashmap: FxHashMap<Piles, Action>) -> bool {
    let mut piles = piles_originales;
    loop {
        if piles.zero_partout() {
            return false;
        }

        let mut piles_triées = piles.ajout_index();
        piles_triées.trie_croissant();

        let action_prise = hashmap
            .get(&piles_triées.enleve_index())
            .expect("Pile et action inaccessibles dans le hashmap.");

        piles = action_prise.future_piles(piles);

        if piles.zero_partout() {
            return true;
        }

        piles = piles.trouver_xor_zero();
    }
}

pub fn teste_victoire(piles: &Piles, nb_partie: usize, nb_modèle: usize, p: Paramètres) -> u32 {
    let mut nb_victoire = 0;
    for _ in 0..nb_modèle {
        let hashmap = entraine(piles, nb_partie, p);
        nb_victoire += victoire_parfaite(*piles, hashmap) as u32;
    }
    nb_victoire
}

pub fn teste_fiabilité(
    piles: Piles,
    nb_partie: usize,
    nb_modèle: usize,
    nb_coeur: usize,
    p: Paramètres,
) -> f64 {
    let mut travailleurs = Vec::new();

    for _ in 0..nb_coeur {
        let travailleur = thread::spawn(move || {
            return teste_victoire(&piles, nb_partie, nb_modèle, p);
        });

        // À noter qu'ici on pousse des travailleurs #Totalement pas un goulag
        travailleurs.push(travailleur);
    }

    let mut nb_victoire = 0;
    for travailleur in travailleurs {
        let resultat: u32 = travailleur.join().unwrap();
        nb_victoire += resultat;
    }

    nb_victoire as f64 / (nb_modèle * nb_coeur) as f64
}
