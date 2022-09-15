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

const TOTAL_ATTEMPTS: u8 = 5;

fn main() {
    let mut word_to_find = Word::new_random_from_file("words.txt");

    let mut remaining_attempts: u8 = TOTAL_ATTEMPTS;

    loop {
        println!("\nAttempts left: {}", remaining_attempts);
        print_word_to_find(&word_to_find);

        println!("Enter a letter: ");
        let input = read_first_char_from_console();

        let mut input_matched = false;

        for letter in word_to_find.letters.iter_mut() {
            if letter.character == input {
                letter.is_revealed = true;
                input_matched = true;
            }
        }

        if !input_matched {
            remaining_attempts -= 1;
        }

        match word_to_find.game_state(remaining_attempts) {
            GameState::Won => {
                println!("Congratulation you won!");
                break;
            }
            GameState::Lost => {
                println!("Bad lock, you are out of attempts!");
                println!("The word was: {}", word_to_find.word);
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

