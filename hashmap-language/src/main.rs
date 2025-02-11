//use clap::Parser;
use hashmap_language::*;

// #[derive(Parser, Debug)]
// #[clap(
//     version = "1.0",
//     author = "Phil Lehman <lehmanpm@gmail.com>",
//     about = "Displays fruits to include in the salad"
// )]
// struct Args {
//     #[clap(short, long)]
//     language: String,
//     year: i32,
// }

fn main() {
    //let opts: Args = Args::parse();
    let mut languages = init_languages();

    // if !opts.language.is_empty() {
    //     let cmd_line_language = opts.language;
    //     let cmd_line_year = opts.year;
    //     languages.insert(cmd_line_language, cmd_line_year);
    // }

    loop {
        let mut choice = String::new();
        println!("Would you like to add another language? (y/n)");
        std::io::stdin().read_line(&mut choice).unwrap();
        if choice.trim() == "y" {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let details : Vec<&str> = input.trim().split(' ').collect();
            let language = details[0].to_string();
            let year = details[1].parse::<i32>().unwrap();
            languages.insert(language, year);
        } else {
            break;
        }
    } 

    let weights = calculate_weights(&mut languages);
    let mut sorted_weights = weights.iter().collect::<Vec<_>>();
    sorted_weights.sort_by(|a, b| b.1.cmp(a.1));
    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &sorted_weights {
        println!("{}: {}", language, weight);
    }
}
