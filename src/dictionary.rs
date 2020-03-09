use std::collections::HashSet;
use std::iter::FromIterator;

use rand::Rng;

const MIN_WORD_LEN: usize = 3;

pub struct Dictionary {
    words: Vec<String>,
}

impl Dictionary {
    pub fn load_from_file(filename: &str, max_len: usize) -> Dictionary {
        let mut word_set = HashSet::new();

        let data = std::fs::read_to_string(filename).unwrap();
        for line in data.split('\n') {
            let current_line: String = line.split_whitespace().collect();

            if current_line.len() < MIN_WORD_LEN {
                continue;
            }
            if max_len > 0 && current_line.len() > max_len {
                continue;
            }
            if current_line.chars().next().unwrap() == '#' {
                continue;
            }
            word_set.insert(current_line.to_uppercase());
        }

        Dictionary {
            words: Vec::from_iter(word_set),
        }
    }
}

impl<'a> IntoIterator for &'a Dictionary {
    type Item = String;
    type IntoIter = DictionarylIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DictionarylIterator {
            dictionary: self,
            index: 0,
            offset: rand::thread_rng().gen_range(0, self.words.len()),
        }
    }
}

pub struct DictionarylIterator<'a> {
    dictionary: &'a Dictionary,
    index: usize,
    offset: usize,
}

impl<'a> Iterator for DictionarylIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.dictionary.words.len() {
            return None;
        }
        let actual_index = (self.index + self.offset) % self.dictionary.words.len();
        self.index += 1;

        Some(self.dictionary.words[actual_index].clone())
    }
}
