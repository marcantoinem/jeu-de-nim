use eframe::{egui, epi};
use num_cpus;
use num_format::{Locale, ToFormattedString};
use std::time::Instant;
pub mod qlearning;
use crate::app::qlearning::piles_et_action::{Paramètres, Piles, NB_DE_PILE};

pub struct TemplateApp {
    sortie: String,
    sortie_piles: String,
    k: f64,
    nb_partie: usize,
    nb_modèle: usize,
    nb_coeur: usize,
    incertitude: bool,
    _piles: Piles,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            sortie: "".to_owned(),
            sortie_piles: "".to_owned(),
            k: 1.0,
            nb_partie: 1_000,
            nb_modèle: 100,
            nb_coeur: 4,
            incertitude: false,
            _piles: Piles([4, 3, 2, 1, 0, 0, 0, 0]),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Qlearning appliqué au jeu de Nim"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            sortie: _,
            sortie_piles: _,
            k,
            nb_partie,
            nb_modèle,
            nb_coeur,
            incertitude: _,
            _piles,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Fichier", |ui| {
                    if ui.button("Quitter").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Configuration");

            ui.add(
                egui::Slider::new(k, 0.000001..=1.0)
                    .text("Vitesse de disparition de la surestimation")
                    .logarithmic(true),
            );

            ui.add(
                egui::Slider::new(nb_partie, 1..=1_000_000)
                    .text("Nombre de partie(s)")
                    .logarithmic(true),
            );
            ui.add(
                egui::Slider::new(nb_modèle, 1..=1_000_000)
                    .text("Nombre de modèle(s) par coeur")
                    .logarithmic(true),
            );
            ui.add(egui::Slider::new(nb_coeur, 1..=num_cpus::get()).text("Nombre de coeur(s)"));
            ui.checkbox(&mut self.incertitude, "Calcul des incertitudes");

            ui.heading("Piles");
            ui.horizontal(|ui| {
                for index in 0..NB_DE_PILE {
                    ui.add(
                        egui::DragValue::new(&mut self._piles[index])
                            .speed(0.1)
                            .clamp_range(0..=255),
                    );
                }
            });
            if ui.button("Informations sur les piles").clicked() {
                self.sortie_piles = format!(
                    "Pour résoudre ces piles, il faut effectuer {} coups parfaits.\n",
                    self._piles.nb_coup()
                );
                if self._piles.xor() == 0 {
                    self.sortie_piles = format!(
                        "{}Attention, le deuxième joueur devrait gagner!",
                        self.sortie_piles
                    );
                }
            }
            ui.label(self.sortie_piles.to_owned());
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Résolution du jeu de Nim avec le Qlearning.");
            ui.hyperlink("https://novalemark.nohost.me/gitea/lemark/jeu_de_nim");
            if ui.button("Entrainer les modèles").clicked() {
                let piles = self._piles;
                let nb_partie = self.nb_partie;
                let k = self.k;
                let nb_coeur = self.nb_coeur;
                let nb_modèle = self.nb_modèle;

                let paramètres = Paramètres {
                    alpha: 0.9,
                    gamma: 1.0,
                    k,
                    récompense: 1.0,
                };

                let avant = Instant::now();
                if self.incertitude {
                    let mut min = 100.0;
                    let mut max = 0.0;
                    let mut somme = 0.0;

                    for _ in 0..10 {
                        let pourcent = qlearning::teste_fiabilité(
                            piles,
                            nb_partie,
                            nb_modèle,
                            nb_coeur,
                            paramètres,
                        ) * 100.0;
                        somme += pourcent;
                        if pourcent > max {
                            max = pourcent;
                        }
                        if pourcent < min {
                            min = pourcent;
                        }
                    }

                    let chrono = avant
                        .elapsed()
                        .as_millis()
                        .to_formatted_string(&Locale::fr_CA);

                    let moyenne = somme / 10.0;
                    let incertitude = (max - min) / 2.0;
                    let nb_partie = nb_partie.to_formatted_string(&Locale::fr_CA);

                    self.sortie = format!(
                        "{:.2}±{:.2}% avec {} parties en {} ms\n{}",
                        moyenne, incertitude, nb_partie, chrono, self.sortie
                    );
                } else {
                    let pourcent = qlearning::teste_fiabilité(
                        piles,
                        nb_partie,
                        nb_modèle,
                        nb_coeur,
                        paramètres,
                    ) * 100.0;

                    let chrono = avant
                        .elapsed()
                        .as_millis()
                        .to_formatted_string(&Locale::fr_CA);

                    self.sortie = format!(
                        "{:.2}% avec {} parties en {} ms\n{}",
                        pourcent, nb_partie, chrono, self.sortie
                    );
                }

                self.sortie = format!(
                    "\nVous avez choisi des piles de {}avec un k de {} et {} modèles.\n{}",
                    piles,
                    k,
                    nb_modèle * nb_coeur,
                    self.sortie
                );
            }
            ui.label(self.sortie.to_owned());
            egui::warn_if_debug_build(ui);
        });
    }
}
