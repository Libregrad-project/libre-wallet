mod app;
mod rpc;

use app::WalletApp;

use eframe;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Libre Wallet",
        options,
        Box::new(|_cc| Box::new(WalletApp::default())),
    )
}
