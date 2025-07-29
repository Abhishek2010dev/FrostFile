use eframe::egui::{self, Align, Color32, FontId, Layout, RichText, Stroke, Vec2};

#[derive(Default, PartialEq)]
enum PageState {
    #[default]
    Home,
    ScanFile,
    ScanDir,
}

#[derive(Default)]
pub struct Zora {
    page_state: PageState,
}

impl eframe::App for Zora {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;

                if self.page_state != PageState::Home
                    && ui.button(RichText::new("⬅ Back").strong()).clicked()
                {
                    self.page_state = PageState::Home;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.page_state {
            PageState::Home => self.home_ui(ui),
            PageState::ScanFile => self.scan_file_ui(ui),
            PageState::ScanDir => self.scan_dir_ui(ui),
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.hyperlink_to(
                    RichText::new("© 2025 Zora Antivirus · Abhishek2010dev")
                        .small()
                        .color(Color32::GRAY),
                    "https://github.com/Abhishek2010dev",
                );
            });
        });
    }
}

impl Zora {
    fn home_ui(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(60.0);

            ui.label(
                RichText::new("Zora Antivirus")
                    .font(FontId::proportional(32.0))
                    .color(Color32::WHITE),
            );
            ui.add_space(10.0);

            ui.label(
                RichText::new("Choose a scan mode below")
                    .size(18.0)
                    .color(Color32::GRAY),
            );
            ui.add_space(40.0);

            if ui
                .add(
                    egui::Button::new(
                        RichText::new("📄 Scan File")
                            .heading()
                            .color(Color32::WHITE),
                    )
                    .fill(Color32::from_rgb(80, 130, 250))
                    .min_size(Vec2::new(220.0, 50.0))
                    .corner_radius(12.0)
                    .stroke(Stroke::new(1.5, Color32::BLACK)),
                )
                .clicked()
            {
                self.page_state = PageState::ScanFile;
            }

            ui.add_space(20.0);

            if ui
                .add(
                    egui::Button::new(
                        RichText::new("📁 Scan Directory")
                            .heading()
                            .color(Color32::WHITE),
                    )
                    .fill(Color32::from_rgb(100, 200, 120))
                    .min_size(Vec2::new(220.0, 50.0))
                    .corner_radius(12.0)
                    .stroke(Stroke::new(1.5, Color32::BLACK)),
                )
                .clicked()
            {
                self.page_state = PageState::ScanDir;
            }
        });
    }

    fn scan_file_ui(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label(RichText::new("📄 File Scan Screen").heading().strong());
        });
    }

    fn scan_dir_ui(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label(RichText::new("📁 Directory Scan Screen").heading().strong());
        });
    }
}

