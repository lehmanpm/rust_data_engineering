/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/
use clap::Parser;
use std::collections::VecDeque;
use vecdeque_fruit_salad::*;

#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Phil Lehman <lehmanpm@gmail.com>",
    about = "Displays fruits to include in the salad"
)]
struct Args {
    #[clap(short, long, use_value_delimiter = true)]
    fruits: Vec<String>,
}

fn main() {
    let opts: Args = Args::parse();
    let mut fruit: VecDeque<String> = create_fruit_salad();
    fruit.push_back(String::from("Arbutus"));
    fruit.push_back(String::from("Loquat"));
    fruit.push_back(String::from("Strawberry Tree Berry"));

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front(String::from("Pomegranate"));
    fruit.push_back(String::from("Fig"));
    fruit.push_back(String::from("Cherry"));

    for f in opts.fruits {
        add_fruits_to_salad(f, &mut fruit);
    }
    
    sort_fruit_salad(&mut fruit);

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
