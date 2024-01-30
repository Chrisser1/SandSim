use eframe::{egui, glow::CLIENT_STORAGE_BIT};
use egui::{emath::RectTransform, Id, Pos2, Sense, Ui, Vec2};
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
        let mut side_panel_width = 0.0;
        let mut top_bar_height = 0.0;
        
        let top_bottom_panel = egui::TopBottomPanel::top("top_panel").resizable(false);        
        top_bottom_panel.show(ctx, |ui| {
            top_bar_height = ui.available_height() + 20.0;
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

        let side_panel = egui::SidePanel::left("my_left_panel").resizable(false);
        side_panel.show(ctx, |ui| {
            side_panel_width = ui.available_width() + 16.0;
            let mut count = 0;
            for x in 0..self.world.data.len() {
                for y in 0..self.world.data[x].len() {
                    
                    if self.world.data[x][y].is_some() {
                        count += 1;
                    }
                }
            }
            let count_label = format!("Sand count: {}", count);
            ui.label(count_label);
            // ui.add(egui::Slider::new(&mut self.data, 0..=255).text("Red"));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let central_panel_position = egui::pos2(side_panel_width, top_bar_height);
            let available_size = ui.available_size();

            ui.input(|input| {
                self.world.clicked(available_size, input.pointer.interact_pos(), central_panel_position);
            });
            
            // Check if the size has changed
            if self.previous_size != Some(available_size) {
                self.world = self.world.resize(available_size, central_panel_position);
                self.previous_size = Some(available_size);
            }

            self.world.show(ui);
            if self.world.update() {
                ctx.request_repaint();
            }
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
        Box::new(|_cc| {

            Box::<MyApp>::default()
        }),
    )
}
