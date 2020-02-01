#![feature(test)]

extern crate test;
extern crate wasm_game_of_life;
use wasm_game_of_life::{Universe, UniverseMode};

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    const SIZE: u32 = 1000;
    let mut universe = Universe::new(SIZE, SIZE, UniverseMode::FixedSizeNonPeriodic);

    b.iter(|| {
        universe.tick();
    });
}
