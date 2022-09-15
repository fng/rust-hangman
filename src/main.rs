mod hangman;

use hangman::HangmanGame;

fn main() {
    // let mut game = HangmanGame::new_random_from_file("words.txt", 5);
    let mut game = HangmanGame::new_random_from_webservice(10);
    game.start();


}




