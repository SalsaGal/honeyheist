use std::fs::{read_to_string, File};
use std::io::Write;

use creator::Creator;
use eframe::{App, NativeOptions};
use egui::Button;
use play::Play;
use rand::rngs::ThreadRng;
use rfd::FileDialog;

mod bear;
mod creator;
mod play;

fn main() {
    let options = NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Honey Heist",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);
            Box::<HoneyApp>::default()
        }),
    )
    .unwrap();
}

pub enum State {
    Creator(Creator),
    Play(Play),
}

struct HoneyApp {
    rng: ThreadRng,
    state: State,
}

impl App for HoneyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Honey Heist");
                if ui.button("New Bear").clicked() {
                    self.state = State::Creator(Creator::new(&mut self.rng));
                }
                if ui.button("Load Bear").clicked() {
                    if let Some(bear) = FileDialog::new()
                        .add_filter("toml", &["toml"])
                        .pick_file()
                        .and_then(|path| read_to_string(path).ok())
                        .and_then(|contents| toml::from_str(&contents).ok())
                    {
                        self.state = State::Play(Play::new(bear));
                    }
                }
                if ui
                    .add_enabled(
                        matches!(self.state, State::Play(_)),
                        Button::new("Save Bear"),
                    )
                    .clicked()
                {
                    if let State::Play(Play { bear }) = &self.state {
                        if let Some(mut file) = FileDialog::new()
                            .add_filter("toml", &["toml"])
                            .save_file()
                            .and_then(|mut path| {
                                path.set_extension("toml");
                                File::create(path).ok()
                            })
                        {
                            write!(file, "{}", toml::to_string_pretty(bear).unwrap()).unwrap();
                        }
                    } else {
                        unreachable!();
                    }
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let new_state = match &mut self.state {
                State::Creator(creator) => creator.update(ui, &mut self.rng),
                State::Play(play) => play.update(ui, &mut self.rng),
            };
            if let Some(state) = new_state {
                self.state = state;
            }
        });
    }
}

impl Default for HoneyApp {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            state: State::Creator(Creator::new(&mut rng)),
            rng,
        }
    }
}
