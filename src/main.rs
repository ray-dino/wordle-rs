

use wordle::setup::generate_secret_word;
use wordle::game::{Game, TurnResult};
use wordle::interface::input_guess;

fn main() {
    run();
}

fn run() {
    let game = Game::new(generate_secret_word());
    for _turn in 0..game.get_turns() {
        match game.take_a_guess(input_guess().expect("Invalid guess")) {
            TurnResult::Right => {
                println!("You won!");
            }
            TurnResult::Wrong(pattern) => {
                println!("Mistakes: {:?}", pattern);
            }
            TurnResult::Invalid(message) => {
                println!("Something went wrong: {}", message);
            }
        }
        // dbg!(guess);
    }

}




