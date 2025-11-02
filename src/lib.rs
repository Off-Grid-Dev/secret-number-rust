use rand::Rng;
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pick_new_number() -> i32 {
    // Generate a random number between 1 and 100
    let number = rand::thread_rng().gen_range(1..=100);
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
