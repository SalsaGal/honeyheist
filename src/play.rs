use egui::Ui;
use rand::Rng;

use crate::{bear::Bear, State};

pub struct Play {
    pub bear: Bear,
}

impl Play {
    pub fn new(bear: Bear) -> Self {
        Self { bear }
    }

    pub fn update(&mut self, ui: &mut Ui, rng: &mut impl Rng) -> Option<State> {
        ui.heading(&self.bear.name);
        None
    }
}
