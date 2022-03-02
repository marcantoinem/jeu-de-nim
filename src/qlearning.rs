use derive_more::{Index, IndexMut, IntoIterator};
use fxhash::FxHashMap;
use rand::Rng;

// Récompense
// +1 pour une victoire sur toutes les actions
// -1 pour une défaite sur les actions

const ALPHA: f32 = 0.9;
const GAMMA: f32 = 0.5;
const RÉCOMPENSE: f32 = 2.0;
const MINIMUM: f32 = 0.0001;

const NOMBRE_DE_PILE: usize = 4;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Action {
    pile: u8,
    nombre_enleve: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ActionAvecQualité {
    action: Action,
    qualité: f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Index, IndexMut, IntoIterator)]
pub struct Piles(pub [u8; NOMBRE_DE_PILE]);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Index, IndexMut, IntoIterator)]
pub struct PilesAvecIndex([(u8, u8); NOMBRE_DE_PILE]);

impl Piles {
    pub fn xor(&self) -> u8 {
        let mut xor = 0;
        for pile in *self {
            xor ^= pile;
        }
        xor
    }

    pub fn ajout_index(self) -> PilesAvecIndex {
        let mut piles_avec_index = PilesAvecIndex([(0, 0); NOMBRE_DE_PILE]);
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

    pub fn zero_partout(&self) -> bool {
        for pile in self.0 {
            if pile != 0 {
                return false;
            }
        }
        true
    }

    fn trouver_xor_zero(self) -> Piles {
        for index in 0..NOMBRE_DE_PILE {
            if self[index] != 0 {
                for i in 1..(self[index] + 1) {
                    let mut piles_futures = self;
                    piles_futures[index] -= i;
                    if piles_futures.xor() == 0 {
                        return piles_futures;
                    }
                }
            }
        }
        for index in 0..self.0.len() {
            if self[index] > 0 {
                let mut piles_futures = self;
                piles_futures[index] -= 1;
                return piles_futures;
            }
        }
        Piles([0; NOMBRE_DE_PILE])
    }

    fn genere_action(self) -> Vec<ActionAvecQualité> {
        let mut actions = vec![];
        let mut pile_index = 0;
        for pile in self.0 {
            if pile != 0 {
                for i in 1..(pile + 1) {
                    let action = Action {
                        pile: pile_index,
                        nombre_enleve: i,
                    };
                    let action_avec_qualité = ActionAvecQualité {
                        action,
                        qualité: 1.0,
                    };
                    actions.push(action_avec_qualité);
                }
            }
            pile_index += 1;
        }
        actions
    }
}

impl PilesAvecIndex {
    pub fn enleve_index(self) -> Piles {
        let mut piles = Piles([(0), (0), (0), (0)]);
        let mut index: u8 = 0;
        for (_, pile) in self {
            piles[index as usize] = pile;
            index += 1;
        }
        piles
    }
    pub fn trie_croissant(&mut self) {
        self.0.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
    }

    pub fn trie_original(&mut self) {
        self.0.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    }
}

impl Action {
    pub fn future_piles(self, piles: Piles) -> Piles {
        let mut future_piles = piles.ajout_index();
        future_piles.trie_croissant();
        let pile_index = self.pile as usize;
        future_piles[pile_index].1 -= self.nombre_enleve;
        future_piles.trie_original();
        future_piles.enleve_index()
    }
}

fn cherche_et_choisis_action(
    piles: Piles,
    dictionnaire_de_position: &FxHashMap<Piles, Vec<ActionAvecQualité>>,
) -> Action {
    let mut piles_triées = piles.clone();
    piles_triées.trie_croissant();

    let vecteur = dictionnaire_de_position
        .get(&piles_triées)
        .expect("Erreur lors de la recherche de position.");

    choisis_action(vecteur)
}

fn choisis_action(vecteur: &Vec<ActionAvecQualité>) -> Action {
    let mut somme = 0.0;
    for action_avec_qualité in vecteur {
        somme += action_avec_qualité.qualité;
    }

    let mut rng = rand::thread_rng();
    let mut valeur_aléatoire: f32 = rng.gen();

    for action_avec_qualité in vecteur {
        valeur_aléatoire -= action_avec_qualité.qualité / somme;
        if valeur_aléatoire <= 0.0 {
            return action_avec_qualité.action;
        }
    }

    return vecteur[0].action;
}

pub fn entraine(piles: &Piles, nombre_de_partie: u32) -> FxHashMap<Piles, Action> {
    let mut dictionnaire_de_position = FxHashMap::default();
    let mut points = vec![];
    let mut nb_de_win = 0;
    let mut piles_triées = piles.clone();
    piles_triées.trie_croissant();

    for i in 0..(piles_triées[0] + 1) {
        for j in i..(piles_triées[1] + 1) {
            for k in j..(piles_triées[2] + 1) {
                for l in k..(piles_triées[3] + 1) {
                    let piles = Piles([i, j, k, l]);
                    let actions = piles.genere_action();
                    dictionnaire_de_position.insert(piles, actions);
                }
            }
        }
    }

    for nb in 1..=nombre_de_partie {
        let mut piles = piles.clone();
        let mut partie = vec![];
        let win = loop {
            if piles.zero_partout() {
                // println!("Deuxième joueur");
                break false;
            }

            let action_prise = cherche_et_choisis_action(piles, &dictionnaire_de_position);
            partie.push((piles, action_prise));
            piles = action_prise.future_piles(piles);

            if piles.zero_partout() {
                // println!("Premier joueur");
                nb_de_win += 1;
                break true;
            }

            let action_prise = cherche_et_choisis_action(piles, &dictionnaire_de_position);
            piles = action_prise.future_piles(piles);
        };

        points.push((nb as f32, nb_de_win as f32));
        partie.reverse();

        let mut action_future = vec![];

        for (piles, action_prise) in partie {
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
                entrée[index].qualité = (1.0 - ALPHA) * entrée[index].qualité
                    + ALPHA * (RÉCOMPENSE + GAMMA * qualité_maximale(action_future));
            } else if entrée[index].qualité > 0.0 {
                entrée[index].qualité = (1.0 - ALPHA) * entrée[index].qualité
                    + ALPHA * (-RÉCOMPENSE + GAMMA * qualité_maximale(action_future));
            }

            if entrée[index].qualité < 0.0 {
                entrée[index].qualité = MINIMUM
            }

            action_future = entrée.clone();
        }
    }
    nettoyer_hashmap(dictionnaire_de_position)
}

fn qualité_maximale(liste_action: Vec<ActionAvecQualité>) -> f32 {
    if liste_action.len() == 0 {
        return 1.0;
    }
    let mut meilleure_action = &liste_action[0];
    for i in 0..liste_action.len() {
        let next_action = &liste_action[i];
        if next_action.qualité > meilleure_action.qualité {
            meilleure_action = next_action;
        }
    }
    meilleure_action.qualité
}

fn action_avec_qualité_maximale(liste_action: Vec<ActionAvecQualité>) -> Action {
    if liste_action.len() == 0 {
        return Action {
            pile: 0,
            nombre_enleve: 0,
        };
    }
    let mut meilleure_action = &liste_action[0];
    for i in 0..liste_action.len() {
        let next_action = &liste_action[i];
        if next_action.qualité > meilleure_action.qualité {
            meilleure_action = next_action;
        }
    }
    meilleure_action.action
}

fn nettoyer_hashmap(
    hashmap: FxHashMap<Piles, Vec<ActionAvecQualité>>
) -> FxHashMap<Piles, Action> {
    let mut hashmap_nettoyé = FxHashMap::default();
    for (pile, liste_action) in hashmap {
        hashmap_nettoyé.insert(pile, action_avec_qualité_maximale(liste_action));
    }
    hashmap_nettoyé
}

pub fn victoire_parfaite(piles_originales: Piles, hashmap: FxHashMap<Piles, Action>) -> bool {
    let mut piles = piles_originales;
    loop {
        if piles.zero_partout() {
            return false;
        }

        let mut piles_triées = piles.clone().ajout_index();
        piles_triées.trie_croissant();

        let action_prise = match hashmap.get(&piles_triées.enleve_index()) {
            Some(value) => value,
            None => {
                return false;
            }
        };
        println!("{:?}", piles.xor());
        piles = action_prise.future_piles(piles);

        if piles.zero_partout() {
            return true;
        }
        println!("{:?}", piles.xor());
        piles = piles.trouver_xor_zero();
    }
}
