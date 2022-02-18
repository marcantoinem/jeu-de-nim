#[derive(Debug)]
struct game {
    pile1: u8,
    pile2: u8,
}

impl game {
    fn new() -> game {                                                      //Poser les valeurs des piles
        game {
            pile1: 5,
            pile2: 4,
        }
    }
                                                                            //algorithme pour vérifier quel coups est possible pour que le xor arrive à 0.
    fn find_xor_zero(&self) -> Option<game> {
        for i in 1..self.pile1 {                                            //1er pile vérif
            if (self.pile1 - i) ^ self.pile2 == 0{
                let modified_game = game {
                    pile1: self.pile1 - i,
                    pile2: self.pile2,
                };
                return Some(modified_game);
            }
        }
        for j in 1..self.pile2 {                                            //2e pile vérif
            if self.pile1 ^ (self.pile2 - j) == 0{
                let modified_game = game {
                    pile1: self.pile1,
                    pile2: self.pile2 - j,
                };
                return Some(modified_game);
            }
        }
        None
    }
}


fn main() {                                                                 //Effectuer le move calculé précédamment
    let mut game = game::new();
    println!("{:?}", game);
    let next_state = game.find_xor_zero();
    match game.find_xor_zero() {
        Some(position) => game = position,
        None => println!("Pssst, c'est la chance du débutant... Baka!"),
    };
    println!("{:?}", game); 
}

//[] = notation dans vecteur

//créer vecteur pour contenir les mouvements sans ordre
//loop : part de 0 pour une variable "w" jusqu'à la fin de la pile (nb max)
// chaque début de la loop on rajoute 1 à "w"
// avec un while (tant que) < ou = ???+- nb max
//on va le combiner avec une autre variable "t" (nb de variable combiner dépend du nb de pile total) qui part de la même valeur que la variable "w"
// pousser la combinaison des variables dans le vecteur et retour au début de la loop
