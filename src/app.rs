use eframe::{egui, App, Frame};

pub struct WalletApp {
    show_about: bool,
    show_transactions: bool,
    show_dashboard: bool,
    show_wallet: bool,
    show_mining: bool,
    show_settings: bool,

    // Mining state
    mining_active: bool,
    mining_threads: usize,
    mining_hashrate: f64,
    mining_pool_host: String,
    mining_pool_port: u16,
    mining_worker: String,
    mining_password: String,
    mining_logs: Vec<String>,
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

            mining_active: false,
            mining_threads: 4,
            mining_hashrate: 0.0,
            mining_pool_host: "pool.example.com".to_owned(),
            mining_pool_port: 3333,
            mining_worker: "worker1".to_owned(),
            mining_password: "password".to_owned(),
            mining_logs: vec![],
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

        // SIDE PANEL
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

        // CENTRAL PANEL
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Wallet Overview");
                ui.separator();
                ui.label("Use the menu on the left to open windows.");
            });
        });

        // BOTTOM PANEL
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label("Libre Wallet v0.1 • Built with Rust + eframe/egui");
            });
        });

        // Helper closure for windows with close button & consistent styling
        let mut show_window = |title: &str, open_flag: &mut bool, default_size: egui::Vec2, content: &mut dyn FnMut(&mut egui::Ui)| {            let mut open = *open_flag;
            let mut close_clicked = false;

            egui::Window::new(title)
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .default_size(default_size)
                .min_size(egui::vec2(300.0, 150.0))
                .frame(egui::Frame::window(&ctx.style()).fill(ctx.style().visuals.window_fill()))
                .show(ctx, |ui| {
                    content(ui);
                    ui.add_space(10.0);
                    ui.horizontal_centered(|ui| {
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

        // About window
        if self.show_about {
            show_window(
                "About Libre Wallet",
                &mut self.show_about,
                egui::vec2(400.0, 220.0),
                &mut |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.heading("Libre Wallet");
                        ui.label("v0.1");
                    });
                    ui.separator();
                    ui.label("A privacy-focused cryptocurrency wallet.");
                    ui.label("Built in Rust using eframe/egui.");
                },
            );
        }

        // Transactions window
        if self.show_transactions {
            show_window(
                "Transactions",
                &mut self.show_transactions,
                egui::vec2(600.0, 350.0),
               &mut |ui| {
                    ui.heading("Transaction History");
                    ui.separator();
                    ui.label("No transactions to display.");
                },
            );
        }

        // Dashboard window
        if self.show_dashboard {
            show_window(
                "Dashboard",
                &mut self.show_dashboard,
                egui::vec2(600.0, 350.0),
               &mut |ui| {
                    ui.heading("Dashboard");
                    ui.separator();
                    ui.label("Overview of your wallet's status.");
                },
            );
        }

        // Wallet window
        if self.show_wallet {
            show_window(
                "Wallet",
                &mut self.show_wallet,
                egui::vec2(600.0, 350.0),
                &mut |ui| {
                    ui.heading("Wallet");
                    ui.separator();
                    ui.label("Manage your wallet here.");
                },
            );
        }

        // Mining window with full UI
        if self.show_mining {
            // Create a mutable variable to hold the mining event message
            let mut mining_event: Option<String> = None;

            show_window(
                "Mining",
                &mut self.show_mining,
                egui::vec2(700.0, 500.0),
                &mut |ui| {
                    ui.heading("Mining Control Panel");
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        ui.colored_label(
                            if self.mining_active {
                                egui::Color32::GREEN
                            } else {
                                egui::Color32::RED
                            },
                            if self.mining_active { "Running" } else { "Stopped" },
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("Hashrate:");
                        ui.label(format!("{:.2} H/s", self.mining_hashrate));
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Mining Threads:");
                        ui.add(egui::Slider::new(&mut self.mining_threads, 1..=16).text("threads"));
                    });

                    ui.separator();

                    ui.heading("Pool Settings");
                    ui.horizontal(|ui| {
                        ui.label("Host:");
                        ui.text_edit_singleline(&mut self.mining_pool_host);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Port:");
                        ui.add(egui::DragValue::new(&mut self.mining_pool_port).clamp_range(0..=65535));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Worker:");
                        ui.text_edit_singleline(&mut self.mining_worker);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Password:");
                        ui.text_edit_singleline(&mut self.mining_password);
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        if self.mining_active {
                            if ui.button("Stop Mining").clicked() {
                                self.mining_active = false;
                                mining_event = Some("Mining stopped by user".to_owned());
                                self.mining_hashrate = 0.0;
                            }
                        } else {
                            if ui.button("Start Mining").clicked() {
                                self.mining_active = true;
                                mining_event = Some(format!(
                                    "Mining started on {}:{} with worker '{}'",
                                    self.mining_pool_host, self.mining_pool_port, self.mining_worker
                                ));
                                self.mining_hashrate = (self.mining_threads as f64) * 1000.0;
                            }
                        }
                    });

                    ui.separator();

                    ui.label("Logs:");
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .max_height(150.0)
                        .show(ui, |ui| {
                            for log in &self.mining_logs {
                                ui.label(log);
                            }
                        });
                },
            );

            // Now outside the closure, log the event to avoid double mutable borrow
            if let Some(event) = mining_event {
                self.log_mining_event(&event);
            }

            // Simulate hashrate fluctuation when mining
            if self.mining_active {
                self.mining_hashrate *= 0.95 + (rand::random::<f64>() * 0.1);
                if self.mining_hashrate < 10.0 {
                    self.mining_hashrate = 10.0;
                }
                ctx.request_repaint_after(std::time::Duration::from_millis(500));
            }
        }


        // Settings window
        if self.show_settings {
            show_window(
                "Settings",
                &mut self.show_settings,
                egui::vec2(400.0, 300.0),
               &mut |ui| {
                    ui.heading("Settings");
                    ui.separator();
                    ui.label("Configure your wallet preferences.");
                },
            );
        }
    }
}

impl WalletApp {
    fn log_mining_event(&mut self, message: &str) {
        use chrono::Local;
        let timestamp = Local::now().format("%H:%M:%S").to_string();
        self.mining_logs.push(format!("[{}] {}", timestamp, message));

        if self.mining_logs.len() > 100 {
            self.mining_logs.drain(0..(self.mining_logs.len() - 100));
        }
    }
}
