#[derive(Debug)]
                                                                          //algorithme pour vérifier quel coups est possible pour que le xor arrive à 0.
fn find_xor_zero(piles: Vec<u8>) -> Vec<u8> {
    for i in 1..piles[0] {                                            //1er pile vérif
        if (piles[0] - i) ^ piles[1] == 0{
            let modified_game = vec![piles[0] - i, piles[1]];
            return modified_game;
        }
    }
    for j in 1..piles[1] {                                            //2e pile vérif
        if piles[0] ^ (piles[1] - j) == 0{
            let modified_game = vec![piles[0], piles[1] - j];
            return modified_game;
        }
    }
    if piles[0] > 0{
        let modified_game = vec![piles[0] - 1, piles[1]];
        modified_game
    } else {
        let modified_game = vec![piles[0], piles[1] - 1];
        modified_game
    }
}

//[] = notation dans vecteur

//créer vecteur pour contenir les mouvements sans ordre
//loop : part de 0 pour une variable "w" jusqu'à la fin de la pile (nb max)
// chaque début de la loop on rajoute 1 à "w"
// avec un while (tant que) < ou = ???+- nb max
//on va le combiner avec une autre variable "t" (nb de variable combiner dépend du nb de pile total) qui part de la même valeur que la variable "w"
// pousser la combinaison des variables dans le vecteur et retour au début de la loop
