use std::collections::HashMap;
use weighted_rs::{RandWeight, Weight};

// Récompense
// +1 pour une victoire sur toutes les actions
// -1 pour une défaite sur les actions

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Action {
    pile: u8,
    nombre_enleve: u8,
}

impl Action {
    fn future_piles(self, piles: [(u8, u8); 2]) -> [u8; 2] {
        let mut future_piles = piles;
        future_piles.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
        let pile_index = self.pile as usize;
        future_piles[pile_index].1 -= self.nombre_enleve;
        future_piles.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
        let mut piles_simplifie: [u8; 2] = [0; 2];
        let mut index = 0;
        for (_, pile) in future_piles {
            piles_simplifie[index] = pile;
            index += 1;
        }
        piles_simplifie
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct ActionAvecPoids {
    action: Action,
    poids: isize,
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
    for i in 1..(piles[0] + 1) {
        //1er pile vérif
        if (piles[0] - i) ^ piles[1] == 0 {
            let modified_game = [piles[0] - i, piles[1]];
            return modified_game;
        }
    }
    for j in 1..(piles[1] + 1) {
        //2e pile vérif
        if piles[0] ^ (piles[1] - j) == 0 {
            let modified_game = [piles[0], piles[1] - j];
            return modified_game;
        }
    }
    if piles[0] > 0 {
        let modified_game = [piles[0] - 1, piles[1]];
        modified_game
    } else {
        let modified_game = [piles[0], piles[1] - 1];
        modified_game
    }
}

fn actions_possibles(piles: &[u8; 2]) -> Vec<ActionAvecPoids> {
    let mut actions = vec![];
    let mut pile_index = 0;
    for pile in piles {
        if *pile != 0 {
            for i in 1..(*pile + 1) {
                let action = Action {
                    pile: pile_index,
                    nombre_enleve: i,
                };
                let action_avec_poids = ActionAvecPoids { action, poids: 50 };
                actions.push(action_avec_poids);
            }
        }
        pile_index += 1;
    }
    actions
}

pub fn train(piles: &[u8; 2], number_of_games: u32) -> HashMap<[u8; 2], Action> {
    let mut dictionary_of_position = HashMap::new();

    let mut sorted_piles = *piles;
    sorted_piles.sort_by(|a, b| a.cmp(b));

    for i in 0..(sorted_piles[0] + 1) {
        for j in i..(sorted_piles[1] + 1) {
            let position = [i, j];
            let actions = actions_possibles(&position);
            dictionary_of_position.insert(position, actions);
        }
    }

    for _ in 0..number_of_games {
        let mut piles = *piles;
        let mut all_piles = vec![];
        let win = loop {
            if zero_partout(piles) {
                println!("défaite");
                break false;
            }
            let mut sorted_piles = piles;
            sorted_piles.sort_by(|a, b| a.cmp(b));
            let moves_vec = match dictionary_of_position.get(&sorted_piles) {
                Some(value) => value,
                None => {
                    println!("2");
                    break true;
                }
            };
            let mut moves_list: RandWeight<Action> = RandWeight::new();
            for moves in moves_vec {
                moves_list.add(moves.action, moves.poids);
            }
            let action_prise = moves_list.next().unwrap();

            let mut piles_avec_index = [(0, 0); 2];
            let mut index: u8 = 0;
            for pile in piles {
                piles_avec_index[index as usize] = (index, pile);
                index += 1;
            }
            all_piles.push((piles, action_prise));
            piles = action_prise.future_piles(piles_avec_index);
            if zero_partout(piles) {
                println!("victoire");
                break true;
            }
            piles = find_xor_zero(piles);
        };
        for (piles, action_prise) in all_piles {
            let mut piles = piles;
            piles.sort_by(|a, b| a.cmp(b));
            let entree = dictionary_of_position.get(&piles).unwrap();
            let mut index = 0;
            for element in entree {
                if element.action == action_prise {
                    break;
                }
                index += 1;
            }
            let entree = dictionary_of_position.entry(piles).or_default();
            if win {
                entree[index].poids += 1;
            } else if entree[index].poids > 10 {
                entree[index].poids -= 1;
            }
            //*entree.poids += 1;
        }
        // for position in all_piles {
        //     println!("{:?}", position);
        // }
    }
    // println!("{:?}", dictionary_of_position);
    // let test = nettoyer_hashmap(dictionary_of_position);
    // println!("{:?}", test);
    // test
    // println!("{:#?}", dictionary_of_position);
    nettoyer_hashmap(dictionary_of_position)
}

fn action_avec_poids_maximal(liste_action: Vec<ActionAvecPoids>) -> Action {
    if liste_action.len() == 0 {
        return Action {
            pile: 0,
            nombre_enleve: 0,
        };
    }
    let mut best_action = &liste_action[0];
    for i in 0..liste_action.len() {
        let next_action = &liste_action[i];
        if next_action.poids > best_action.poids {
            best_action = next_action;
        }
    }
    best_action.action
}

fn nettoyer_hashmap(hashmap: HashMap<[u8; 2], Vec<ActionAvecPoids>>) -> HashMap<[u8; 2], Action> {
    let mut hashmap_nettoye = HashMap::new();
    for (pile, liste_action) in hashmap {
        hashmap_nettoye.insert(pile, action_avec_poids_maximal(liste_action));
    }
    hashmap_nettoye
}
