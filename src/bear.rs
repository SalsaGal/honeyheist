use std::fmt::Display;

use rand::{seq::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum Descriptor {
    Rookie,
    WashedUp,
    Retired,
    Unhinged,
    Slick,
    Incompetent,
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rookie => write!(f, "rookie"),
            Self::WashedUp => write!(f, "washed up"),
            Self::Retired => write!(f, "retired"),
            Self::Unhinged => write!(f, "unhinged"),
            Self::Slick => write!(f, "slick"),
            Self::Incompetent => write!(f, "incompetent"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum Species {
    Grizzly,
    Polar,
    Panda,
    Black,
    Sun,
    HoneyBadger,
}

impl Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Grizzly => write!(f, "grizzly bear"),
            Self::Polar => write!(f, "polar bear"),
            Self::Panda => write!(f, "panda bear"),
            Self::Black => write!(f, "black bear"),
            Self::Sun => write!(f, "sun bear"),
            Self::HoneyBadger => write!(f, "honey badger"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum Role {
    Muscle,
    Brains,
    Driver,
    Hacker,
    Thief,
    Face,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Muscle => write!(f, "muscle"),
            Self::Brains => write!(f, "brains"),
            Self::Driver => write!(f, "driver"),
            Self::Hacker => write!(f, "hacker"),
            Self::Thief => write!(f, "thief"),
            Self::Face => write!(f, "face"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum Hat {
    Trilby,
    Top,
    Bowler,
    FlatCap,
    Cowboy,
    Fez,
    Crown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hats {
    pub hat: Hat,
    pub extra_hat: Option<Hat>,
}

impl Hats {
    pub fn gen(rng: &mut impl Rng) -> Self {
        let i = rng.gen_range(0..8);
        Hat::iter().nth(i).map_or(
            Hats {
                hat: Hat::iter().choose(rng).unwrap(),
                extra_hat: Some(Hat::iter().choose(rng).unwrap()),
            },
            |hat| Hats {
                hat,
                extra_hat: None,
            },
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub count: usize,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x {}", self.count, self.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bear {
    pub name: String,
    pub descriptor: Descriptor,
    pub species: Species,
    pub role: Role,
    #[serde(flatten)]
    pub hats: Hats,
    pub bear: u8,
    pub items: Vec<Item>,
}

impl Bear {
    pub fn new(rng: &mut impl Rng, name: String) -> Self {
        Self {
            name,
            descriptor: Descriptor::iter().choose(rng).unwrap(),
            species: Species::iter().choose(rng).unwrap(),
            role: Role::iter().choose(rng).unwrap(),
            hats: Hats::gen(rng),
            bear: 3,
            items: vec![],
        }
    }

    pub fn criminal(&self) -> u8 {
        6 - self.bear
    }
}
