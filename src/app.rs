use eframe::{egui, epi};
use num_cpus;
use num_format::{Locale, ToFormattedString};
use std::time::Instant;
pub mod qlearning;
use crate::app::qlearning::piles_et_action::{Paramètres, Piles};

pub struct TemplateApp {
    label: String,
    k: f32,
    nb_partie: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "".to_owned(),
            k: 0.001,
            nb_partie: 1_000.0,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Qlearning appliqué au jeu de Nim"
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            label: _,
            k,
            nb_partie,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Configuration");

            ui.add(egui::Slider::new(k, 0.000001..=1.0).text("k").logarithmic(true));

            ui.add(egui::Slider::new(nb_partie, 1.0..=1_000_000.0).text("Nombre de partie").integer().logarithmic(true));

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Milieu de
            
            ui.heading("Résolution du jeu de Nim avec le Qlearning.");
            ui.hyperlink("https://novalemark.nohost.me/gitea/lemark/jeu_de_nim");
            if ui.button("Entrainer les modèles").clicked() {
                let piles = Piles([4, 3, 2, 1, 0, 0, 0, 0]);

                let nb_partie = self.nb_partie as usize;
                let k = 0.01;
                let nb_coeur = num_cpus::get();
                let nb_modèle = 125;

                let paramètres = Paramètres {
                    alpha: 0.9,
                    gamma: 1.0,
                    k,
                    récompense: 1.0,
                };

                self.label = format!(
                    "Vous avez choisi des piles de {}avec un k de {} et {} modèles.\n{}",
                    piles,
                    k,
                    nb_modèle * nb_coeur,
                    self.label
                );

                if piles.xor() == 0 {
                    self.label = format!(
                        "Attention, le deuxième joueur devrait gagner!\n{}",
                       self.label
                    );
                }
                let avant = Instant::now();

                // let nb_coup = piles.nb_coup();
                let mut min = 100.0;
                let mut max = 0.0;

                for _ in 0..10 {
                    let pourcent = qlearning::teste_fiabilité(
                        piles,
                        nb_partie,
                        nb_modèle,
                        nb_coeur,
                        paramètres,
                    ) * 100.0;
                    if pourcent > max {
                        max = pourcent;
                    } else if pourcent < min {
                        min = pourcent;
                    }
                }

                let chrono = avant
                    .elapsed()
                    .as_millis()
                    .to_formatted_string(&Locale::fr_CA);

                let milieu = (max + min) / 2.0;
                let incertitude = (max - min) / 2.0;
                let nb_partie = nb_partie.to_formatted_string(&Locale::fr_CA);

                self.label = format!(
                    "{:.2}±{:.2}% avec {} parties en {} ms\n{}",
                    milieu, incertitude, nb_partie, chrono, self.label
                );
            }
            ui.label(self.label.to_owned());
            egui::warn_if_debug_build(ui);
        });
    }
}
