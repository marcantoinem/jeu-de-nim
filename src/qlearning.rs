use std::collections::HashMap;

// Récompense
// +1 pour une victoire sur toutes les actions
// -1 pour une défaite sur les actions

struct ActionAvecPoids {
    pile: u8,
    nombre_enleve: u8,
    poids: u32,
}

#[derive(Hash, Eq, PartialEq)]
pub struct Piles {
    pub piles: Vec<u8>,
}

impl Piles {
    fn zero_partout(&self) -> bool {
        for pile in &self.piles {
            if pile != &0 {
                return false;
            }
        }
        true
    }
    fn action_possible(&self) -> Vec<Action> {
        let mut actions = vec![];
        let mut pile_index = 1;
        for pile in &self.piles {
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
}

pub struct Action {
    pile: u8,
    nombre_enleve: u8,
}

fn nettoyer_hashmap(Entree: HashMap<Piles, Vec<ActionAvecPoids>>) -> HashMap<Piles, Action> {
    let temp = HashMap::new();
    temp
}

pub fn train(game: Piles, number_of_games: u32) -> HashMap<Piles, Action> {
    let mut dictionary_of_position = HashMap::new();
    for i in 0..game.piles[0] {
        for j in 0..game.piles[1] {
            let position = Piles { piles: vec![i, j] };
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
        let mut default_game = &game;

        while game.zero_partout() == false {
        
        }
    }
    nettoyer_hashmap(dictionary_of_position)
}
