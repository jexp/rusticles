//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use fixedbitset::FixedBitSet;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    Universe::init(6,6,&[(1,2), (2,3), (3,1), (3,2), (3,3)])
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    Universe::init(6,6,&[(2,1), (2,3), (3,2), (3,3), (4,2)])
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(input_universe.cell_tuples(), *expected_universe.cell_tuples());
}