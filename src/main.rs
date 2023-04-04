use wordle::setup::SecretWord;

fn main() {
    run();
}

fn run() {
    let secret_word = SecretWord::new();
    println!("{}", secret_word.word);
}


