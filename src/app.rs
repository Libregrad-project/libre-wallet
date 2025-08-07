use eframe::{egui, App, Frame};
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

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

    mining_logs: Arc<Mutex<Vec<String>>>,
    miner_process: Option<Arc<Mutex<Child>>>,
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

            mining_logs: Arc::new(Mutex::new(vec![])),
            miner_process: None,
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
        let show_window = |title: &str, open_flag: &mut bool, default_size: egui::Vec2, content: &mut dyn FnMut(&mut egui::Ui)| {
            let mut open = *open_flag;
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
            #[derive(PartialEq, Eq)]
            enum MiningAction {
                Start,
                Stop,
            }
            let mut mining_action: Option<MiningAction> = None;

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
                                mining_action = Some(MiningAction::Stop);
                            }
                        } else {
                            if ui.button("Start Mining").clicked() {
                                mining_action = Some(MiningAction::Start);
                            }
                        }
                    });

                    ui.separator();

                    ui.label("Logs:");
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .max_height(150.0)
                        .show(ui, |ui| {
                            // Clone the Arc to avoid holding lock during UI update
                            let logs = self.mining_logs.lock().unwrap();
                            for log in logs.iter() {
                                ui.label(log);
                            }
                        });
                },
            );

            // Perform mining start/stop after UI closure to avoid multiple mutable borrows
            if let Some(action) = mining_action {
                match action {
                    MiningAction::Start => self.start_mining(),
                    MiningAction::Stop => self.stop_mining(),
                }
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
    fn start_mining(&mut self) {
        if self.miner_process.is_some() {
            self.log_mining_event("Miner is already running.");
            return;
        }

        let pool_addr = format!("{}:{}", self.mining_pool_host, self.mining_pool_port);
        let threads_str = self.mining_threads.to_string(); // keep alive!

        let args = vec![
            "-o", &pool_addr,
            "-u", &self.mining_worker,
            "-p", &self.mining_password,
            "-t", &threads_str,
            "-a", "cn/0",
            "--donate-level", "0",
        ];

        match Command::new("./xmrig")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                let stdout = child.stdout.take();
                let stderr = child.stderr.take();

                let logs_stdout = Arc::clone(&self.mining_logs);
                if let Some(stdout) = stdout {
                    thread::spawn(move || {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                let mut logs = logs_stdout.lock().unwrap();
                                logs.push(format!("xmrig: {}", line));
                                let len = logs.len();
                                if len > 100 {
                                    logs.drain(0..(len - 100));
                                }
                            }
                        }
                    });
                }

                let logs_stderr = Arc::clone(&self.mining_logs);
                if let Some(stderr) = stderr {
                    thread::spawn(move || {
                        let reader = BufReader::new(stderr);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                let mut logs = logs_stderr.lock().unwrap();
                                logs.push(format!("xmrig err: {}", line));
                                let len = logs.len();
                                if len > 100 {
                                    logs.drain(0..(len - 100));
                                }
                            }
                        }
                    });
                }

                self.miner_process = Some(Arc::new(Mutex::new(child)));
                self.mining_active = true;
                self.mining_hashrate = (self.mining_threads as f64) * 1000.0;
                self.log_mining_event("Mining started.");
            }
            Err(e) => {
                self.log_mining_event(&format!("Failed to start miner: {}", e));
                self.mining_active = false;
            }
        }
    }


    fn stop_mining(&mut self) {
        if let Some(miner) = &self.miner_process {
            if let Ok(mut child) = miner.lock() {
                let _ = child.kill();
                let _ = child.wait();
            }
            self.miner_process = None;
            self.mining_active = false;
            self.mining_hashrate = 0.0;
            self.log_mining_event("Mining stopped.");
        } else {
            self.log_mining_event("No miner process running.");
        }
    }

    fn log_mining_event(&mut self, message: &str) {
        use chrono::Local;
        let timestamp = Local::now().format("%H:%M:%S").to_string();
        let mut logs = self.mining_logs.lock().unwrap();
        logs.push(format!("[{}] {}", timestamp, message));
        if logs.len() > 100 {
           let len = logs.len();
           logs.drain(0..(len - 100));
        }
    }
}
