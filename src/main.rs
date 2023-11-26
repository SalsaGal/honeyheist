use rand::thread_rng;

use crate::bear::Bear;

mod bear;

fn main() {
    let mut rng = thread_rng();
    dbg!(Bear::new(&mut rng, "Beepboop".to_owned()));
}
