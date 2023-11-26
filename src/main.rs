use creator::Creator;
use eframe::{App, NativeOptions};
use play::Play;
use rand::rngs::ThreadRng;

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
        Box::new(|_| Box::<HoneyApp>::default()),
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
