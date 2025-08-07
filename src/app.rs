use eframe::{egui, App, Frame};

pub struct WalletApp {
    show_about: bool,
    show_transactions: bool,
    show_dashboard: bool,
    show_wallet: bool,
    show_mining: bool,
    show_settings: bool,
    about_window_size: egui::Vec2,
}

impl Default for WalletApp {
    fn default() -> Self {
        Self {
            show_about: false,
            show_transactions: false,
            show_dashboard: false,
            show_wallet: false,
            show_mining: false,
            show_settings: false,
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
                        self.show_settings = true;
                    }
                });
            });
        });

        // SIDE PANEL with clickable menu buttons
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .default_width(180.0)
            .show(ctx, |ui| {
                ui.heading("Menu");
                ui.separator();

                if ui.button("Dashboard").clicked() {
                    self.show_dashboard = true;
                }
                if ui.button("Wallet").clicked() {
                    self.show_wallet = true;
                }
                if ui.button("Transactions").clicked() {
                    self.show_transactions = true;
                }
                if ui.button("Mining").clicked() {
                    self.show_mining = true;
                }
                if ui.button("Settings").clicked() {
                    self.show_settings = true;
                }
            });

        // CENTRAL PANEL: You can keep this or show based on active window if you want
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wallet Overview");
            ui.separator();
            ui.label("Use the menu to open windows.");
        });

        // BOTTOM PANEL
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label("Libre Wallet v0.1 • Built with Rust + eframe/egui");
            });
        });

        // Helper closure to create popups with close button
        let mut show_window = |title: &str, open_flag: &mut bool, default_size: egui::Vec2, content: &dyn Fn(&mut egui::Ui)| {
            let mut open = *open_flag;
            let mut close_clicked = false;
            egui::Window::new(title)
                .open(&mut open)
                .resizable(true)
                .default_size(default_size)
                .show(ctx, |ui| {
                    content(ui);
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            close_clicked = true;
                        }
                    });
                });
            if close_clicked {
                open = false;
            }
            *open_flag = open;
        };

        // Show About window
        if self.show_about {
            show_window(
                "About Libre Wallet",
                &mut self.show_about,
                egui::vec2(400.0, 200.0),
                &|ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Libre Wallet");
                        ui.label("v0.1");
                    });
                    ui.separator();
                    ui.label("A privacy-focused cryptocurrency wallet.");
                    ui.add_space(8.0);
                    ui.label("Built in Rust using eframe/egui.");
                },
            );
        }

        // Show Transactions window
        if self.show_transactions {
            show_window(
                "Transactions",
                &mut self.show_transactions,
                egui::vec2(600.0, 300.0),
                &|ui| {
                    ui.heading("Transaction History");
                    ui.separator();
                    ui.label("No transactions to display.");
                },
            );
        }

        // Show Dashboard window
        if self.show_dashboard {
            show_window(
                "Dashboard",
                &mut self.show_dashboard,
                egui::vec2(600.0, 300.0),
                &|ui| {
                    ui.heading("Dashboard");
                    ui.separator();
                    ui.label("Overview of your wallet's status.");
                },
            );
        }

        // Show Wallet window
        if self.show_wallet {
            show_window(
                "Wallet",
                &mut self.show_wallet,
                egui::vec2(600.0, 300.0),
                &|ui| {
                    ui.heading("Wallet");
                    ui.separator();
                    ui.label("Manage your wallet here.");
                },
            );
        }

        // Show Mining window
        if self.show_mining {
            show_window(
                "Mining",
                &mut self.show_mining,
                egui::vec2(600.0, 300.0),
                &|ui| {
                    ui.heading("Mining");
                    ui.separator();
                    ui.label("Mining status and controls.");
                },
            );
        }

        // Show Settings window
        if self.show_settings {
            show_window(
                "Settings",
                &mut self.show_settings,
                egui::vec2(400.0, 300.0),
                &|ui| {
                    ui.heading("Settings");
                    ui.separator();
                    ui.label("Configure your wallet preferences.");
                },
            );
        }
    }
}
