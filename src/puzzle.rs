use std::collections::HashSet;
use std::fmt;

lazy_static! {
    pub static ref DIRECTIONS: Vec<(isize, isize)> = vec![
        (1isize, 0isize),
        (1isize, -1isize),
        (0isize, -1isize),
        (-1isize, -1isize),
        (-1isize, 0isize),
        (-1isize, 1isize),
        (0isize, 1isize),
        (1isize, 1isize),
    ];
}

pub struct Location {
    pub x: usize,
    pub y: usize,
    pub dx: isize,
    pub dy: isize,
}

pub struct Puzzle {
    pub grid: Vec<Vec<char>>,
    pub words: HashSet<String>,
    pub width: usize,
    pub height: usize,
}

impl Puzzle {
    pub fn from_string(input: String) -> Puzzle {
        let mut done_reading_grid = false;

        let mut words = HashSet::new();
        let mut grid = Vec::new();

        for line in input.split('\n') {
            let current_line: String = line.split_whitespace().collect();

            if current_line.len() == 0 {
                done_reading_grid = true;
                continue;
            }
            if current_line.chars().next().unwrap() == '#' {
                continue;
            }

            if done_reading_grid {
                words.insert(current_line);
            } else {
                grid.push(current_line.chars().collect::<Vec<char>>());
            }
        }
        let height = grid.len();
        let width = grid[0].len();
        Puzzle {
            grid: grid,
            words: words,
            width: width,
            height: height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.grid[y][x])
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();

        self.grid.iter().for_each(|v| {
            buffer.push_str(v.iter().collect::<String>().as_str());
            buffer.push('\n')
        });

        buffer.push('\n');
        self.words.iter().for_each(|w| {
            buffer.push_str(w);
            buffer.push('\n')
        });

        write!(f, "{}", buffer)
    }
}
