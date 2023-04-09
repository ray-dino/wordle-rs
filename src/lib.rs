pub mod setup {
    use project_root::get_project_root;
    use rand::Rng;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;



    pub fn generate_secret_word() -> String {
        const WORDS_PATH : &str = "data/words.txt";
        let project_root = get_project_root().expect("Can't find project root.");
        let words_file = project_root.join(WORDS_PATH);

        let lines = read_lines(words_file);

        let word_index = rand::thread_rng().gen_range(0..lines.len());
        lines[word_index].clone()
    }

    fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
        // TODO: Return Result instead. Handle error in caller.
        let file = File::open(filename).expect("No such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }



}

pub mod game {
    const TURNS: u8 = 5;

    pub struct Game {
        secret_word: String,
        turns: u8
    }

    impl Game {
        pub fn new(secret_word: String) -> Self {
            dbg!(&secret_word);
            Self {
                secret_word: secret_word,
                turns: TURNS
            }
        }

        pub fn take_a_guess(&self, guess: String) -> Result<String, String>  {
            dbg!(&guess);
            // validate guess
            if Game::validate_guess(&guess) {
                println!("Ok!");
                return Ok(String::from("Ok"))
            }
            Err(String::from("Message"))
        }

        pub fn get_turns(&self) -> u8 {
            self.turns
        }

        fn validate_guess(guess: &String) -> bool {
            guess.trim().chars().all(|c|c.is_ascii_lowercase()) 
            && guess.trim().len() == 5
        }
        
    }
}

pub mod interface {
    use std::io;

    pub fn input_guess() -> io::Result<String> {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;
        Ok(guess)
    }
}