use eframe::{egui, glow::CLIENT_STORAGE_BIT};
use world::{Display, World};
mod world;

struct MyApp {
    world: World,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            world: World::default()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.world.show(ui);

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        println!("saved");
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });
    }
}


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Team Lukas app",
        options,
        Box::new(|cc| {

            Box::<MyApp>::default()
        }),
    )
}
