pub enum Descriptor {
    Rookie,
    WashedUp,
    Retired,
    Unhinged,
    Slick,
    Incompetent,
}

pub enum Species {
    Grizzly,
    Polar,
    Panda,
    Black,
    Sun,
    HoneyBadger,
}

pub enum Role {
    Muscle,
    Brains,
    Driver,
    Hacker,
    Thief,
    Face,
}

pub enum Hat {
    Trilby,
    Top,
    Bowler,
    FlatCap,
    Cowboy,
    Fez,
    Crown,
}

pub struct Hats {
    pub hat: Hat,
    pub extra_hat: Option<Hat>,
}

pub struct Bear {
    pub name: String,
    pub descriptor: Descriptor,
    pub species: Species,
    pub role: Role,
    pub hats: Hats,
}
