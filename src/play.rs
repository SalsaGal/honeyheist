use egui::Ui;
use rand::Rng;

use crate::{
    bear::{Bear, Descriptor},
    State,
};

pub struct Play {
    pub bear: Bear,
}

impl Play {
    pub fn new(bear: Bear) -> Self {
        Self { bear }
    }

    pub fn update(&mut self, ui: &mut Ui, rng: &mut impl Rng) -> Option<State> {
        ui.heading(&self.bear.name);
        let article = match self.bear.descriptor {
            Descriptor::Unhinged | Descriptor::Incompetent => "an",
            _ => "a",
        };
        ui.label(format!(
            "You are {article} {} {}, acting as the group's {}",
            self.bear.descriptor, self.bear.species, self.bear.role
        ));
        None
    }
}
