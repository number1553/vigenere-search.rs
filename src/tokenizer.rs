use lazy_static::lazy_static;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use crate::token::Token;

pub fn tokenize(text: &str) -> Vec<Token> {
    lazy_static! {
        static ref STEMMER: Stemmer = Stemmer::create(Algorithm::English);
        static ref WORDS_REGEX: Regex = Regex::new(r"([\w']*)").unwrap();
    }

    WORDS_REGEX.captures_iter(text)
        .filter(|x| x.get(1).is_some() && !x.get(1).unwrap().as_str().is_empty())
        .map(|x| {
            Token {
                value: STEMMER.stem(x.get(1).unwrap().as_str())
                    .to_lowercase()
                    .trim()
                    .to_string()
            }
        }).collect()
}