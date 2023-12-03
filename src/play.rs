use std::io::Write;
use std::{fs::File, path::PathBuf};

use dirtytype::Dirty;
use egui::{Color32, Label, RichText, Ui};
use rand::Rng;

use crate::{
    bear::{Bear, Descriptor},
    State,
};

pub struct Play {
    pub bear: Dirty<Bear>,
    path: Option<PathBuf>,
    last_roll: Option<(u8, bool)>,
    new_item: String,
}

impl Play {
    pub fn new(bear: Bear, path: Option<PathBuf>) -> Self {
        Self {
            bear: Dirty::new(bear),
            path,
            last_roll: None,
            new_item: String::new(),
        }
    }

    pub fn update(&mut self, ui: &mut Ui, rng: &mut impl Rng) -> Option<State> {
        if let Some(path) = &self.path {
            write!(
                File::create(path).unwrap(),
                "{}",
                toml::to_string_pretty(&self.bear.data).unwrap(),
            )
            .unwrap();
        }

        ui.heading(&self.bear.name);
        let article = match self.bear.descriptor {
            Descriptor::Unhinged | Descriptor::Incompetent => "an",
            _ => "a",
        };
        ui.label(format!(
            "You are {article} {} {}, acting as the group's {}.\n{} bear points, {} criminal points. You are wearing {}",
            self.bear.descriptor,
            self.bear.species,
            self.bear.role,
            self.bear.bear,
            self.bear.criminal(),
            self.bear.hats,
        ));

        if self.bear.bear == 6 {
            ui.label(RichText::new("Gone Wild!").color(Color32::RED));
        } else if self.bear.criminal() == 6 {
            ui.label(RichText::new("Gone Criminal!").color(Color32::RED));
        }

        ui.heading("Items:");
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
                    self.bear.bear -= 1;
                } else {
                    self.bear.bear += 1;
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
