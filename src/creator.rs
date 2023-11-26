use egui::{ComboBox, Ui};
use rand::Rng;
use strum::IntoEnumIterator;

use crate::{
    bear::{Bear, Descriptor, Hat, Role, Species},
    play::Play,
    State,
};

pub struct Creator {
    pub bear: Bear,
    has_extra_hat: bool,
}

impl Creator {
    pub fn new(rng: &mut impl Rng) -> Self {
        let bear = Bear::new(rng, "Unnamed Bear".to_owned());

        Self {
            has_extra_hat: bear.hats.extra_hat.is_some(),
            bear,
        }
    }

    pub fn update(&mut self, ui: &mut Ui, rng: &mut impl Rng) -> Option<State> {
        macro_rules! enum_field {
            ($ty: ident, $value: expr) => {
                ui.horizontal(|ui| {
                    ComboBox::from_label(stringify!($ty))
                        .selected_text(format!("{:?}", $value))
                        .show_ui(ui, |ui| {
                            for variant in $ty::iter() {
                                ui.selectable_value(&mut $value, variant, format!("{variant:?}"));
                            }
                        });
                })
            };
        }

        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.bear.name);
        });
        enum_field!(Descriptor, self.bear.descriptor);
        enum_field!(Species, self.bear.species);
        enum_field!(Role, self.bear.role);
        enum_field!(Hat, self.bear.hats.hat);
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.has_extra_hat, "Has extra hat");
            ComboBox::from_label("Extra Hat")
                .selected_text(format!(
                    "{:?}",
                    self.bear.hats.extra_hat.unwrap_or(Hat::Trilby)
                ))
                .show_ui(ui, |ui| {
                    ui.set_enabled(self.has_extra_hat);
                    for variant in Hat::iter() {
                        ui.selectable_value(
                            &mut self.bear.hats.extra_hat.unwrap_or(Hat::Trilby),
                            variant,
                            format!("{variant:?}"),
                        );
                    }
                });
        });

        if ui.button("Reroll").clicked() {
            self.bear = Bear::new(rng, std::mem::take(&mut self.bear.name));
        } else if ui.button("Play").clicked() {
            return Some(State::Play(Play::new(self.bear.clone())));
        }

        None
    }
}
