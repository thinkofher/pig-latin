const VOWELS: &str = "aeiouy";
const ILLEGAL_CHARS: &str = ":;.,?!";
const VOWELS_ADDITION: &str = "hay";
const CONSONANT_ADDITION: &str = "ay";

enum Letter {
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

fn get_rid_of_chars(mut s: String, chars_slice: &str) -> String {
    for ch in chars_slice.chars() {
        s.retain(|c| c != ch)
    }
    s
}

fn get_rid_of_illegal_chars(s: String) -> String {
    get_rid_of_chars(s, ILLEGAL_CHARS)
}

pub fn transcript(text: String) -> String {
    let text_vev: Vec<String> = text
        .split_whitespace()
        .map(|s| s.to_string())
        .map(|s| get_rid_of_illegal_chars(s))
        .map(|s| Word::from(&s[..]).to_pig_latin())
        .collect();
    text_vev.join(" ")
}

struct Word(String);

impl Word {
    fn from(s: &str) -> Word {
        Word(String::from(s.to_lowercase()))
    }

    fn to_pig_latin(&self) -> String {
        match self.starts_with() {
            Letter::Consonant(c) => format!("{}{}{}", self.rest(), c, CONSONANT_ADDITION),
            Letter::Vowel(_) => format!("{}{}", self.0, VOWELS_ADDITION),
        }
    }

    fn first_letter(&self) -> char {
        self.0.chars().next().unwrap()
    }

    fn rest(&self) -> String {
        self.0.chars().skip(1).collect()
    }

    fn starts_with(&self) -> Letter {
        Letter::new(self.first_letter())
    }
}
