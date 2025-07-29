use eframe::egui;

#[derive(Default)]
pub struct FrostFile;

impl eframe::App for FrostFile {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, move |_| ());
    }
}
