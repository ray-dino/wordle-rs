pub mod setup {
    use project_root::get_project_root;
    use rand::Rng;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    pub struct SecretWord {
        word: String,
    }

    impl SecretWord {
        pub fn new() -> Self {
            const WORDS_PATH : &str = "data/words.txt";
            let project_root = get_project_root().expect("Can't find project root.");
            let words_file = project_root.join(WORDS_PATH);
    
            let lines = SecretWord::read_lines(words_file);
    
            let word_index = rand::thread_rng().gen_range(0..lines.len());
            Self {
                word: lines[word_index].clone()
            }
        }
    
        fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
            // TODO: Return Result instead. Handle error in caller.
            let file = File::open(filename).expect("No such file");
            let buf = BufReader::new(file);
            buf.lines()
                .map(|l| l.expect("Could not parse line"))
                .collect()
        }

        pub fn word(&self) -> &String {
            &self.word
        }
    }


}

pub mod game {
    use std::io;

    use crate::setup::SecretWord;

    const TURNS: u8 = 5;

    pub struct Guess {
        pub guess: String
    }

    impl Guess {
        pub fn make_a_guess() -> io::Result<Self> {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess)?;

            Ok(Guess { guess: guess })
        }
    }

    pub struct Game {
        secret_word: SecretWord,
        turns: u8
    }

    impl Game {
        pub fn new(secret_word: SecretWord) -> Self {
            dbg!(secret_word.word());
            Self {
                secret_word: secret_word,
                turns: TURNS
            }
        }

        fn take_a_guess() {

        }

        pub fn get_turns(&self) -> u8 {
            self.turns
        }
        
    }
}