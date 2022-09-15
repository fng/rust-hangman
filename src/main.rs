mod hangman;

use std::env;
use hangman::HangmanGame;

fn main() {
    match env::args().nth(1).as_deref() {
        Some("file") =>
            HangmanGame::new_random_from_file("words.txt", 5).start(),
        Some("webservice") =>
            HangmanGame::new_random_from_webservice(10).start(),
        _ =>
            HangmanGame::new_random_from_voci_file("voci.txt", 10).start()
    }
}

