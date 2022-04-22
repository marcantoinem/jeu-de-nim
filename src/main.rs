#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg(not(target_arch = "wasm32"))]
use jeu_de_nim::TemplateApp;

pub mod app;
fn main() {
    let app = TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
