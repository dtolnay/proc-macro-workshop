use std::time::{SystemTime, UNIX_EPOCH};

use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use termcolor::Color;
use termcolor::Color::*;

use crate::term;

const WIDTH: usize = 30;
const COLORS: &[Color] = &[Blue, Green, Red, Yellow, Yellow];
const CHARS: &[char] = &['░', '▒'];

pub fn colorful() {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");
    let seed = u64::from(since_epoch.subsec_nanos());

    println!();
    let mut rng = XorShiftRng::seed_from_u64(seed);

    for _ in 0..WIDTH {
        let color = COLORS[rng.next_u32() as usize % COLORS.len()];
        term::color(color);

        let ch = CHARS[rng.next_u32() as usize % CHARS.len()];
        print!("{}{}", ch, ch);
    }

    term::reset();
    print!("\n\n");
}

pub fn dotted() {
    println!("{}", "┈┈".repeat(WIDTH));
}
