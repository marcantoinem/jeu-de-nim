# Implémentation du Qlearning avec le jeu de Nim en rust
## Installation d'un environnement rust sur Windows
1. Aller sur et télécharger rustup-init.exe https://www.rust-lang.org/tools/install
2. Exécuter rustup-init et choisir customize-installation pour changer le default host triple à x86_64-pc-windows-gnu (les autres paramètres peuvent être laissé à leur valeur par défaut)
3. Attendre que l'installation se complète
4. Redémarrer l'ordinateur pour rafraichir le shell

## Mise en route
Pour obtenir le code utiliser
```
git clone https://novalemark.nohost.me/gitea/lemark/jeu_de_nim && cd jeu_de_nim
```
Pour compiler et exécuter, il faut être à l'intérieur du dossier et exécuter
```
cargo run --release
```

## Structure du code
Le code est réparti en trois fichiers importantes: main.rs, qlearning.rs et qlearning/piles_et_action.rs
### main.rs
C'est ici que se trouve la fonction main où le code est exécuté.
### qlearning.rs
Contient l'implémentation du Qlearning.
### qlearning/piles_et_actions.rs
Est composé de plusieurs petits algorithmes relié aux structures piles et actions notamment l'algorithme xor-zéro et des algorithmes de choix d'actions.