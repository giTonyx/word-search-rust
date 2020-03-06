extern crate clap;
#[macro_use]
extern crate lazy_static;

mod puzzle;
use puzzle::{Location, Puzzle, DIRECTIONS};
use std::collections::{HashMap, HashSet};

struct Solver {
    puzzle: Puzzle,
}

impl Solver {
    fn from_string(input: String) -> Solver {
        Solver {
            puzzle: Puzzle::from_string(input),
        }
    }

    fn mark_location(location: &Location, len: usize, marked_cells: &mut HashSet<(usize, usize)>) {
        for offset in 0..len {
            if let Some((x, y)) = Solver::location_step(location, offset) {
                marked_cells.insert((x, y));
            }
        }
    }

    fn position_with_offset(start: usize, delta: isize, number_of_steps: usize) -> Option<usize> {
        let final_position = start as isize + (number_of_steps as isize * delta);
        if final_position < 0 {
            None
        } else {
            Some(final_position as usize)
        }
    }

    fn location_step(location: &Location, step: usize) -> Option<(usize, usize)> {
        let maybe_x = Solver::position_with_offset(location.x, location.dx, step);
        let maybe_y = Solver::position_with_offset(location.y, location.dy, step);
        match (maybe_x, maybe_y) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    }

    fn get_valid_word(
        &self,
        location: &Location,
        len: usize,
        marked_cells: &HashSet<(usize, usize)>,
    ) -> Option<String> {
        let mut unmarked_letters = 0;
        let mut buffer = String::new();

        for offset in 0..len {
            if let Some((x, y)) = Solver::location_step(location, offset) {
                if !marked_cells.contains(&(x, y)) {
                    unmarked_letters += 1;
                }
                match self.puzzle.get(x, y) {
                    None => break,
                    Some(c) => buffer.push(c),
                }
            }
        }

        if unmarked_letters == 0 {
            None
        } else {
            Some(buffer)
        }
    }

    fn search_and_mark(
        &self,
        word: &String,
        marked_cells: &mut HashSet<(usize, usize)>,
        letter_map: &HashMap<char, HashSet<(usize, usize)>>,
    ) -> bool {
        let mut locations = Vec::new();
        for (x, y) in letter_map
            .get(&(word.chars().next().unwrap()))
            .unwrap()
            .iter()
        {
            for (dx, dy) in DIRECTIONS.iter() {
                let location = Location {
                    x: *x,
                    y: *y,
                    dx: *dx,
                    dy: *dy,
                };
                if let Some(w) = self.get_valid_word(&location, word.len(), marked_cells) {
                    if w == *word {
                        locations.push(location);
                    }
                }
            }
        }

        if locations.len() != 1 {
            return false;
        }
        Solver::mark_location(&locations[0], word.len(), marked_cells);
        true
    }

    fn create_letter_map(&self) -> HashMap<char, HashSet<(usize, usize)>> {
        let mut letter_map = HashMap::new();
        for i in 0..self.puzzle.width {
            for j in 0..self.puzzle.height {
                let c = self.puzzle.get(i, j).unwrap();
                if !letter_map.contains_key(&c) {
                    letter_map.insert(c, HashSet::new());
                }
                letter_map.get_mut(&c).unwrap().insert((i, j));
            }
        }
        letter_map
    }

    fn collect_unmarked(&self, marked_cells: &HashSet<(usize, usize)>) -> String {
        let mut buffer = String::new();

        for j in 0..self.puzzle.height {
            for i in 0..self.puzzle.width {
                let c = self.puzzle.get(i, j).unwrap();
                if !marked_cells.contains(&(i, j)) {
                    buffer.push(c);
                }
            }
        }

        buffer
    }

    fn solve(&self) -> Option<String> {
        let mut words_to_find = HashSet::new();
        for word in self.puzzle.words.iter() {
            words_to_find.insert(word);
        }

        let mut marked_cells = HashSet::new();
        let letter_map: HashMap<char, HashSet<(usize, usize)>> = self.create_letter_map();

        let mut previous_found_word_size = 0;
        while words_to_find.len() > 0 {
            // If we couldn't find a word at last iteration, we are stuck.
            if words_to_find.len() == previous_found_word_size {
                return None;
            }
            previous_found_word_size = words_to_find.len();

            words_to_find.retain(|w| !self.search_and_mark(w, &mut marked_cells, &letter_map));
        }

        Some(self.collect_unmarked(&marked_cells))
    }
}

fn main() {
    let arg_matches = clap::App::new("Word Search Solver")
        .arg(clap::Arg::with_name("input_file").required(true))
        .get_matches();
    let input_filename = arg_matches.value_of("input_file").unwrap();

    let data = std::fs::read_to_string(input_filename).unwrap();
    let solver = Solver::from_string(data);
    match solver.solve() {
        None => println!("Not solvable"),
        Some(s) => println!("{}", s),
    }
}
