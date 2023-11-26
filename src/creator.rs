use egui::Ui;
use rand::Rng;

use crate::bear::Bear;

pub struct Creator {
    pub bear: Bear,
}

impl Creator {
    pub fn new(rng: &mut impl Rng) -> Self {
        Self {
            bear: Bear::new(rng, "Untitled Bear".to_owned()),
        }
    }

    pub fn update(&mut self, ui: &mut Ui, rng: &mut impl Rng) {}
}
