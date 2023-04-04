pub mod setup {
    use project_root::get_project_root;
    use rand::Rng;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    pub struct SecretWord {
        pub word: String,
    }

    impl SecretWord {
        pub fn new() -> SecretWord {
            let mut words_file = get_project_root().expect("Can't file project root.");
            words_file.push("data/words.txt");
    
            let lines = SecretWord::read_lines(words_file);
    
            let word_index = rand::thread_rng().gen_range(0..lines.len());
            SecretWord {
                word: lines[word_index].clone()
            }
        }
    
        fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
            let file = File::open(filename).expect("no such file");
            let buf = BufReader::new(file);
            buf.lines()
                .map(|l| l.expect("Could not parse line"))
                .collect()
        }
    }


}

mod game {

}