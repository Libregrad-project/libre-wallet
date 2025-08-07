use eframe::{egui, App, Frame};

pub struct WalletApp;

impl App for WalletApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Libre Wallet");
            if ui.button("Sync Wallet").clicked() {
                println!("Syncing...");
                //TODO: Call the RPC Functions.
            }
        });
    }
}
