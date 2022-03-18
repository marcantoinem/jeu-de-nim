use derive_more::{Index, IndexMut, IntoIterator};
use fxhash::FxHashMap;
use rand::Rng;
use std::thread;

const MINIMUM: f64 = 0.001;
const MAXIMUM: f64 = 40.0;
const NB_DE_PILE: usize = 8;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Action {
    pile: u8,
    nb_enleve: u8,
}

// #[derive(Copy, Clone, Debug, PartialEq)]
// struct ActionQualité {
//     action: Action,
//     qualité: f64,
// }

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Paramètres {
    pub alpha: f64,
    pub gamma: f64,
    pub beta: f64,
    pub récompense: f64,
    pub punition: f64,
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
                for i in 1..=self[index] {
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

    fn genere_action(self) -> FxHashMap<Action, f64> {
        let mut actions = FxHashMap::default();
        let mut pile_index = 0;
        for pile in self.0 {
            if pile != 0 {
                for i in 1..=pile {
                    let action = Action {
                        pile: pile_index,
                        nb_enleve: i,
                    };
                    actions.insert(action, 1.0);
                }
            }
            pile_index += 1;
        }
        actions
    }

    fn genere_hashmap(self) -> FxHashMap<Piles, FxHashMap<Action, f64>> {
        let mut piles_triées = self;
        piles_triées.trie_croissant();
        let mut hashmap = FxHashMap::default();

        for i in 0..=piles_triées[0] {
            for j in i..=piles_triées[1] {
                for k in j..=piles_triées[2] {
                    for l in k..=piles_triées[3] {
                        for m in l..=piles_triées[4] {
                            for n in m..=piles_triées[5] {
                                for o in n..=piles_triées[6] {
                                    for p in o..=piles_triées[7] {
                                        let piles = Piles([i, j, k, l, m, n, o, p]);
                                        let actions = piles.genere_action();
                                        hashmap.insert(piles, actions);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        hashmap
    }

    fn cherche_action(self, hashmap: &FxHashMap<Piles, FxHashMap<Action, f64>>) -> &Action {
        let mut piles_triées = self;
        piles_triées.trie_croissant();

        let vecteur = hashmap
            .get(&piles_triées)
            .expect("Erreur lors de la recherche de position.");

        choisis_action(vecteur)
    }

    fn teste_victoire(&self, nb_partie: u64, nb_modèle: u32, p: Paramètres) -> u32 {
        let mut nb_victoire = 0;
        for _ in 0..nb_modèle {
            let hashmap = entraine(&self, nb_partie, p);
            // let temps_écoulé = maintenant.elapsed();
            nb_victoire += victoire_parfaite(*self, hashmap) as u32;
        }
        nb_victoire
    }

    pub fn teste_fiabilité(
        self,
        nb_partie: u64,
        nb_modèle: u32,
        nb_travailleur: u32,
        p: Paramètres,
    ) -> f64 {
        let mut travailleurs = Vec::new();

        for _ in 0..nb_travailleur {
            let travailleur = thread::spawn(move || {
                return self.teste_victoire(nb_partie, nb_modèle, p);
            });
            travailleurs.push(travailleur);
        }

        let mut nb_victoire = 0;
        for travailleur in travailleurs {
            let resultat: u32 = travailleur.join().unwrap();
            nb_victoire += resultat;
        }

        nb_victoire as f64 / (nb_modèle * nb_travailleur) as f64
    }

    pub fn nb_coup(self) -> u32 {
        let mut nb_coup = 0;
        let mut piles = self;
        while piles.zero_partout() != true {
            piles = piles.trouver_xor_zero();
            nb_coup += 1;
        }
        // for index in 0..NB_DE_PILE {
        //     nb_coup += self[index as usize] as u32 * nb_coup;
        // }
        nb_coup
    }
}

impl PilesIndex {
    pub fn enleve_index(self) -> Piles {
        let mut piles = Piles([(0); NB_DE_PILE]);
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

fn choisis_action(hashmap: &FxHashMap<Action, f64>) -> &Action {
    let mut somme = 0.0;
    for entrée in hashmap {
        somme += entrée.1;
        // somme += (action_qualité.qualité/0.389).exp();
    }

    let mut rng = rand::thread_rng();
    let mut valeur_aléatoire: f64 = rng.gen();

    for entrée in hashmap {
        valeur_aléatoire -= entrée.1 / somme;
        // valeur_aléatoire -= (action_qualité.qualité/0.389).exp() / somme;
        if valeur_aléatoire <= 0.0 {
            return entrée.0;
        }
    }

    hashmap.keys().next().unwrap()
    // return hashmap.
}

// Algorithme Epsilon-Greedy
// fn choisis_action(vecteur: &Vec<ActionQualité>, epsilon: f64) -> Action {
//     let mut rng = rand::thread_rng();
//     let valeur_aléatoire: f64 = rng.gen();
//
//     if valeur_aléatoire < epsilon {
//         let index = rng.gen_range(0..vecteur.len());
//         return vecteur[index].action;
//     } else {
//         return action_qualité_maximale(vecteur);
//     }
// }

pub fn entraine(piles: &Piles, nb_partie: u64, p: Paramètres) -> FxHashMap<Piles, Action> {
    let mut hashmap = piles.genere_hashmap();
    let mut beta = 0.0;

    for nb in 0..nb_partie {
        let mut piles = *piles;
        let mut partie = vec![];
        let win = loop {
            if piles.zero_partout() {
                // Victoire deuxième joueur
                break false;
            }

            let action_prise = piles.cherche_action(&hashmap);
            partie.push((piles, *action_prise));
            piles = action_prise.future_piles(piles);

            if piles.zero_partout() {
                // Victoire premier joueur
                break true;
            }

            let action_prise = piles.cherche_action(&hashmap);
            piles = action_prise.future_piles(piles);
        };

        partie.reverse();

        let mut qualité_dbs = 1.0;

        for (piles, action_prise) in partie {
            let mut piles = piles;
            piles.trie_croissant();

            // let mut index = 0;

            // for element in entrée {
            //     if element.action == action_prise {
            //         break;
            //     }
            //     index += 1;
            // }
            let entrée = hashmap.entry(piles).or_default();
            // let qualité = entrée.get(action_prise).unwrap();
            let qualité = entrée.entry(action_prise).or_default();
            if win {
                *qualité =
                    (1.0 - p.alpha) * *qualité + p.alpha * (p.récompense + p.gamma * qualité_dbs);
            } else {
                *qualité =
                    (1.0 - p.alpha) * *qualité + p.alpha * (p.punition + p.gamma * qualité_dbs);
            }

            if *qualité < MINIMUM {
                *qualité = MINIMUM
            } else if *qualité > MAXIMUM {
                *qualité = MAXIMUM;
            }
            let entrée = hashmap.get(&piles).unwrap();
            qualité_dbs = qualité_maximale_dbs(entrée, beta)
        }
        beta = p.beta * (nb * nb) as f64 / (nb_partie * nb_partie) as f64;
    }
    nettoyer_hashmap(hashmap)
}

fn qualité_maximale_dbs(liste_action: &FxHashMap<Action, f64>, beta: f64) -> f64 {
    if liste_action.len() == 0 {
        return 1.0;
    }
    // Source : https://www.ijcai.org/proceedings/2020/0276.pdf
    let mut dbs: f64 = 0.0;
    let mut somme: f64 = 0.0;

    for action_qualité in liste_action {
        somme += (beta * action_qualité.1).exp();
    }

    for action_qualité in liste_action {
        dbs += (beta * action_qualité.1).exp() * action_qualité.1 / somme;
    }

    dbs as f64
}

fn action_qualité_maximale(liste_action: FxHashMap<Action, f64>) -> Action {
    if liste_action.len() == 0 {
        return Action {
            pile: 0,
            nb_enleve: 0,
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

fn nettoyer_hashmap(hashmap: FxHashMap<Piles, FxHashMap<Action, f64>>) -> FxHashMap<Piles, Action> {
    let mut hashmap_nettoyé = FxHashMap::default();
    for (pile, liste_action) in hashmap {
        hashmap_nettoyé.insert(pile, action_qualité_maximale(liste_action));
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

        piles = action_prise.future_piles(piles);

        if piles.zero_partout() {
            return true;
        }

        piles = piles.trouver_xor_zero();
    }
}
