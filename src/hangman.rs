extern crate rand;
extern crate core;

use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use std::io::stdin;


struct Letter {
    character: char,
    is_revealed: bool,
}

enum GameState {
    InProgress,
    Won,
    Lost,
}

struct Word {
    word: String,
    letters: Vec<Letter>,
}

impl Word {
    fn game_state(&self, remaining_attempts: u8) -> GameState {
        let mut all_revealed = true;
        for letter in &self.letters {
            if !letter.is_revealed {
                all_revealed = false;
            }
        }

        return if all_revealed {
            GameState::Won
        } else if remaining_attempts > 0 {
            GameState::InProgress
        } else {
            GameState::Lost
        };
    }

    fn new_random_from_file(file_name: &str) -> Word {
        let mut file = File::open(file_name).expect(&format!("Can't open {}", file_name));
        let mut content = String::new();
        file.read_to_string(&mut content).expect(&format!("Can't read from {}", file_name));
        let words: Vec<String> = content.split(',')
            .map(|word| word.to_string())
            .collect();
        let random_index = rand::thread_rng().gen_range(0..words.len());
        let word = words.get(random_index).expect(&format!("Could not select word at index {}", random_index)).to_string();

        return Word {
            word: String::from(&word),
            letters: word.chars()
                .map(|char| Letter {
                    character: char,
                    is_revealed: false,
                }).collect(),
        };
    }
}


pub struct HangmanGame {
    word_to_find: Word,
    remaining_attempts: u8,
}

impl HangmanGame {
    pub fn new_random_from_file(file_name: &str, remaining_attempts: u8) -> HangmanGame {
        return HangmanGame {
            word_to_find: Word::new_random_from_file(&file_name),
            remaining_attempts,
        };
    }

    pub fn start(&mut self) {
        loop {
            println!("\nAttempts left: {}", self.remaining_attempts);
            Self::print_word_to_find(&self.word_to_find);

            println!("Enter a letter: ");
            let input = Self::read_first_char_from_console();

            let mut input_matched = false;

            for letter in self.word_to_find.letters.iter_mut() {
                if letter.character == input {
                    letter.is_revealed = true;
                    input_matched = true;
                }
            }

            if !input_matched {
                self.remaining_attempts -= 1;
            }

            match self.word_to_find.game_state(self.remaining_attempts) {
                GameState::Won => {
                    println!("Congratulation you won!");
                    break;
                }
                GameState::Lost => {
                    println!("Bad lock, you are out of attempts!");
                    println!("The word was: {}", self.word_to_find.word);
                    println!("Try again!");
                    break;
                }
                GameState::InProgress => {
                    continue;
                }
            }
        }
    }

    fn read_first_char_from_console() -> char {
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Invalid input");
        return input.chars().next().expect("Can't get first char from input");
    }

    fn print_word_to_find(word: &Word) {
        let mut output = String::new();
        output.push_str("Word to find: ");
        for letter in &word.letters {
            output.push(' ');
            if letter.is_revealed {
                output.push(letter.character);
            } else {
                output.push('_');
            }
        }
        println!("{}", output);
    }
}
