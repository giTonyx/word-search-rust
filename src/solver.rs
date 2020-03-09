use word_search_rust::puzzle_solver::PuzzleSolver;

fn main() {
    let arg_matches = clap::App::new("Word Search Solver")
        .arg(clap::Arg::with_name("input_file").required(true))
        .get_matches();
    let input_filename = arg_matches.value_of("input_file").unwrap();

    let data = std::fs::read_to_string(input_filename).unwrap();
    let solver = PuzzleSolver::from_string(data);
    match solver.solve() {
        None => println!("Not solvable"),
        Some(s) => println!("{}", s),
    }
}
