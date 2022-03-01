use crate::qlearning::Piles;

mod qlearning;
fn main() {
    let piles = Piles([5, 3, 0, 0]);

    if piles[0] ^ piles[1] ^ piles[2] ^ piles[3] == 0 {
        println!("Le deuxième joueur devrait gagner.");
    } else {
        println!("Le premier joueur devrait gagner.");
    }

    let hashmap = qlearning::entrainé(&piles, 1000000);

    if qlearning::victoire_parfaite(piles, hashmap) {
        println!("Réussite");
    } else {
        println!("Échec");
    }
}

//  001 = 1 = 2⁰
//  010 = 2 = 2¹
//  100 = 4 = 2²
// 1000 = 8 = 2³
// 2⁰ + 2¹ = 3
// 1111
// 2³*1 2²*1 2¹*1 2⁰*1
// 2⁴-1
// u8 = 8 emplacements
