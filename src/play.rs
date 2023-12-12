use std::io::Write;
use std::{fs::File, path::PathBuf};

use dirtytype::Dirty;
use egui::{Color32, DragValue, RichText, TextEdit, Ui};
use rand::Rng;

use crate::bear::Item;
use crate::{
    bear::{Bear},
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
        ui.label(format!("{}", *self.bear));

        if self.bear.bear == 6 {
            ui.label(RichText::new("Gone Wild!").color(Color32::RED));
        } else if self.bear.criminal() == 6 {
            ui.label(RichText::new("Gone Criminal!").color(Color32::RED));
        }

        ui.heading("Items:");
        let mut changed_item = None;
        let mut increase_bear = false;
        for (index, item) in self.bear.items.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                let response = ui.add(DragValue::new(&mut item.count).max_decimals(0));
                ui.label(item.name.to_string());
                if item.name == "Honey" {
                    if item.count > 0 && ui.button("Eat").clicked() {
                        item.count -= 1;
                        increase_bear = true;
                    }
                } else if (response.drag_released() || response.lost_focus()) && item.count == 0 {
                    changed_item = Some(index);
                }
            });
        }
        if increase_bear {
            self.bear.bear = (self.bear.bear + 1).min(6);
        }
        if let Some(item) = changed_item {
            self.bear.items.remove(item);
        }
        if ui
            .add(TextEdit::singleline(&mut self.new_item).hint_text("New Item"))
            .lost_focus()
            && !self.new_item.is_empty()
        {
            for item in std::mem::take(&mut self.new_item).split(',').map(str::trim) {
                let name = item
                    .chars()
                    .skip_while(|c| char::is_digit(*c, 10))
                    .collect::<String>()
                    .trim()
                    .to_owned();
                let count = item
                    .chars()
                    .take_while(|c| char::is_digit(*c, 10))
                    .collect::<String>()
                    .parse()
                    .unwrap_or(1);

                if let Some(item) = self.bear.items.iter_mut().find(|item| item.name == name) {
                    item.count += count;
                } else {
                    self.bear.items.push(Item { name, count });
                }
            }
        }

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Add Bear").clicked() {
                self.bear.bear = (self.bear.bear + 1).min(6);
            }
            if ui.button("Add Criminal").clicked() {
                self.bear.bear = self.bear.bear.saturating_sub(1);
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
                    self.bear.bear = self.bear.bear.saturating_sub(1);
                } else {
                    self.bear.bear = (self.bear.bear + 1).min(6);
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
