/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/
use clap::Parser;
use vector_fruit_salad::create_fruit_salad;

#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Phil Lehman <lehmanpm@gmail.com>",
    about = "Displays fruits to include in the salad"
)]
struct Args {
    #[clap(short, long)]
    number: usize,
    fruits: Vec<String>,
}

fn main() {
    let opts: Args = Args::parse();
    let mut fruit_salad = create_fruit_salad(opts.number);

    // Add the fruits from the command line to the fruit salad
    for fruit in opts.fruits {
        vector_fruit_salad::add_fruits_to_salad(fruit, &mut fruit_salad);
    }

    println!("Chosen fruits: {}", vector_fruit_salad::choose_fruit_from_salad(&fruit_salad));

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit_salad.iter().enumerate() {
        if i != fruit_salad.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
