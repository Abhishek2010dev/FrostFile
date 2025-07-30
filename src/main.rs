mod scan;
mod ui;

use eframe::egui;

use crate::ui::Zora;

fn main() -> eframe::Result {
    egui_logger::builder()
        .init()
        .expect("Error initializing logger");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native("Zora", options, Box::new(|_| Ok(Box::<Zora>::default())))
}
