//! Use release mode
use rayon::prelude::*;
use std::time::Instant;

mod word;
use word::*;

pub fn read_words_to(path: &str, buf: &mut String) -> Result<(), ()>{
    use std::path::Path;
    use std::io::Read;
    use std::fs::File;
    let p = Path::new(path);

    {
        let mut f = match File::open(p) {
            Ok(f) => f,
            Err(_) => return Err(())
        };
        match f.read_to_string(buf) {
            Ok(_) => (),
            Err(_) => return Err(())
        }
    }

    if let Some('\n') = buf.chars().next_back() { buf.pop(); }
    if let Some('\r') = buf.chars().next_back() { buf.pop(); }

    Ok(())
}

const WORD_FILE: &str = "res/wordle-nyt-allowed-guesses.txt";
const WORD_2_FILE: &str = "res/wordle-nyt-answers-alphabetical.txt";

pub fn main() {
    let t_total = Instant::now();

    let mut word_string_buf = String::new();
    println!("Reading words from disk...");
    let t = Instant::now();
    read_words_to(WORD_FILE, &mut word_string_buf).expect("Couldn't read {WORD_FILE}");
    read_words_to(WORD_2_FILE, &mut word_string_buf).expect("Couldn't read {WORD_2_FILE}");
    println!("Done: {:?}\n", t.elapsed());

    println!("Loading words...");
    let t = Instant::now();
    let mut words: Vec<Word> = word_string_buf.par_lines()
        .map(|s| {
            Word::new(s)
        })
        .filter(|s| {
            s.int_repr.count_ones() == 5
        })
        .collect();
    words.dedup_by(|a, b| a.int_repr == b.int_repr);
    words.sort_unstable_by(|a, b| a.int_repr.cmp(&b.int_repr));
    let words_len = words.len();
    println!("Loaded {} words in {:?}.\n", words_len, t.elapsed());
    
    let next_word: Vec<Vec<usize>> = words
        .par_iter()
        .enumerate()
        .map(|(i, w)| {
            let mut res = vec![];
            for j in (i + 1)..words_len {
                if w.int_repr & words[j].int_repr == 0 {
                    res.push(j);
                }
            }

            res
        })
        .collect();

    println!("Total: {:?}", t_total.elapsed());
}
