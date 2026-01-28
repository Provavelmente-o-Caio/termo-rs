use colored::Colorize;
use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn select_word_size(words: Vec<String>, size: u32) -> Vec<String> {
    words
        .into_iter()
        .filter(|word| word.len() == size.try_into().unwrap())
        .collect()
}

fn select_random_word(words: &Vec<String>) -> String {
    let word = &words[rand::rng().random_range(0..words.len())];
    word.to_string()
}

fn main() {
    let word_length = 5;
    let mut words = read_lines("lexico.txt").unwrap();
    words = select_word_size(words, word_length);
    let word = select_random_word(&words);
    let mut revealed_word = "_".repeat(word_length.try_into().unwrap());
    let mut colored = revealed_word.clone();

    let mut guesses = 0;

    // Leitura de palavra
    loop {
        println!("Palavra revelada até agora:");
        println!("{}", colored);
        println!("Escreva seu chute: ");

        let mut guess = String::new();
        loop {
            io::stdin()
                .read_line(&mut guess)
                .expect("Unable to read line");

            guess.truncate(guess.trim_end().len());
            guess = guess.to_lowercase();
            let guess_length = guess.len();

            if guess_length != word_length.try_into().unwrap() {
                println!("Seu chute não é do mesmo tamanho da palavra");
                println!("Current size: {guess_length}, expected word size: {word_length}");
                guess.clear();
            } else if !words.contains(&guess) {
                println!("{guess} não está entre as palavras permitidas");
                guess.clear();
            } else {
                break;
            }
        }

        // Lógica de verificação de acertos
        guesses += 1;

        let guess_chars: Vec<char> = guess.chars().collect();
        let word_chars: Vec<char> = word.chars().collect();
        let mut revealed_word_chars: Vec<char> = revealed_word.chars().collect();

        let mut used = vec![false; word_chars.len()];

        // verdes
        for i in 0..word_chars.len() {
            if guess_chars[i] == word_chars[i] {
                revealed_word_chars[i] = guess_chars[i];
                used[i] = true;
            }
        }

        // montar visual
        colored.clear();

        for i in 0..word_chars.len() {
            if guess_chars[i] == word_chars[i] {
                colored.push_str(&guess_chars[i].to_string().green().bold().to_string());
            } else if let Some(j) = word_chars
                .iter()
                .enumerate()
                .find(|(j, c)| !used[*j] && **c == guess_chars[i])
                .map(|(j, _)| j)
            {
                used[j] = true;
                colored.push_str(&guess_chars[i].to_string().yellow().bold().to_string());
            } else {
                colored.push_str(&guess_chars[i].to_string().bright_black().to_string());
            }
        }

        revealed_word = revealed_word_chars.into_iter().collect();

        println!();

        if guesses >= word_length {
            println!("A palavra era {word}, você perdeu D:");
            break;
        } else if guess == word {
            println!("Ganhou :D");
            break;
        }
    }
}
