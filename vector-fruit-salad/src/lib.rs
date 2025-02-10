use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(number: usize) -> Vec<String> {
    let mut fruit = vec![
        String::from("Orange"),
        String::from("Fig"),
        String::from("Pomegranate"),
        String::from("Cherry"),
        String::from("Apple"),
        String::from("Pear"),
        String::from("Peach"),
    ];

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    // Return the first `number` fruits
    fruit.into_iter().take(number).collect()
}

pub fn add_fruits_to_salad(fruit: String, fruit_salad: &mut Vec<String>) {
    fruit_salad.push(fruit);
}

pub fn choose_fruit_from_salad(fruit_salad: &[String]) -> &str {
    let mut rng = thread_rng();
    fruit_salad.choose(&mut rng).unwrap()
}