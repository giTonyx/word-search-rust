use std::cmp::max;
use word_search_rust::{dictionary, partial_puzzle, puzzle_solver};

fn generate(width: usize, height: usize, message: Option<&str>, dictionary_file: &str) -> String {
    let dictionary = dictionary::Dictionary::load_from_file(dictionary_file, max(width, height));
    let secret = match message {
        Some(s) => s.to_uppercase().to_string(),
        None => unimplemented!(),
    };

    loop {
        let mut partial_puzzle =
            partial_puzzle::PartialPuzzle::create(width, height, secret.clone());
        let mut last_word_count = partial_puzzle.word_count();

        while !partial_puzzle.is_complete() {
            for word in &dictionary {
                if partial_puzzle.try_insert_word(word) {
                    break;
                }
            }

            if partial_puzzle.word_count() == last_word_count {
                break; // We can't complete the problem
            }
            last_word_count = partial_puzzle.word_count();
        }

        if partial_puzzle.is_complete() {
            let puzzle_string = partial_puzzle.get_puzzle_string();
            let solver = puzzle_solver::PuzzleSolver::from_string(puzzle_string.clone());
            if let Some(_) = solver.solve() {
                return puzzle_string;
            }
        }
    }
}

fn main() {
    let arg_matches = clap::App::new("Word Search Generator")
        .arg(
            clap::Arg::with_name("x")
                .help("Width of the grid")
                .default_value("10"),
        )
        .arg(
            clap::Arg::with_name("y")
                .help("Height of the grid")
                .default_value("10"),
        )
        .arg(
            clap::Arg::with_name("solution")
                .short("s")
                .value_name("MESSAGE")
                .help("Message to encode. If missing will use a random word."),
        )
        .arg(
            clap::Arg::with_name("dictionary")
                .value_name("FILE")
                .short("d")
                .help("Dictionary file to use."),
        )
        .get_matches();

    let x = match arg_matches.value_of("x").unwrap().parse::<usize>() {
        Ok(n) => n,
        _ => {
            println!("Value X must be an unsigned number");
            return;
        }
    };
    let y = match arg_matches.value_of("y").unwrap().parse::<usize>() {
        Ok(n) => n,
        _ => {
            println!("Value Y must be an unsigned number");
            return;
        }
    };
    let message = arg_matches.value_of("solution");
    let dictionary_file = arg_matches
        .value_of("dictionary")
        .unwrap_or("assets/word_list.txt");

    let puzzle = generate(x, y, message, dictionary_file);
    println!("{}", puzzle);
}
