struct game {
    pile1: u8,
    pile2: u8,
    ctaki: bool,
}

impl game {
    fn new() -> game {
        game {
            pile1: 5,
            pile2: 4,
            ctaki: true,
        }
    }
}

fn main() {
    let game = game::new();
    println!("Le jeu de Nim.");
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