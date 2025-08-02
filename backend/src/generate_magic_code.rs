use std::fs;

const WORDLIST_PATH: &str = "eff_short_wordlist_1.txt";

pub async fn generate_code() {
    let wordlist = fs::read_to_string(WORDLIST_PATH).expect("Should have read the wordlist");
    println!("{wordlist}");
}
