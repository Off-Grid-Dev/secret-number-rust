use rand::Rng;
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

const MIN_VALUE: i32 = 1;
const MAX_VALUE: i32 = 100;

#[wasm_bindgen]
pub fn min_value() -> i32 {
    MIN_VALUE
}

#[wasm_bindgen]
pub fn max_value() -> i32 {
    MAX_VALUE
}

#[wasm_bindgen]
pub fn pick_new_number() -> i32 {
    // Generate a random number between the given "min" and "max" values
    let number = rand::thread_rng().gen_range(MIN_VALUE..=MAX_VALUE);
    number
}

#[wasm_bindgen]
pub fn check_number(secret: i32, guess: i32) -> String {
    match guess.cmp(&secret) {
        Ordering::Less => "less".to_string(),
        Ordering::Greater => "more".to_string(),
        Ordering::Equal => "match".to_string(),
    }
}
