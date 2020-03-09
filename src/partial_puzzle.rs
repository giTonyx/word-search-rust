use crate::puzzle::{Location, DIRECTIONS};
use rand::Rng;
use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;

struct RandomDirectionIterator {
    offset: usize,
    index: usize,
}

impl RandomDirectionIterator {
    pub fn new() -> RandomDirectionIterator {
        let offset = rand::thread_rng().gen_range(0, DIRECTIONS.len());
        RandomDirectionIterator {
            offset: offset,
            index: 0,
        }
    }
}

impl Iterator for RandomDirectionIterator {
    type Item = (isize, isize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= DIRECTIONS.len() {
            return None;
        }
        let actual_index = (self.index + self.offset) % DIRECTIONS.len();
        self.index += 1;
        Some(DIRECTIONS[actual_index])
    }
}

pub struct PartialPuzzle {
    width: usize,
    heigth: usize,
    message: String,
    grid: Vec<Vec<char>>,
    words: HashSet<String>,
}

impl PartialPuzzle {
    pub fn create(width: usize, height: usize, message: String) -> PartialPuzzle {
        let mut grid = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(' ');
            }
            grid.push(row);
        }

        PartialPuzzle {
            width: width,
            heigth: height,
            message: message,
            grid: grid,
            words: HashSet::new(),
        }
    }

    fn get_random_empty_cell(&self) -> (usize, usize) {
        let mut x = rand::thread_rng().gen_range(0, self.width);
        let mut y = rand::thread_rng().gen_range(0, self.heigth);

        while self.grid[y][x] != ' ' {
            x += 1;
            if x == self.width {
                x = 0;
                y = (y + 1) % self.heigth;
            }
        }

        (x, y)
    }

    fn position_with_offset(&self, location: &Location, offset: isize) -> Option<(usize, usize)> {
        let isize_x = location.x as isize + (offset) * location.dx;
        let isize_y = location.y as isize + (offset) * location.dy;
        if isize_x < 0 || isize_y < 0 {
            return None;
        }
        let usize_x = isize_x as usize;
        let usize_y = isize_y as usize;
        if usize_x >= self.width || usize_y >= self.heigth {
            return None;
        }
        Some((usize_x, usize_y))
    }

    fn can_insert_word_at(&self, location: &Location, word: String) -> bool {
        let mut new_letters = 0;

        for i in 0..word.len() {
            let maybe_position = self.position_with_offset(location, i as isize);
            if maybe_position.is_none() {
                return false;
            }
            let (x, y) = maybe_position.unwrap();
            let grid_char = self.grid[y][x];

            if grid_char == ' ' {
                new_letters += 1;
                continue;
            }
            if grid_char != word.chars().nth(i).unwrap() {
                return false;
            }
        }

        if (new_letters + self.message.len()) > self.count_empty_cells() {
            return false;
        }
        true
    }

    fn insert_word_at(&mut self, location: &Location, word: String) {
        for i in 0..word.len() {
            let (x, y) = self.position_with_offset(location, i as isize).unwrap();
            self.grid[y][x] = word.chars().nth(i).unwrap();
        }
        self.words.insert(word);
    }

    fn find_string_start(
        &self,
        x: usize,
        y: usize,
        dx: isize,
        dy: isize,
        offset: usize,
    ) -> Option<(usize, usize)> {
        self.position_with_offset(
            &Location {
                x: x,
                y: y,
                dx: dx,
                dy: dy,
            },
            -(offset as isize),
        )
    }

    pub fn try_insert_word(&mut self, word: String) -> bool {
        let (x, y) = self.get_random_empty_cell();

        for (dx, dy) in RandomDirectionIterator::new() {
            for word_index in 0..word.len() {
                let maybe_position = self.find_string_start(x, y, dx, dy, word_index);
                if maybe_position.is_none() {
                    continue;
                }
                let (start_x, start_y) = maybe_position.unwrap();
                let location = Location {
                    x: start_x,
                    y: start_y,
                    dx: dx,
                    dy: dy,
                };

                if self.can_insert_word_at(&location, word.clone()) {
                    self.insert_word_at(&location, word);
                    return true;
                }
            }
        }
        false
    }

    fn count_empty_cells(&self) -> usize {
        let mut empty_cells = 0;
        for y in 0..self.heigth {
            for x in 0..self.width {
                if self.grid[y][x] == ' ' {
                    empty_cells += 1;
                }
            }
        }
        empty_cells
    }

    pub fn is_complete(&self) -> bool {
        self.count_empty_cells() == self.message.len()
    }

    pub fn get_puzzle_string(&self) -> String {
        let mut buffer = String::new();
        let mut message_index = 0;

        for y in 0..self.heigth {
            for x in 0..self.width {
                let c = self.grid[y][x];
                if c == ' ' {
                    buffer.push(self.message.chars().nth(message_index).unwrap());
                    message_index += 1;
                } else {
                    buffer.push(c);
                }
            }
            buffer.push('\n');
        }
        buffer.push('\n');

        buffer.push_str(Vec::from_iter(self.words.clone()).join("\n").as_str());

        buffer
    }

    pub fn word_count(&self) -> usize {
        self.words.len()
    }
}

impl fmt::Debug for PartialPuzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();

        self.grid.iter().for_each(|v| {
            buffer.push_str(v.iter().collect::<String>().as_str());
            buffer.push('\n')
        });
        write!(f, "{}", buffer)
    }
}
