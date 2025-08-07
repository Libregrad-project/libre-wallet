use eframe::{egui, App, Frame};
use serde::{Deserialize, Serialize};

struct WalletApp;

impl App for WalletApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Libre Wallet");
            if ui.button("Sync Wallet").clicked() {
                println!("Syncing...");
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Libre Wallet",
        options,
        Box::new(|_cc| Box::new(WalletApp)),
    )
}
