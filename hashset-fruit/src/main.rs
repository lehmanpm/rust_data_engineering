
use clap::Parser;
use std::collections::HashSet;
use hashset_fruit::*;

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
    let mut fruit_set = HashMap::new();
    println!("Generating 100 random fruits...");
    for _ in 0..100 {
        fruit_set.insert(generate_fruit());
    }

    println!("{:?}", fruit_set);

    // Add the fruits from the command line to the fruit salad
    for fruit in opts.fruits {
        fruit_set.insert(fruit);
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());
    println!("Frequency of each fruit:");
    let fruit_frequency = calculate_fruit_frequency(&fruit_set);
    for (fruit, count) in fruit_frequency {
        println!("{}: {}", fruit, count);
    }
}
