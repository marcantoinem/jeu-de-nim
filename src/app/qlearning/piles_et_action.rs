use derive_more::{Index, IndexMut, IntoIterator};
use fxhash::FxHashMap;
use rand::Rng;
use std::fmt;

// C'est le nombre de pile, la seule partie du code à changer si on
// change cette constante est la fonction genere_hashmap() dans ce fichier
pub const NB_DE_PILE: usize = 8;

// Cette structure représente une action en fonction de l'index de la
// pile où elle est effectuée et le nombre d'objet enlevé.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Action {
    pub pile: u8,
    pub nb_enlevé: u8,
}

// Cette structure regroupe toutes les constantes nécessaires pour le 
// calcul du Q-learning.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Paramètres {
    pub alpha: f64,
    pub gamma: f64,
    pub k: f64,
    pub récompense: f64,
}

// Cette structure représente les piles.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Index, IndexMut, IntoIterator)]
pub struct Piles(pub [u8; NB_DE_PILE]);

// Cette implémentation sert à afficher les piles.
impl fmt::Display for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut piles_str = String::new();
        for pile in self.0 {
            if pile != 0 {
                piles_str.push_str(&format!("{} ", pile));
            }
        }
        write!(f, "{}", piles_str)
    }
}

// Cette structure représente des piles avec l'index initial
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Index, IndexMut, IntoIterator)]
pub struct PilesIndex([(u8, u8); NB_DE_PILE]);

// Toutes les implémentations relatives à la structures piles
impl Piles {
    // Cette fonction calcule le xor des piles
    pub fn xor(&self) -> u8 {
        let mut xor = 0;
        for pile in *self {
            // ^ est l'opérateur xor
            xor ^= pile;
        }
        xor
    }

    // Cette fonction transforme des la structure Piles en PilesIndex
    pub fn ajout_index(self) -> PilesIndex {
        let mut piles_avec_index = PilesIndex([(0, 0); NB_DE_PILE]);
        for (index, pile) in self.0.into_iter().enumerate() {
            piles_avec_index[index] = (index as u8, pile);
        }
        piles_avec_index
    }

    // Cette fonction trie en ordre croissant les piles
    // !Attention irréversible (pas d'index)
    pub fn trie_croissant(&mut self) {
        self.0.sort_by(|a, b| a.cmp(b));
    }

    // Cette fonction vérifie si toutes les piles sont vides.
    pub fn zero_partout(&self) -> bool {
        for pile in self.0 {
            if pile != 0 {
                return false;
            }
        }
        true
    }

    // Cette fonction est la solution du jeu de Nim. Elle consiste
    // à choisir l'action qui mène à un xor de zéro.
    pub fn trouver_xor_zero(self) -> Piles {
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

    // Cette fonction génère les actions possibles pour des piles
    // et attribue la qualité par défaut.
    fn genere_action(self) -> FxHashMap<Action, f64> {
        let mut actions = FxHashMap::default();
        for (index, pile) in self.0.into_iter().enumerate() {
            if pile != 0 {
                for i in 1..=pile {
                    let action = Action {
                        pile: index as u8,
                        nb_enlevé: i,
                    };
                    actions.insert(action, 1.0);
                }
            }
        }
        actions
    }

    // Cette fonction génère l'hashmap initiale en ne tenant pas compte
    // de l'ordre des piles.
    pub fn genere_hashmap(self) -> FxHashMap<Piles, FxHashMap<Action, f64>> {
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

    // Cette fonction cherche dans l'hashmap du qlearning et choisis une action.
    pub fn cherche_action(self, hashmap: &FxHashMap<Piles, FxHashMap<Action, f64>>) -> &Action {
        let mut piles_triées = self;
        piles_triées.trie_croissant();

        let vecteur = hashmap
            .get(&piles_triées)
            .expect("Erreur lors de la recherche de position.");

        choisis_action(vecteur)
    }

    // Cette fonction compte le nombre de coups nécessaires pour résoudre
    // les piles.
    pub fn nb_coup(self) -> u32 {
        let mut nb_coup = 0;
        let mut piles = self;
        while !piles.zero_partout() {
            piles = piles.trouver_xor_zero();
            nb_coup += 1;
        }
        nb_coup
    }
}

// Cette fonction sert à choisir une action avec une probabilité
// au ratio de qualité de l'action sur la somme des qualités.
// (Échantillonage de Thompson, voir la section 3.5 du rapport)
fn choisis_action(hashmap: &FxHashMap<Action, f64>) -> &Action {
    let mut somme = 0.0;
    for entrée in hashmap {
        somme += entrée.1;
    }

    let mut rng = rand::thread_rng();
    let mut valeur_aléatoire: f64 = rng.gen();

    for entrée in hashmap {
        valeur_aléatoire -= entrée.1 / somme;
        if valeur_aléatoire <= 0.0 {
            return entrée.0;
        }
    }

    hashmap.keys().next().unwrap()
}

// Cette fonction sert à choisir l'action maximale avec 90% de chance
// et une action aléatoire avec 10% de chance
// (algorithme Epsilon-Gloûton, voir la section 3.5 du rapport)
fn _choisis_action(hashmap: &FxHashMap<Action, f64>) -> &Action {
    let vecteur = Vec::from_iter(hashmap.iter());
    let mut rng = rand::thread_rng();
    let valeur_aléatoire: f64 = rng.gen();

    if valeur_aléatoire < 0.1 {
        let index = rng.gen_range(0..vecteur.len());
        return vecteur[index].0;
    } else {
        return &_vecteur_max(&vecteur);
    }
}

// Cette fonction sélectionne l'action avec la qualité maximale dans un vecteur.
// Cette fonction sert uniquement pour la fonction _choisir_action().
fn _vecteur_max<'a>(liste_action: &Vec<(&'a Action, &f64)>) -> &'a Action {
    if liste_action.is_empty() {
        return &Action {
            pile: 0,
            nb_enlevé: 0,
        };
    }
    let mut iterator = liste_action.into_iter();
    let mut meilleure_action = iterator.next().unwrap();
    for action_qualité in iterator {
        if action_qualité.1 > meilleure_action.1 {
            meilleure_action = action_qualité;
        }
    }
    &meilleure_action.0
}

// Implémentation relative à la structure PilesIndex
impl PilesIndex {
    // Cette fonction transforme une structure PilesIndex en Piles.
    pub fn enleve_index(self) -> Piles {
        let mut piles = Piles([(0); NB_DE_PILE]);
        for (index, (_, pile)) in self.into_iter().enumerate() {
            piles[index] = pile;
        }
        piles
    }

    // Cette fonction trie en ordre croissant les piles
    pub fn trie_croissant(&mut self) {
        self.0.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
    }

    // Cette fonction retourne les piles à leur ordre initial
    pub fn trie_original(&mut self) {
        self.0.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    }
}

// Implémentation relative à la structure Action
impl Action {
    // Applique une action (pour des piles triées) à des piles
    // et retourne les piles résultantes.
    pub fn future_piles(self, piles: Piles) -> Piles {
        let mut future_piles = piles.ajout_index();
        future_piles.trie_croissant();
        let index = self.pile as usize;
        future_piles[index].1 -= self.nb_enlevé;
        future_piles.trie_original();
        future_piles.enleve_index()
    }
}


