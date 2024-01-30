use eframe::{egui, glow::CLIENT_STORAGE_BIT};
use egui::{Id, Sense, Ui, Vec2};
use world::{Display, World};
mod world;

struct MyApp {
    world: World,
    previous_size: Option<Vec2>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            world: World::default(),
            previous_size: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::SidePanel::left("my_left_panel").show(ctx, |_ui| {
            // ui.add(egui::Slider::new(&mut self.data, 0..=255).text("Red"));
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
        
            // Check if the size has changed
            if self.previous_size != Some(ui.available_size()) {
                self.world = self.world.resize(ui.available_size());
                self.previous_size = Some(ui.available_size());
            }

            self.world.show(ui);
            if self.world.update() {
                ctx.request_repaint();
            }
    
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
