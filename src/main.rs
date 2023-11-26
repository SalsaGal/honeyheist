use std::fs::{read_to_string, File};
use std::io::Write;

use bear::Bear;
use eframe::{App, NativeOptions};
use rand::rngs::ThreadRng;
use rfd::FileDialog;

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
                ui.heading("Honey Heist");
                if ui.button("New Bear").clicked() {
                    self.bear = Some(Bear::new(&mut self.rng, "Foo".to_owned()));
                }
                if ui.button("Load Bear").clicked() {
                    self.bear = FileDialog::new()
                        .add_filter("toml", &["toml"])
                        .pick_file()
                        .and_then(|path| read_to_string(path).ok())
                        .and_then(|contents| toml::from_str(&contents).ok());
                }
                if let Some(bear) = &self.bear {
                    if ui.button("Save Bear").clicked() {
                        if let Some(mut file) = FileDialog::new()
                            .add_filter("toml", &["toml"])
                            .save_file()
                            .map(|path| path.with_extension("toml"))
                            .and_then(|path| File::create(path).ok())
                        {
                            write!(file, "{}", toml::to_string(bear).unwrap()).unwrap();
                        }
                    }
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
