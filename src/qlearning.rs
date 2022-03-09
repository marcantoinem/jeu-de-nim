use derive_more::{Index, IndexMut, IntoIterator};
use fxhash::FxHashMap;
use rand::Rng;
use std::thread;

const MINIMUM: f32 = 0.001;
const NB_DE_PILE: usize = 4;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Action {
    pile: u8,
    nb_enleve: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ActionQualité {
    action: Action,
    qualité: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Paramètres {
    pub alpha: f32,
    pub gamma: f32,
    pub récompense: f32,
    pub punition: f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Index, IndexMut, IntoIterator)]
pub struct Piles(pub [u8; NB_DE_PILE]);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Index, IndexMut, IntoIterator)]
pub struct PilesIndex([(u8, u8); NB_DE_PILE]);

impl Piles {
    pub fn xor(&self) -> u8 {
        let mut xor = 0;
        for pile in *self {
            xor ^= pile;
        }
        xor
    }

    pub fn ajout_index(self) -> PilesIndex {
        let mut piles_avec_index = PilesIndex([(0, 0); NB_DE_PILE]);
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
        for index in 0..NB_DE_PILE {
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
        Piles([0; NB_DE_PILE])
    }

    fn genere_action(self) -> Vec<ActionQualité> {
        let mut actions = vec![];
        let mut pile_index = 0;
        for pile in self.0 {
            if pile != 0 {
                for i in 1..(pile + 1) {
                    let action = Action {
                        pile: pile_index,
                        nb_enleve: i,
                    };
                    let action_avec_qualité = ActionQualité {
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

    fn cherche_action(self, hashmap: &FxHashMap<Piles, Vec<ActionQualité>>) -> Action {
        let mut piles_triées = self;
        piles_triées.trie_croissant();

        let vecteur = hashmap
            .get(&piles_triées)
            .expect("Erreur lors de la recherche de position.");

        choisis_action(vecteur)
    }

    fn teste_victoire(&self, nb_partie: u32, nb_modele: u32, p: Paramètres) -> u32 {
        let mut nb_victoire = 0;
        for _ in 0..nb_modele {
            let hashmap = entraine(&self, nb_partie, p);
            // let temps_écoulé = maintenant.elapsed();
            nb_victoire += victoire_parfaite(*self, hashmap) as u32;
        }
        nb_victoire
    }

    pub fn teste_fiabilité(
        self,
        nb_partie: u32,
        nb_modele_par_travailleur: u32,
        nb_travailleur: u32,
        p: Paramètres,
    ) -> f32 {
        let mut travailleurs = Vec::new();

        for _ in 0..nb_travailleur {
            let travailleur = thread::spawn(move || {
                return self.teste_victoire(nb_partie, nb_modele_par_travailleur, p);
            });
            travailleurs.push(travailleur);
        }

        let mut nb_victoire = 0;
        for travailleur in travailleurs {
            let resultat: u32 = travailleur.join().unwrap();
            nb_victoire += resultat;
        }

        nb_victoire as f32 / (nb_modele_par_travailleur * nb_travailleur) as f32
    }

    pub fn difficulté(self) -> u32 {
        let mut difficulté = 0;
        let mut piles = self;
        while piles.zero_partout() != true {
            piles = piles.trouver_xor_zero();
            difficulté += 1;
        }
        for index in 0..NB_DE_PILE {
            difficulté += self[index as usize] as u32 * difficulté;
        }
        difficulté
    }
}

impl PilesIndex {
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
        future_piles[pile_index].1 -= self.nb_enleve;
        future_piles.trie_original();
        future_piles.enleve_index()
    }
}

fn choisis_action(vecteur: &Vec<ActionQualité>) -> Action {
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

// Algorithme Epsilon-Greedy
// fn choisis_action(vecteur: &Vec<ActionQualité>, epsilon: f32) -> Action {
//     let mut rng = rand::thread_rng();
//     let valeur_aléatoire: f32 = rng.gen();
//
//     if valeur_aléatoire < epsilon {
//         let index = rng.gen_range(0..vecteur.len());
//         return vecteur[index].action;
//     } else {
//         return action_avec_qualité_maximale(vecteur);
//     }
// }

pub fn entraine(piles: &Piles, nb_partie: u32, p: Paramètres) -> FxHashMap<Piles, Action> {
    let mut hashmap = FxHashMap::default();
    let mut piles_triées = *piles;
    piles_triées.trie_croissant();

    for i in 0..(piles_triées[0] + 1) {
        for j in i..(piles_triées[1] + 1) {
            for k in j..(piles_triées[2] + 1) {
                for l in k..(piles_triées[3] + 1) {
                    let piles = Piles([i, j, k, l]);
                    let actions = piles.genere_action();
                    hashmap.insert(piles, actions);
                }
            }
        }
    }

    for _ in 0..nb_partie {
        let mut piles = *piles;
        let mut partie = vec![];
        let win = loop {
            if piles.zero_partout() {
                // Victoire deuxième joueur
                break false;
            }

            let action_prise = piles.cherche_action(&hashmap);
            partie.push((piles, action_prise));
            piles = action_prise.future_piles(piles);

            if piles.zero_partout() {
                // Victoire premier joueur
                break true;
            }

            let action_prise = piles.cherche_action(&hashmap);
            piles = action_prise.future_piles(piles);
        };

        partie.reverse();

        let mut action_future = vec![];

        for (piles, action_prise) in partie {
            let mut piles = piles;
            piles.trie_croissant();
            let entrée = hashmap.get(&piles).unwrap();
            let mut index = 0;
            for element in entrée {
                if element.action == action_prise {
                    break;
                }
                index += 1;
            }

            let entrée = hashmap.entry(piles).or_default();
            if win {
                entrée[index].qualité = (1.0 - p.alpha) * entrée[index].qualité
                    + p.alpha * (p.récompense + p.gamma * qualité_maximale(action_future));
            } else if entrée[index].qualité > 0.0 {
                entrée[index].qualité = (1.0 - p.alpha) * entrée[index].qualité
                    + p.alpha * (p.punition + p.gamma * qualité_maximale(action_future));
            }

            if entrée[index].qualité < 0.0 {
                entrée[index].qualité = MINIMUM
            }

            action_future = entrée.clone();
        }
    }
    nettoyer_hashmap(hashmap)
}

fn qualité_maximale(liste_action: Vec<ActionQualité>) -> f32 {
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

fn action_avec_qualité_maximale(liste_action: &Vec<ActionQualité>) -> Action {
    if liste_action.len() == 0 {
        return Action {
            pile: 0,
            nb_enleve: 0,
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

fn nettoyer_hashmap(hashmap: FxHashMap<Piles, Vec<ActionQualité>>) -> FxHashMap<Piles, Action> {
    let mut hashmap_nettoyé = FxHashMap::default();
    for (pile, liste_action) in hashmap {
        hashmap_nettoyé.insert(pile, action_avec_qualité_maximale(&liste_action));
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
        // println!("{:?}", piles.xor());
        piles = action_prise.future_piles(piles);

        if piles.zero_partout() {
            return true;
        }
        // println!("{:?}", piles.xor());
        piles = piles.trouver_xor_zero();
    }
}
