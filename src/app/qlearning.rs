use crate::app::qlearning::piles_et_action::{Action, Paramètres, Piles};
use fxhash::FxHashMap;
use std::thread;

pub mod piles_et_action;

// Limite minimale et maximale de qualité d'une action
const MINIMUM: f64 = 0.001;
const MAXIMUM: f64 = 30.0;
const BETA_MAX: f64 = 16.0;

// Cette fonction entraine un modèle de Q-learning avec un certain nombre de parties et retourne le modèle.
pub fn entraine(piles: &Piles, nb_partie: usize, p: Paramètres) -> FxHashMap<Piles, Action> {
    let mut hashmap = piles.genere_hashmap();

    for nb in 0..nb_partie {
        let mut piles = *piles;
        let mut partie = vec![];

        // Cette loop représente une partie
        let win = loop {
            if piles.zero_partout() {
                // Le deuxième joueur à gagner, ce qui termine la partie
                break false;
            }

            let action_prise = piles.cherche_action(&hashmap);
            partie.push((piles, *action_prise));
            piles = action_prise.future_piles(piles);

            if piles.zero_partout() {
                // Le premier joueur à gagner, ce qui termine la partie
                break true;
            }

            let action_prise = piles.cherche_action(&hashmap);
            piles = action_prise.future_piles(piles);
        };

        // On part de la fin afin de pouvoir calculer le maximum des piles futures avant d'appliquer la formule du Q-learning.
        partie.reverse();
        // Valeur du maximum pour la dernière valeur.
        let mut qualité_max = if win { 1.0 } else { -1.0 };

        // Niveau de surestimation
        let beta = p.k * (nb * nb) as f64;

        for (piles, action_prise) in partie {
            let mut piles = piles;
            piles.trie_croissant();
            let entrée = hashmap.entry(piles).or_default();
            let qualité = entrée.entry(action_prise).or_default();

            // Calcul de la formule du Q-learning ici
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
        }
    }
    nettoyer_hashmap(hashmap)
}

fn qualité_maximale_régularisée(liste_action: &FxHashMap<Action, f64>, beta: f64) -> f64 {
    if liste_action.is_empty() {
        panic!("Des listes actions vides ont été envoyées.");
    };

    // Approxime le maximum régularisé par un maximum si le bêta est trop grand.
    if beta > BETA_MAX {
        return qualité_maximale(liste_action);
    }

    let mut maximum_régularisé: f64 = 0.0;
    let mut somme: f64 = 0.0;

    // Calcule le dénominateur (la somme en-dessous)
    for action_qualité in liste_action {
        somme += (beta * action_qualité.1).exp();
    }

    // Calcule le numérateur
    for action_qualité in liste_action {
        maximum_régularisé += (beta * action_qualité.1).exp() * action_qualité.1 / somme;
    }

    // Retourne le maximum régularisé
    maximum_régularisé as f64
}

// Cette fonction calcule le maximum d'une liste d'action avec leur qualité
fn qualité_maximale(liste_action: &FxHashMap<Action, f64>) -> f64 {
    let mut iterator = liste_action.into_iter();
    // Le maximum est la première action
    let mut meilleure_action = iterator.next().unwrap();
    // Si une qualité est plus grande que le maximum, remplace le maximum
    for action_qualité in iterator {
        if action_qualité.1 > meilleure_action.1 {
            meilleure_action = action_qualité;
        }
    }
    // Retourne le maximum
    *meilleure_action.1
}

// Cette fonction sélectionne l'action avec la plus grande qualité
fn action_qualité_maximale(liste_action: FxHashMap<Action, f64>) -> Action {
    if liste_action.is_empty() {
        return Action{
            pile: 0,
            nb_enlevé: 0,
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

// Cette fonction transforme une hashmap avec des actions et des qualités pour chaques piles et en ressort une 
// hashmap qui associe chacune des piles avec la meilleure action
fn nettoyer_hashmap(hashmap: FxHashMap<Piles, FxHashMap<Action, f64>>) -> FxHashMap<Piles, Action> {
    let mut hashmap_nettoyé = FxHashMap::default();
    for (pile, liste_action) in hashmap {
        hashmap_nettoyé.insert(pile, action_qualité_maximale(liste_action));
    }
    hashmap_nettoyé
}

// Cette fonction teste un modèle de Q-learning contre l'algorithme xor-zéro (voir dans qlearning/piles_et_actions.rs)
// et retourne un 1 pour une victoire ou un 0 pour une défaite.
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

pub fn teste_victoire(piles: &Piles, nb_partie: usize, nb_modèle: usize, p: Paramètres) -> usize {
    let mut nb_victoire = 0;
    for _ in 0..nb_modèle {
        let hashmap = entraine(piles, nb_partie, p);
        nb_victoire += victoire_parfaite(*piles, hashmap) as usize;
    }
    nb_victoire
}

// Cette fonction utilise teste_victoire() sur plusieurs coeurs du processeurs et retourne le ratio de victoire/nb_total.
pub fn teste_fiabilité(
    piles: Piles,
    nb_partie: usize,
    nb_modèle: usize,
    nb_coeur: usize,
    p: Paramètres,
) -> Vec<usize> {
    let mut travailleurs = Vec::new();

    for _ in 0..nb_coeur {
        let travailleur = thread::spawn(move || {
            return teste_victoire(&piles, nb_partie, nb_modèle, p);
        });

        // À noter qu'ici on pousse (ajoute des travailleurs à un vecteur) des travailleurs #Totalement pas un goulag
        travailleurs.push(travailleur);
    }

    let mut liste_statistique = Vec::new();
    for travailleur in travailleurs {
        let nb_victoire: usize = travailleur.join().unwrap();
        liste_statistique.push(nb_victoire);
    }

    liste_statistique
}
