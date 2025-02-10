
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};

pub fn generate_fruit() -> String {
    let fruits = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Date".to_string(),
        "Elderberry".to_string(),
        "Fig".to_string(),
        "Grape".to_string(),
        "Honeydew".to_string(),
    ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap().to_string()
}

pub fn calculate_fruit_frequency(fruit_set: &HashSet<String>) -> HashMap<String, u32> {
    let mut fruit_frequency = HashMap::new();
    for fruit in fruit_set {
        let count = fruit_frequency.entry(fruit.to_string()).or_insert(0);
        *count += 1;
    }
    fruit_frequency
}