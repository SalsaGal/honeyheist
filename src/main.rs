use rand::thread_rng;

use crate::bear::Bear;

mod bear;

fn main() {
    let mut rng = thread_rng();
    let bear = Bear::new(&mut rng, "Beepboop".to_owned());
    println!("{}", toml::to_string_pretty(&bear).unwrap());
}
