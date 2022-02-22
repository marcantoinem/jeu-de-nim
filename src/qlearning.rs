use rand::distributions::WeightedIndex;
use std::collections::HashMap;

// Récompense
// +1 pour une victoire sur toutes les actions
// -1 pour une défaite sur les actions

#[derive(Copy, Clone, Debug)]
pub struct Action {
    pile: u8,
    nombre_enleve: u8,
}

#[derive(Copy, Clone, Debug)]
struct ActionAvecPoids {
    action: Action,
    poids: u32,
}

fn zero_partout(piles: [u8; 2]) -> bool {
    for pile in piles {
        if pile != 0 {
            return false;
        }
    }
    true
}

fn find_xor_zero(piles: [u8; 2]) -> [u8; 2] {
    for i in 1..piles[0] {                                            //1er pile vérif
        if (piles[0] - i) ^ piles[1] == 0{
            let modified_game = [piles[0] - i, piles[1]];
            return modified_game;
        }
    }
    for j in 1..piles[1] {                                            //2e pile vérif
        if piles[0] ^ (piles[1] - j) == 0{
            let modified_game = [piles[0], piles[1] - j];
            return modified_game;
        }
    }
    if piles[0] > 0{
        let modified_game = [piles[0] - 1, piles[1]];
        modified_game
    } else {
        let modified_game = [piles[0], piles[1] - 1];
        modified_game
    }
}

fn actions_possibles(piles: &[u8; 2], poids: u32) -> Vec<ActionAvecPoids> {
    let mut actions = vec![];
    let mut pile_index = 1;
    for pile in piles {
        for i in 1..*pile {
            let action = Action {
                pile: *pile,
                nombre_enleve: i,
            };
            let action_nettoye = ActionAvecPoids { action, poids };
            actions.push(action_nettoye);
        }
        pile_index += 1;
    }
    actions
}

pub fn train(piles: &[u8; 2], number_of_games: u32) -> HashMap<[u8; 2], Action> {
    let mut dictionary_of_position = HashMap::new();
    
    let mut sorted_piles = *piles;
    sorted_piles.sort();

    for i in 0..(sorted_piles[0] + 1) {
        for j in i..(sorted_piles[1] + 1) {
            let position = [i, j];
            let actions = actions_possibles(&sorted_piles, 0);
            dictionary_of_position.insert(position, actions);
        }
    }

    for _ in 0..number_of_games {
        let mut piles = *piles;
        let mut all_piles = vec![piles];
        loop {
            if zero_partout(piles){ break }
            piles = find_xor_zero(piles);
            all_piles.push(piles);

            if zero_partout(piles){ break }
            
            piles = find_xor_zero(piles);
            all_piles.push(piles);
        }

        for position in all_piles {
            println!("{:?}", position);
        }
    }
    // let test = nettoyer_hashmap(dictionary_of_position);
    // println!("{:?}", test);
    // test
    nettoyer_hashmap(dictionary_of_position)
}

fn action_avec_poids_maximal(liste_action: &Vec<ActionAvecPoids>) -> Action {
    let mut best_action = &liste_action[0];
    for i in 0..liste_action.len() {
        let next_action = &liste_action[i];
        if next_action.poids > best_action.poids {
            best_action = next_action;
        }
    }
    best_action.action
}

fn nettoyer_hashmap(hashmap: HashMap<[u8; 2], Vec<ActionAvecPoids>>) -> HashMap<[u8;2], Action> {
    let mut hashmap_nettoye = HashMap::new();
    for (pile, liste_action) in hashmap {
        hashmap_nettoye.insert(pile, action_avec_poids_maximal(&liste_action));
    }
    hashmap_nettoye
}
