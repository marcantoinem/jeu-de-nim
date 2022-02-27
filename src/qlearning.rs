use fxhash::FxHashMap;
use derive_more::{Index, IndexMut, IntoIterator};
use rand::Rng;
// Récompense
// +1 pour une victoire sur toutes les actions
// -1 pour une défaite sur les actions

const ALPHA: f32 = 1.0;
const GAMMA: f32 = 0.8;
const RÉCOMPENSE: f32 = 2.0;
const MINIMUM: f32 = 0.0001;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Action {
    pile: u8,
    nombre_enleve: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ActionAvecPoids {
    action: Action,
    poids: f32,
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
                let piles_futures = Piles([self[0] - i, self[1]]);
                return piles_futures;
            }
        }
        for j in 1..(self[1] + 1) {
            //2e pile vérif
            if self[0] ^ (self[1] - j) == 0 {
                let piles_futures = Piles([self[0], self[1] - j]);
                return piles_futures;
            }
        }
        if self[0] > 0 {
            let piles_futures = Piles([self[0] - 1, self[1]]);
            piles_futures
        } else {
            let piles_futures = Piles([self[0], self[1] - 1]);
            piles_futures
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
                    let action_avec_poids = ActionAvecPoids { action, poids: 1.0 };
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

fn choisis_action(vecteur: &Vec<ActionAvecPoids>) -> Action {
    let mut somme = 0.0;
    for action_avec_poids in vecteur {
        somme += action_avec_poids.poids;
    }
    
    let mut rng = rand::thread_rng();
    let mut valeur_aléatoire: f32 = rng.gen();
    
    for action_avec_poids in vecteur {
        valeur_aléatoire -= action_avec_poids.poids / somme;
        if valeur_aléatoire <= 0.0 {
            return action_avec_poids.action;
        }
    }

    return vecteur[0].action;
}

pub fn train(piles: &Piles, nombre_de_partie: u32) -> FxHashMap<Piles, Action> {
    let mut dictionnaire_de_position = FxHashMap::default();

    let mut piles_triées = piles.clone();
    piles_triées.trie_croissant();

    for i in 0..(piles_triées[0] + 1) {
        for j in i..(piles_triées[1] + 1) {
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

            let mut piles_triées = piles.clone();
            piles_triées.trie_croissant();

            let vecteur = match dictionnaire_de_position.get(&piles_triées) {
                Some(value) => value,
                None => {
                    println!("Erreur");
                    break false;
                }
            };

            let action_prise = choisis_action(vecteur);
            all_piles.push((piles, action_prise));
            piles = action_prise.future_piles(piles);

            if piles.zero_partout() {
                println!("Premier joueur");
                break true;
            }

            let mut piles_triées = piles.clone();
            piles_triées.trie_croissant();
            
            let vecteur = match dictionnaire_de_position.get(&piles_triées) {
                Some(value) => value,
                None => {
                    println!("Erreur");
                    break false;
                }
            };

            let action_prise = choisis_action(vecteur);
            piles = action_prise.future_piles(piles);
        };

        all_piles.reverse();

        let mut action_future= vec![];

        for (piles, action_prise) in all_piles {
            let mut piles = piles;
            piles.trie_croissant();
            let entrée = dictionnaire_de_position.get(&piles).unwrap();
            let mut index = 0;
            for element in entrée {
                if element.action == action_prise {
                    break;
                }
                index += 1;
            }
            
            let entrée = dictionnaire_de_position.entry(piles).or_default();
            if win {
                entrée[index].poids = entrée[index].poids as f32 + ALPHA * (RÉCOMPENSE + GAMMA * poids_maximal(action_future) - entrée[index].poids as f32);
            } else if entrée[index].poids > 0.0 {
                entrée[index].poids = entrée[index].poids as f32 + ALPHA * (-RÉCOMPENSE + GAMMA * poids_maximal(action_future) - entrée[index].poids as f32);
            }

            if entrée[index].poids < 0.0 {
                entrée[index].poids = MINIMUM
            }

            action_future = entrée.clone();
        }
    }
    nettoyer_hashmap(dictionnaire_de_position)
}

fn poids_maximal(liste_action: Vec<ActionAvecPoids>) -> f32 {
    if liste_action.len() == 0 {
        return 1.0;
    }
    let mut meilleure_action = &liste_action[0];
    for i in 0..liste_action.len() {
        let next_action = &liste_action[i];
        if next_action.poids > meilleure_action.poids {
            meilleure_action = next_action;
        }
    }
    meilleure_action.poids
}

fn action_avec_poids_maximal(liste_action: Vec<ActionAvecPoids>) -> Action {
    if liste_action.len() == 0 {
        return Action {
            pile: 0,
            nombre_enleve: 0,
        };
    }
    let mut meilleure_action = &liste_action[0];
    for i in 0..liste_action.len() {
        let next_action = &liste_action[i];
        if next_action.poids > meilleure_action.poids {
            meilleure_action = next_action;
        }
    }
    meilleure_action.action
}

fn nettoyer_hashmap(hashmap: FxHashMap<Piles, Vec<ActionAvecPoids>>) -> FxHashMap<Piles, Action> {
    let mut hashmap_nettoyé = FxHashMap::default();
    for (pile, liste_action) in hashmap {
        hashmap_nettoyé.insert(pile, action_avec_poids_maximal(liste_action));
    }
    hashmap_nettoyé
}
