const VOWELS: &str = "aeiouy";
const ILLEGAL_CHARS: &str = ":;.,?!";
const VOWELS_ADDITION: &str = "hay";
const CONSONANT_ADDITION: &str = "ay";

pub enum Letter {
    Consonant(char),
    Vowel(char),
}

impl Letter {
    pub fn new(c: char) -> Letter {
        if is_vowel(c) {
            Letter::Vowel(c)
        } else {
            Letter::Consonant(c)
        }
    }
}

fn is_vowel(c: char) -> bool {
    VOWELS.contains(c.to_lowercase().next().unwrap())
}

fn filter_chars(s: String, undesirable: &str) -> String {
    s.chars().filter(|&c| !undesirable.contains(c)).collect()
}

fn filter_illegal_chars(s: String) -> String {
    filter_chars(s, ILLEGAL_CHARS)
}

pub fn transcript(text: String) -> String {
    text.split_whitespace()
        .map(|s| s.to_string())
        .map(|s| filter_illegal_chars(s))
        .map(|s| s.pig_latin())
        .collect::<Vec<String>>()
        .join(" ")
}

pub trait PigLatin {
    fn first_letter(&self) -> char;

    fn rest(&self) -> String;

    fn starts_with(&self) -> Letter {
        Letter::new(self.first_letter())
    }

    fn pig_latin(&self) -> String {
        match self.starts_with() {
            Letter::Consonant(c) => format!("{}-{}{}", self.rest(), c, CONSONANT_ADDITION),
            Letter::Vowel(_) => {
                format!("{}{}-{}", self.first_letter(), self.rest(), VOWELS_ADDITION)
            }
        }
    }
}

impl PigLatin for String {
    fn first_letter(&self) -> char {
        self.to_lowercase().chars().next().unwrap()
    }

    fn rest(&self) -> String {
        self.to_lowercase().chars().skip(1).collect()
    }
}
