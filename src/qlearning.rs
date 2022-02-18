use rand::distributions::WeightedIndex;
use std::collections::HashMap;

// Récompense
// +1 pour une victoire sur toutes les actions
// -1 pour une défaite sur les actions

#[derive(Copy, Clone)]
pub struct Action {
    pile: u8,
    nombre_enleve: u8,
}

#[derive(Copy, Clone)]
struct ActionAvecPoids {
    action: Action,
    poids: u32,
}

fn zero_partout(piles: &Vec<u8>) -> bool {
    for pile in piles {
        if pile != &0 {
            return false;
        }
    }
    true
}

fn actions_possibles(piles: Vec<u8>, poids: u32) -> Vec<ActionAvecPoids> {
    let mut actions = vec![];
    let mut pile_index = 1;
    for pile in piles {
        for i in 1..pile {
            let action = Action {
                pile,
                nombre_enleve: i,
            };
            let action_nettoye = ActionAvecPoids { action, poids };
            actions.push(action_nettoye);
        }
        pile_index += 1;
    }
    actions
}

pub fn train(mut piles: Vec<u8>, number_of_games: u32) -> HashMap<Vec<u8>, Action> {
    let mut dictionary_of_position = HashMap::new();
    
    let mut sorted_piles = &mut piles;
    sorted_piles.sort();

    for i in 0..(sorted_piles[0] + 1) {
        for j in i..(sorted_piles[1] + 1) {
            let position = vec![i, j];
            let mut actions = actions_possibles(sorted_piles.to_vec(), 0);
            dictionary_of_position.insert(position, actions);
        }
    }

    for _ in 0..number_of_games {
        let mut mut_piles = &piles;
        while zero_partout(mut_piles) == false {}
    }

    nettoyer_hashmap(dictionary_of_position)
}

fn action_avec_poids_maximal(liste_action: &Vec<ActionAvecPoids>) -> Action {
    let mut best_action = &liste_action[0];
    for i in 0..liste_action.len() {
        let next_action = &liste_action[i + 1];
        if next_action.poids > best_action.poids {
            best_action = next_action;
        }
    }
    best_action.action
}

fn nettoyer_hashmap(hashmap: HashMap<Vec<u8>, Vec<ActionAvecPoids>>) -> HashMap<Vec<u8>, Action> {
    let mut hashmap_nettoye = HashMap::new();
    for (pile, liste_action) in hashmap {
        hashmap_nettoye.insert(pile, action_avec_poids_maximal(&liste_action));
    }
    hashmap_nettoye
}
