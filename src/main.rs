

use wordle::setup::generate_secret_word;
use wordle::game::Game;
// use wordle::game::Guess;
use wordle::interface::input_guess;

fn main() {
    run();
}

fn run() {
    let game = Game::new(generate_secret_word());
    for _turn in 0..game.get_turns() {
        if let Ok(x) = game.take_a_guess(input_guess().expect("Invalid guess")) {
            dbg!(x);
        }
        // dbg!(guess);
    }

}




