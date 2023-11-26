use bear::Bear;
use eframe::{App, NativeOptions};
use rand::rngs::ThreadRng;

mod bear;

fn main() {
    let options = NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Honey Heist",
        options,
        Box::new(|_| Box::<HoneyApp>::default()),
    )
    .unwrap();
}

#[derive(Default)]
struct HoneyApp {
    rng: ThreadRng,
    bear: Option<Bear>,
}

impl App for HoneyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Control");
                if ui.button("New Bear").clicked() {
                    self.bear = Some(Bear::new(&mut self.rng, "Foo".to_owned()));
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(bear) = &self.bear {
                ui.heading(&bear.name);
            }
        });
    }
}
