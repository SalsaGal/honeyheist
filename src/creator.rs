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
        let bear = Bear::new(rng);

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
            if ui
                .checkbox(&mut self.has_extra_hat, "Has extra hat")
                .clicked()
            {
                self.bear.hats.extra_hat = if self.has_extra_hat {
                    Some(Hat::Trilby)
                } else {
                    None
                }
            }
            if let Some(extra_hat) = &mut self.bear.hats.extra_hat {
                ComboBox::from_label("")
                    .selected_text(format!("{extra_hat:?}"))
                    .show_ui(ui, |ui| {
                        for hat in Hat::iter() {
                            ui.selectable_value(extra_hat, hat, format!("{hat:?}"));
                        }
                    });
            }
        });

        let mut to_ret = None;
        ui.horizontal(|ui| {
            if ui.button("Reroll").clicked() {
                self.bear = Bear::new(rng);
                self.has_extra_hat = self.bear.hats.extra_hat.is_some();
            } else if ui.button("Play").clicked() {
                to_ret = Some(State::Play(Play::new(self.bear.clone(), None)));
            }
        });

        to_ret
    }
}
