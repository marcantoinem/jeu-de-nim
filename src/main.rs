use crate::qlearning::piles_et_action::{Paramètres, Piles};
pub mod qlearning;

fn main() {
    let piles = Piles([3, 1, 1]);
    let k = 1.0;
    let nb_partie = 5;

    let paramètres = Paramètres {
        alpha: 0.9,
        gamma: 1.0,
        k,
        récompense: 1.0,
    };

    println!("Piles initiales: {}.", piles,);

    if piles.xor() == 0 {
        println!("Attention, le deuxième joueur devrait gagner!");
    }

    qlearning::entraine(&piles, nb_partie, paramètres);
}
