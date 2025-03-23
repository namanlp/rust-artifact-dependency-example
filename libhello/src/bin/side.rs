use libhello;

use rand::prelude::*;


macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

pub fn random_add() -> usize{
    let mut rng = rand::rng();
    let x = (rng.random::<u64>() % 10000) as usize;
    let y = (rng.random::<u64>() % 10000) as usize;
    unsafe {libhello::adding_num(x, y)}
}

fn main() {
    p!("Hello, from side_bin!");
    p!("Addition of Random Numbers = {}", random_add())
}