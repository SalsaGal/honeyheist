use eframe::{App, NativeOptions};

mod bear;

fn main() {
    let options = NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Honey Heist",
        options,
        Box::new(|cc| Box::new(HoneyApp::default())),
    )
    .unwrap();
}

#[derive(Default)]
struct HoneyApp {}

impl App for HoneyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Honey Heist");
        });
    }
}
