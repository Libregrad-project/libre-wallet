use eframe::{egui, App, Frame};

pub struct WalletApp {
    show_about: bool,
}

impl Default for WalletApp {
    fn default() -> Self {
        Self { show_about: false }
    }
}

impl App for WalletApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // TOP PANEL
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("Libre Wallet");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                    }

                    if ui.button("Settings").clicked() {
                        // Placeholder for settings
                    }
                });
            });
        });

        // SIDE PANEL
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .default_width(180.0)
            .show(ctx, |ui| {
                ui.heading("Menu");
                ui.separator();
                ui.selectable_label(false, "Dashboard");
                ui.selectable_label(false, "Wallet");
                ui.selectable_label(false, "Transactions");
                ui.selectable_label(false, "Mining");
                ui.selectable_label(false, "Settings");
            });

        // CENTRAL PANEL
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wallet Overview");
            ui.separator();

            egui::Grid::new("wallet_info_grid")
                .striped(true)
                .spacing([20.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Address:");
                    ui.monospace("44Affq5kSiGBoZ..."); ui.end_row();
                    ui.label("Balance:");
                    ui.label("123.456 OHN"); ui.end_row();
                    ui.label("Unlocked:");
                    ui.label("120.000 OHN"); ui.end_row();
                });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.label("Actions");
                if ui.button("🔄 Sync Wallet").clicked() {
                    println!("Syncing...");
                }
                if ui.button("💸 Send").clicked() {
                    println!("Send clicked");
                }
                if ui.button("📥 Receive").clicked() {
                    println!("Receive clicked");
                }
            });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.label("Recent Transactions");
                ui.label("(none)");
            });
        });

        // BOTTOM PANEL
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label("Libre Wallet v0.1 • Built with Rust + eframe/egui");
            });
        });

        // ABOUT POPUP WINDOW
        if self.show_about {
            egui::Window::new("About Libre Wallet")
                .open(&mut self.show_about)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Libre Wallet v0.1");
                    ui.label("A privacy-focused cryptocurrency wallet.");
                    ui.label("Built in Rust using the eframe + egui GUI framework.");
                    ui.separator();
                    ui.label("Created by Vyzic");
                });
        }
    }
}
