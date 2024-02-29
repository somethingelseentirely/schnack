use eframe::{egui, emath::align};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "schnack",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::empty();
            fonts.font_data.insert(
                "Sono".to_owned(),
                egui::FontData::from_static(include_bytes!(
                    "../fonts/Sono/static/Sono_Proportional-Regular.ttf"
                )),
            );

            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .push("Sono".to_owned());

            fonts
                .families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .push("Sono".to_owned());

            cc.egui_ctx.set_fonts(fonts);

            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    msgs: Vec<String>,
    draft: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            msgs: vec![],
            draft: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.vertical(|ui| {
                    for msg in &self.msgs {
                        ui.text_edit_multiline(&mut msg.as_str());
                    }
                });
                ui.horizontal(|ui| {
                    ui.text_edit_multiline(&mut self.draft);
                    if ui.button("↑").clicked() {
                        let draft = std::mem::replace(&mut self.draft, "".to_owned());
                        self.msgs.push(draft);
                    }
                });
            })
        });
    }
}
