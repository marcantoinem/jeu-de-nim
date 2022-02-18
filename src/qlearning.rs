use std::collections::HashMap;

// Récompense
// +1 pour une victoire sur toutes les actions
// -1 pour une défaite sur les actions

pub struct Action {
    pile: u8,
    nombre_enleve: u8,
}

struct ActionAvecPoids {
    pile: u8,
    nombre_enleve: u8,
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
fn action_possible(piles: &Vec<u8>) -> Vec<Action> {
    let mut actions = vec![];
    let mut pile_index = 1;
    for pile in piles {
        for i in 1..*pile {
            let action = Action {
                pile: pile_index,
                nombre_enleve: i,
            };
            actions.push(action);
        }
        pile_index += 1;
    }
    actions
}

fn nettoyer_hashmap(hashmap: HashMap<Vec<u8>, Vec<ActionAvecPoids>>) -> HashMap<Vec<u8>, Action> {
    let hashmap_nettoye = HashMap::new();
    for (pile, liste_action) in hashmap {

    }
    hashmap_nettoye
}

pub fn train(piles: Vec<u8>, number_of_games: u32) -> HashMap<Vec<u8>, Action> {
    let mut dictionary_of_position = HashMap::new();
    for i in 0..piles[0] {
        for j in 0..piles[1] {
            let position = vec![i, j];
            let mut actions = vec![];
            for x in 1..i {
                let action = ActionAvecPoids {
                    pile: 1,
                    nombre_enleve: x,
                    poids: 0,
                };
                actions.push(action);
            }
            for y in 1..i {
                let action = ActionAvecPoids {
                    pile: 2,
                    nombre_enleve: y,
                    poids: 0,
                };
                actions.push(action);
            }
            dictionary_of_position.insert(position, actions);
        }
    }
    for _ in 0..number_of_games {
        let mut mut_piles = &piles;
        while zero_partout(mut_piles) == false {

        }
    }
    nettoyer_hashmap(dictionary_of_position)
}
