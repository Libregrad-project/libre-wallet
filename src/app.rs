use eframe::{egui, App, Frame};

pub struct WalletApp {
    show_about: bool,
    about_window_size: egui::Vec2,
}

impl Default for WalletApp {
    fn default() -> Self {
        Self {
            show_about: false,
            about_window_size: egui::Vec2::splat(0.4),
        }
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
            let mut open = self.show_about;
            let mut close_clicked = false;

            egui::Window::new("About Libre Wallet")
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .default_size(egui::vec2(400.0, 200.0))
                .min_size(egui::vec2(200.0, 100.0))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Libre Wallet");
                        ui.label("v0.1");
                    });
                    ui.separator();
                    ui.label("A privacy-focused cryptocurrency wallet.");
                    ui.add_space(8.0);
                    ui.label("Built in Rust using eframe/egui.");

                    if ui.button("Close").clicked() {
                        close_clicked = true;
                    }
                });

            if close_clicked {
                open = false;
            }
            self.show_about = open;
        }



    }
}
