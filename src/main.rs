#[macro_use]
extern crate serde;

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Crosshair",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .unwrap();
}

#[derive(Default, Serialize, Deserialize)]
struct App {
    show_crosshair: bool,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);

        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Settings");

            ui.add_space(8.0);

            if ui.button("Show crosshair").clicked() {
                self.show_crosshair = !self.show_crosshair;
            }

            if self.show_crosshair {
                ui.add(Crosshair::default());
            }
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}

#[derive(Default)]
struct Crosshair;

impl egui::Widget for Crosshair {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.ctx().show_viewport_immediate(
            egui::ViewportId::from_hash_of("crosshair"),
            egui::ViewportBuilder::default().with_transparent(true),
            |ctx, _| {
                egui::CentralPanel::default()
                    .frame(egui::Frame {
                        fill: egui::Color32::TRANSPARENT,
                        ..Default::default()
                    })
                    .show(ctx, |ui| ui.label("crosshair"))
                    .response
            },
        )
    }
}
