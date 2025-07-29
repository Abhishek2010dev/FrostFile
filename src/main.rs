mod ui;

use eframe::egui;

use crate::ui::Zora;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Frost File",
        options,
        Box::new(|_| Ok(Box::<Zora>::default())),
    )
}
