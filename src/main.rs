use app::MyApp;
mod app;

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
