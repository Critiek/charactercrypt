use std::fs;

const WORDLIST_PATH: &str = "eff_short_wordlist_1.txt";

pub async fn generate_code(wordlist: &Vec<String>) {
    for word in wordlist {
        println!("{word}")
    }
}

pub fn read_wordlist() -> Vec<String> {
    let wordlist_raw = fs::read_to_string(WORDLIST_PATH)
        .expect("Should have read the wordlist");
    wordlist_raw.split('\n').map(|x| x.to_string()).collect()
}