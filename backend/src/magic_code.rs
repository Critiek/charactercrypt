use axum::{extract::State, response::IntoResponse};
use charactercrypt::AppState;
use rand::{self, rngs::StdRng, Rng, SeedableRng};
use std::fs;

const WORDLIST_PATH: &str = "eff_short_wordlist_1.txt";

pub async fn handle_code_submissions() {

}

pub async fn generate_code(State(app_state): State<AppState>) -> impl IntoResponse {
    let mut rng = StdRng::from_os_rng();

    let mut magic_code_words: Vec<String> = Vec::new();
    let magic_code_length: u8 = rng.random_range(4..=6);

    for _ in 0..magic_code_length {
        magic_code_words
            .push(app_state.wordlist[rng.random_range(0..app_state.wordlist.len())].clone());
    }

    println!("{}", magic_code_words.join("-"));

    axum::http::status::StatusCode::OK
}

pub fn read_wordlist() -> Vec<String> {
    let wordlist_raw = fs::read_to_string(WORDLIST_PATH).expect("Should have read the wordlist");
    wordlist_raw
        .split('\n')
        .map(|x| x.trim().to_string())
        .collect()
}
