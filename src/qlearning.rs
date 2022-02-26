use weighted_rs::{RandWeight, Weight};
use fxhash::FxHashMap;
use derive_more::{Index, IndexMut, IntoIterator};
// Récompense
// +1 pour une victoire sur toutes les actions
// -1 pour une défaite sur les actions

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Action {
    pile: u8,
    nombre_enleve: u8,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct ActionAvecPoids {
    action: Action,
    poids: isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Index, IndexMut, IntoIterator)]
pub struct Piles(pub [u8;2]);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Index, IndexMut, IntoIterator)]
pub struct PilesAvecIndex([(u8, u8);2]);

impl Piles {
    fn ajout_index(self) -> PilesAvecIndex {
        let mut piles_avec_index = PilesAvecIndex([(0,0),(0,0)]);
        let mut index: u8 = 0;
        for pile in self.0 {
            piles_avec_index[index as usize] = (index, pile);
            index += 1;
        }
        piles_avec_index
    }

    fn trie_croissant(&mut self) {
        self.0.sort_by(|a, b| a.cmp(b));
    }

    fn zero_partout(&self) -> bool {
        for pile in self.0 {
            if pile != 0 {
                return false;
            }
        }
        true
    }

    fn _find_xor_zero(&self) -> Piles {
        for i in 1..(self[0] + 1) {
            //1er pile vérif
            if (self[0] - i) ^ self[1] == 0 {
                let modified_game = Piles([self[0] - i, self[1]]);
                return modified_game;
            }
        }
        for j in 1..(self[1] + 1) {
            //2e pile vérif
            if self[0] ^ (self[1] - j) == 0 {
                let modified_game = Piles([self[0], self[1] - j]);
                return modified_game;
            }
        }
        if self[0] > 0 {
            let modified_game = Piles([self[0] - 1, self[1]]);
            modified_game
        } else {
            let modified_game = Piles([self[0], self[1] - 1]);
            modified_game
        }
    }

    fn genere_action(self) -> Vec<ActionAvecPoids> {
        let mut actions = vec![];
        let mut pile_index = 0;
        for pile in self.0 {
            if pile != 0 {
                for i in 1..(pile + 1) {
                    let action = Action {
                        pile: pile_index,
                        nombre_enleve: i,
                    };
                    let action_avec_poids = ActionAvecPoids { action, poids: 1 };
                    actions.push(action_avec_poids);
                }
            }
            pile_index += 1;
        }
        actions
    }
}

impl PilesAvecIndex {
    fn enleve_index(self) -> Piles {
        let mut piles = Piles([(0),(0)]);
        let mut index: u8 = 0;
        for (_, pile) in self {
            piles[index as usize] = pile;
            index += 1;
        }
        piles
    }
    fn trie_croissant(&mut self) {
        self.0.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
    }

    fn trie_original(&mut self) {
        self.0.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    }
}


impl Action {
    fn future_piles(self, piles: Piles) -> Piles {
        let mut future_piles = piles.ajout_index();
        future_piles.trie_croissant();
        let pile_index = self.pile as usize;
        future_piles[pile_index].1 -= self.nombre_enleve;
        future_piles.trie_original();
        future_piles.enleve_index()
    }
}

fn genere_liste_action_avec_poids(vecteur_action_avec_poids: &Vec<ActionAvecPoids>) -> RandWeight<Action> {
    let mut liste_action_avec_poids = RandWeight::new();
    for action_avec_poids in vecteur_action_avec_poids {
        liste_action_avec_poids.add(action_avec_poids.action, action_avec_poids.poids);
    }
    liste_action_avec_poids
}

pub fn train(piles: &Piles, nombre_de_partie: u32) -> FxHashMap<Piles, Action> {
    let mut dictionnaire_de_position = FxHashMap::default();

    let mut piles_triees = piles.clone();
    piles_triees.trie_croissant();

    for i in 0..(piles_triees[0] + 1) {
        for j in i..(piles_triees[1] + 1) {
            let piles = Piles([i, j]);
            let actions = piles.genere_action();
            dictionnaire_de_position.insert(piles, actions);
        }
    }

    for _ in 0..nombre_de_partie {
        let mut piles = piles.clone();
        let mut all_piles = vec![];
        let win = loop {
            if piles.zero_partout() {
                println!("Deuxième joueur");
                break false;
            }

            let mut piles_triees = piles.clone();
            piles_triees.trie_croissant();

            let vecteur = match dictionnaire_de_position.get(&piles_triees) {
                Some(value) => value,
                None => {
                    println!("Erreur");
                    break false;
                }
            };

            let mut liste_action_avec_poids = genere_liste_action_avec_poids(vecteur);
            let action_prise = liste_action_avec_poids.next().unwrap();
            all_piles.push((piles, action_prise));
            piles = action_prise.future_piles(piles);

            if piles.zero_partout() {
                println!("Premier joueur");
                break true;
            }

            let mut piles_triees = piles.clone();
            piles_triees.trie_croissant();
            
            let vecteur = match dictionnaire_de_position.get(&piles_triees) {
                Some(value) => value,
                None => {
                    println!("Erreur");
                    break false;
                }
            };

            let mut liste_action_avec_poids = genere_liste_action_avec_poids(vecteur);
            let action_prise = liste_action_avec_poids.next().unwrap();
            piles = action_prise.future_piles(piles);
        };

        for (piles, action_prise) in all_piles {
            let mut piles = piles;
            piles.trie_croissant();
            let entree = dictionnaire_de_position.get(&piles).unwrap();
            let mut index = 0;
            for element in entree {
                if element.action == action_prise {
                    break;
                }
                index += 1;
            }
            let entree = dictionnaire_de_position.entry(piles).or_default();
            if win {
                entree[index].poids += 1;
            } else if entree[index].poids > 1 {
                entree[index].poids -= 1;
            }
            //*entree.poids += 1;
        }
        // for position in all_piles {
        //     println!("{:?}", position);
        // }
    }
    // println!("{:?}", dictionnaire_de_position);
    // let test = nettoyer_hashmap(dictionnaire_de_position);
    // println!("{:?}", test);
    // test
    // println!("{:#?}", dictionnaire_de_position);
    // println!("{:#?}", dictionnaire_de_position);
    nettoyer_hashmap(dictionnaire_de_position)
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

fn nettoyer_hashmap(hashmap: FxHashMap<Piles, Vec<ActionAvecPoids>>) -> FxHashMap<Piles, Action> {
    let mut hashmap_nettoye = FxHashMap::default();
    for (pile, liste_action) in hashmap {
        hashmap_nettoye.insert(pile, action_avec_poids_maximal(liste_action));
    }
    hashmap_nettoye
}
