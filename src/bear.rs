use rand::{seq::IteratorRandom, Rng};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum Descriptor {
    Rookie,
    WashedUp,
    Retired,
    Unhinged,
    Slick,
    Incompetent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum Species {
    Grizzly,
    Polar,
    Panda,
    Black,
    Sun,
    HoneyBadger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum Role {
    Muscle,
    Brains,
    Driver,
    Hacker,
    Thief,
    Face,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum Hat {
    Trilby,
    Top,
    Bowler,
    FlatCap,
    Cowboy,
    Fez,
    Crown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hats {
    pub hat: Hat,
    pub extra_hat: Option<Hat>,
}

impl Hats {
    pub fn gen(rng: &mut impl Rng) -> Self {
        let i = rng.gen_range(0..8);
        Hat::iter()
            .nth(i)
            .map(|hat| Hats {
                hat,
                extra_hat: None,
            })
            .unwrap_or(Hats {
                hat: Hat::iter().choose(rng).unwrap(),
                extra_hat: Some(Hat::iter().choose(rng).unwrap()),
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bear {
    pub name: String,
    pub descriptor: Descriptor,
    pub species: Species,
    pub role: Role,
    pub hats: Hats,
}

impl Bear {
    pub fn new(rng: &mut impl Rng, name: String) -> Self {
        Self {
            name,
            descriptor: Descriptor::iter().choose(rng).unwrap(),
            species: Species::iter().choose(rng).unwrap(),
            role: Role::iter().choose(rng).unwrap(),
            hats: Hats::gen(rng),
        }
    }
}
