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
            "You are {article} {} {}, acting as the group's {}. {} bear points, {} criminal points",
            self.bear.descriptor,
            self.bear.species,
            self.bear.role,
            self.bear.bear,
            6 - self.bear.bear,
        ));

        ui.label(format!("Items:"));
        for item in &self.bear.items {
            ui.label(format!(" - {item}"));
        }

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Add Bear").clicked() {
                self.bear.bear += 1;
            }
            if ui.button("Add Criminal").clicked() {
                self.bear.bear -= 1;
            }
        });

        None
    }
}
