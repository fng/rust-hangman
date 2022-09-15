extern crate rand;

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

const TOTAL_ATTEMPTS: u8 = 5;

fn main() {
    let words = read_words_from_file("words.txt");

    let selected_word = select_random_word(&words);

    let mut letters = word_to_letters(&selected_word);

    let mut remaining_attempts: u8 = TOTAL_ATTEMPTS;

    loop {
        println!("\nAttempts left: {}", remaining_attempts);
        print_letters(&letters);

        println!("Enter a letter: ");
        let input = read_first_char_from_console();

        let mut input_matched = false;

        for letter in letters.iter_mut() {
            if letter.character == input {
                letter.is_revealed = true;
                input_matched = true;
            }
        }

        if !input_matched {
            remaining_attempts -= 1;
        }

        match calculate_game_state(remaining_attempts, &letters) {
            GameState::Won => {
                println!("Congratulation you won!");
                break;
            }
            GameState::Lost => {
                println!("Bad lock, you are out of attempts!");
                println!("The word was: {}", selected_word);
                println!("Try again!");
                break;
            }
            GameState::InProgress => {
                continue;
            }
        }
    }
}

fn read_words_from_file(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).expect(&format!("Can't open {}", filename));
    let mut content = String::new();
    file.read_to_string(&mut content).expect(&format!("Can't read from {}", filename));

    return content.split(',')
        .map(|word| word.to_string())
        .collect();
}

fn select_random_word(words: &Vec<String>) -> String {
    let random_index = rand::thread_rng().gen_range(0..words.len());
    return words.get(random_index).expect(&format!("Could not select word at index {}", random_index)).to_string();
}

fn read_first_char_from_console() -> char {
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Invalid input");
    return input.chars().next().expect("Can't get first char from input");
}

fn word_to_letters(word: &str) -> Vec<Letter> {
    return word.chars()
        .map(|char| Letter {
            character: char,
            is_revealed: false,
        }).collect();
}

fn print_letters(letters: &Vec<Letter>) {
    let mut output = String::new();
    output.push_str("Word to find: ");
    for letter in letters {
        output.push(' ');
        if letter.is_revealed {
            output.push(letter.character);
        } else {
            output.push('_');
        }
    }
    println!("{}", output);
}

fn calculate_game_state(remaining_attempts: u8, letters: &Vec<Letter>) -> GameState {
    let mut all_revealed = true;
    for letter in letters {
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