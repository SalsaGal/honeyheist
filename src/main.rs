use std::fs::{read_to_string, File};
use std::io::Write;

use bear::Bear;
use creator::Creator;
use eframe::{App, NativeOptions};
use rand::rngs::ThreadRng;
use rfd::FileDialog;

mod bear;
mod creator;

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

enum State {
    Creator(Creator),
    Play(Bear),
}

struct HoneyApp {
    rng: ThreadRng,
    state: State,
}

impl App for HoneyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("control").show(ctx, |ui| {
            ui.heading("Honey Heist");
        });
        egui::CentralPanel::default().show(ctx, |ui| match &mut self.state {
            State::Creator(creator) => creator.update(ui, &mut self.rng),
            State::Play(_) => todo!(),
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
