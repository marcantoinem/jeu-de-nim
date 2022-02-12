use modular_bitfield::prelude::{bitfield, B15};

// Défini une position comme un entier non-signé de 32 bit où les 8 premiers bits sont alloué
// à la taille de la pile1, les 8 bits suivant à la taille de la pile 2, le bit suivant au tour
// et les 15 derniers bits sont inutilisés.
#[bitfield]
pub struct Position {
    pile1: u8,
    pile2: u8,
    cblanc: bool,
    #[skip]
    inutile: B15,
}

//
//
//
fn random_game() {
    let position_initiale = Position::new().with_pile1(4).with_pile2(5).with_cblanc(true);
    let mut position_actuelle = position_initiale;
}
