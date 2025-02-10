use std::collections::VecDeque;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad() -> VecDeque<String> {
    let mut fruit: VecDeque<String> = VecDeque::new();
    fruit.push_back(String::from("Orange"));
    fruit.push_back(String::from("Apple"));
    fruit.push_back(String::from("Pear"));
    fruit.push_back(String::from("Peach"));

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit_vec: Vec<String> = fruit.into_iter().collect();
    fruit_vec.shuffle(&mut rng);

    // Convert it back to VecDeque
    fruit = fruit_vec.into_iter().collect();
    fruit
}

pub fn add_fruits_to_salad(fruit: String, fruit_salad: &mut VecDeque<String>) {
    fruit_salad.push_back(fruit);
}

pub fn sort_fruit_salad(fruit_salad: &mut VecDeque<String>) {
    fruit_salad.make_contiguous().sort();
}