use egui::Ui;
use rand::Rng;

use crate::{
    bear::{Bear, Descriptor},
    State,
};

pub struct Play {
    pub bear: Bear,
    last_roll: Option<(u8, bool)>,
}

impl Play {
    pub fn new(bear: Bear) -> Self {
        Self {
            bear,
            last_roll: None,
        }
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
            self.bear.criminal(),
        ));

        ui.label("Items:");
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
        ui.horizontal(|ui| {
            let mut roll = None;
            if ui.button("Roll Bear").clicked() {
                roll = Some((rng.gen_range(1..=6), self.bear.bear));
            }
            if ui.button("Roll Criminal").clicked() {
                roll = Some((rng.gen_range(1..=6), self.bear.criminal()));
            }

            if let Some((roll, skill)) = roll {
                let success = roll <= skill;
                if success {
                    self.bear.bear += 1;
                } else {
                    self.bear.bear -= 1;
                }
                self.last_roll = Some((roll, success));
            }

            if let Some((last_roll, success)) = self.last_roll {
                ui.label(format!(
                    "Rolled {last_roll}, {}",
                    if success { "Success!" } else { "Failed" },
                ));
            }
        });

        None
    }
}
