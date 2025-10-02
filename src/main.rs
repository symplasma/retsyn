use eframe::egui;
use retsyn::retsyn_app::RetsynApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Search App"),
        ..Default::default()
    };

    eframe::run_native(
        "Search App",
        options,
        Box::new(|_cc| Ok(Box::new(RetsynApp::default()))),
    )
}
