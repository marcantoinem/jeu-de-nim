use fxhash::FxHashMap;
use crate::qlearning::piles_et_action::{Paramètres, Piles, Action};
use std::thread;

pub mod piles_et_action;

const MINIMUM: f64 = 0.001;
const MAXIMUM: f64 = 40.0;

pub fn teste_fiabilité(
    piles: Piles,
    nb_partie: u64,
    nb_modèle: u32,
    nb_travailleur: u32,
    p: Paramètres,
) -> f64 {
    let mut travailleurs = Vec::new();

    for _ in 0..nb_travailleur {
        let travailleur = thread::spawn(move || {
            return teste_victoire(&piles, nb_partie, nb_modèle, p);
        });
        travailleurs.push(travailleur);
    }

    let mut nb_victoire = 0;
    for travailleur in travailleurs {
        let resultat: u32 = travailleur.join().unwrap();
        nb_victoire += resultat;
    }

    nb_victoire as f64 / (nb_modèle * nb_travailleur) as f64
}

pub fn teste_victoire(piles: &Piles, nb_partie: u64, nb_modèle: u32, p: Paramètres) -> u32 {
    let mut nb_victoire = 0;
    for _ in 0..nb_modèle {
        let hashmap = entraine(piles, nb_partie, p);
        nb_victoire += victoire_parfaite(*piles, hashmap) as u32;
    }
    nb_victoire
}

// Algorithme Epsilon-Greedy
// fn choisis_action(hashmap: &FxHashMap<Action, f64> , epsilon: f64) -> &Action {
//     let vecteur = Vec::from_iter(hashmap.iter());
//     let mut rng = rand::thread_rng();
//     let valeur_aléatoire: f64 = rng.gen();

//     if valeur_aléatoire < epsilon {
//         let index = rng.gen_range(0..vecteur.len());
//         return vecteur[index].0;
//     } else {
//         return &vecteur_max(&vecteur);
//     }
// }

// fn vecteur_max<'a>(liste_action: &Vec<(&'a Action, &f64)>) -> &'a Action {
//     if liste_action.is_empty() {
//         return &Action {
//             pile: 0,
//             nb_enleve: 0,
//         };
//     }
//     let mut iterator = liste_action.into_iter();
//     let mut meilleure_action = iterator.next().unwrap();
//     for action_qualité in iterator {
//         if action_qualité.1 > meilleure_action.1 {
//             meilleure_action = action_qualité;
//         }
//     }
//     &meilleure_action.0
// }

pub fn entraine(piles: &Piles, nb_partie: u64, p: Paramètres) -> FxHashMap<Piles, Action> {
    let mut hashmap = piles.genere_hashmap();
    let mut _beta = 0.0;   

    for _nb in 0..nb_partie {
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

        let mut qualité_dbs = 1.0;

        for (piles, action_prise) in partie {
            let mut piles = piles;
            piles.trie_croissant();
            let entrée = hashmap.entry(piles).or_default();
            let qualité = entrée.entry(action_prise).or_default();

            if win {
                *qualité =
                    (1.0 - p.alpha) * *qualité + p.alpha * (p.récompense + p.gamma * qualité_dbs);
            } else {
                *qualité =
                    (1.0 - p.alpha) * *qualité + p.alpha * (p.punition + p.gamma * qualité_dbs);
            }

            if *qualité < MINIMUM {
                *qualité = MINIMUM
            } else if *qualité > MAXIMUM {
                *qualité = MAXIMUM;
            }

            let entrée = hashmap.get(&piles).unwrap();
            qualité_dbs = _qualité_maximale(entrée);
        }
        // beta = p.beta - p.beta * (-50.0 / nb_partie as f64 * (_nb as f64)).exp();
        _beta = p.beta * _nb as f64 / nb_partie as f64;
        // _beta = p.beta * (_nb * _nb) as f64 / (nb_partie * nb_partie) as f64;
    }
    nettoyer_hashmap(hashmap)
}

pub fn entraine_affiche(piles: &Piles, nb_partie: u64, p: Paramètres) -> FxHashMap<Piles, Action> {
    let mut hashmap = piles.genere_hashmap();
    let mut beta = 0.0;   

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

        let mut qualité_dbs = 1.0;

        for (piles, action_prise) in partie {
            let mut piles = piles;
            piles.trie_croissant();
            let entrée = hashmap.entry(piles).or_default();
            let qualité = entrée.entry(action_prise).or_default();

            if win {
                *qualité =
                    (1.0 - p.alpha) * *qualité + p.alpha * (p.récompense + p.gamma * qualité_dbs);
            } else {
                *qualité =
                    (1.0 - p.alpha) * *qualité + p.alpha * (p.punition + p.gamma * qualité_dbs);
            }

            if *qualité < MINIMUM {
                *qualité = MINIMUM
            } else if *qualité > MAXIMUM {
                *qualité = MAXIMUM;
            }

            println!("Piles: {}, Action: {}, Qualité: {:.3}", piles, action_prise.future_piles(piles), *qualité);

            let entrée = hashmap.get(&piles).unwrap();
            qualité_dbs = _qualité_maximale_dbs(entrée, beta);
        }
        beta = p.beta * (nb * nb) as f64 / (nb_partie * nb_partie) as f64;
    }
    nettoyer_hashmap(hashmap)
}

fn _qualité_maximale_dbs(liste_action: &FxHashMap<Action, f64>, beta: f64) -> f64 {
    if liste_action.is_empty() {
        return 1.0;
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

fn _qualité_maximale(liste_action: &FxHashMap<Action, f64>) -> f64 {
    if liste_action.is_empty() {
        return 1.0
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

