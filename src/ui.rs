use std::{
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use eframe::egui::{self, Align, Color32, FontId, Layout, ProgressBar, RichText, Stroke, Vec2};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use walkdir::WalkDir;

use crate::scan::scan_file;

#[derive(Default, PartialEq)]
enum PageState {
    #[default]
    Home,
    ScanFile,
    ScanDir,
}

#[derive(Default)]
pub struct Zora {
    progress: f32,
    process: Arc<Mutex<f32>>,
    page_state: PageState,
    scanning: bool,
    selected_dir: Option<PathBuf>,
    finished: Arc<Mutex<bool>>,
    infected_found: Arc<Mutex<bool>>,
    infected_paths: Arc<Mutex<Vec<PathBuf>>>,
    show_alert: bool,
}

impl eframe::App for Zora {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if self.page_state == PageState::ScanDir && self.selected_dir.is_none() && !self.scanning {
            if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                self.selected_dir = Some(dir.clone());
                self.reset_scan_state();
                self.scanning = true;
                self.start_scan(ctx.clone(), dir);
            } else {
                self.reset_scan_state();
                self.page_state = PageState::Home;
            }
        }

        if self.show_alert {
            egui::Window::new("⚠️ Infected Files Detected")
                .collapsible(false)
                .resizable(false)
                .fixed_size(Vec2::new(400.0, 200.0))
                .show(ctx, |ui| {
                    ui.label("One or more infected files were found during the scan.");
                    ui.add_space(10.0);
                    ui.label("Please review the logs for details.");
                    ui.add_space(20.0);

                    if ui.button("OK").clicked() {
                        self.show_alert = false;
                    }
                });
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;

                if self.page_state != PageState::Home
                    && ui.button(RichText::new("⬅ Back").strong()).clicked()
                {
                    self.reset_scan_state();
                    self.page_state = PageState::Home;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.page_state {
            PageState::Home => self.home_ui(ui),
            PageState::ScanFile => self.scan_file_ui(ui),
            PageState::ScanDir => self.scan_dir_ui(ui),
        });

        // Bottom footer panel
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
    fn reset_scan_state(&mut self) {
        self.selected_dir = None;
        self.scanning = false;
        *self.process.lock().unwrap() = 0.0;
        *self.finished.lock().unwrap() = false;
        *self.infected_found.lock().unwrap() = false;
        self.infected_paths.lock().unwrap().clear();
        self.show_alert = false;
    }

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
                    .corner_radius(12.0),
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
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(40.0);

            ui.label(
                RichText::new("📄 File Scan")
                    .heading()
                    .color(Color32::WHITE),
            );
            ui.add_space(20.0);

            if ui
                .add(
                    egui::Button::new(RichText::new("Select a File").color(Color32::WHITE))
                        .fill(Color32::from_rgb(120, 150, 250))
                        .min_size(Vec2::new(200.0, 40.0))
                        .corner_radius(10.0),
                )
                .clicked()
            {
                if let Some(file) = rfd::FileDialog::new().pick_file() {
                    match crate::scan::scan_file(&file) {
                        crate::scan::ScanResult::Clean(path) => {
                            log::info!("Clean: {}", path.display());
                        }
                        crate::scan::ScanResult::Infected(path) => {
                            log::warn!("Infected: {}", path.display());
                        }
                        crate::scan::ScanResult::Error(path, err) => {
                            log::error!("Error scanning {}: {}", path.display(), err);
                        }
                    }
                }
            }

            ui.add_space(30.0);

            egui_logger::LoggerUi::default()
                .enable_regex(true)
                .enable_search(true)
                .max_log_length(2000)
                .enable_category("scan", true)
                .enable_category("egui_glow::painter", true)
                .show(ui);
        });
    }

    fn scan_dir_ui(&mut self, ui: &mut egui::Ui) {
        let progress = *self.process.lock().unwrap();
        self.progress = progress;
        ui.add(ProgressBar::new(progress).text(format!("{:.0}%", progress * 100.0)));
        egui_logger::LoggerUi::default()
            .enable_regex(true)
            .enable_search(true)
            .max_log_length(2000)
            .show(ui);
    }

    fn start_scan(&mut self, ctx: egui::Context, dir: PathBuf) {
        self.scanning = true;
        let progres = Arc::clone(&self.process);
        let finished = Arc::clone(&self.finished);
        let infected_found = Arc::clone(&self.infected_found);
        let infected_paths = Arc::clone(&self.infected_paths);

        std::thread::spawn(move || {
            let entries: Vec<_> = WalkDir::new(&dir)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
                .map(|e| e.into_path())
                .collect();

            let total = entries.len().max(1);
            let done = AtomicUsize::new(0);

            entries.into_par_iter().for_each(|path| {
                match scan_file(&path) {
                    crate::scan::ScanResult::Clean(path_buf) => {
                        log::info!("Clean: {}", path_buf.display())
                    }
                    crate::scan::ScanResult::Infected(path_buf) => {
                        log::warn!("Infected: {}", path_buf.display());
                        *infected_found.lock().unwrap() = true;
                        infected_paths.lock().unwrap().push(path_buf);
                    }
                    crate::scan::ScanResult::Error(path_buf, err) => {
                        log::error!("Error: {}: {}", path_buf.display(), err)
                    }
                }

                done.fetch_add(1, Ordering::Relaxed);
                let done_value = done.load(Ordering::Relaxed);
                *progres.lock().unwrap() = (done_value as f32) / (total as f32);
                ctx.request_repaint();
            });

            log::info!("Directory scan complete");
            *finished.lock().unwrap() = true;
            ctx.request_repaint();
        });
    }
}
