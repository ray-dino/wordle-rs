

use wordle::setup::SecretWord;
use wordle::game::Game;
use wordle::game::Guess;

fn main() {
    run();
}

fn run() {
    let game = Game::new(SecretWord::new());
    for _turn in 0..game.get_turns() {
        let guess = Guess::make_a_guess().expect("Invalid guess");
        dbg!(guess.guess);
    }

}




