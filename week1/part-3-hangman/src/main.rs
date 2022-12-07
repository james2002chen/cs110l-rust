// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();

    // Note: given what you know about Rust so far, it's easier to pull characters out of a vector than it is to pull them out of a string. You can get the ith character of secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();

    //println!("random word: {}", secret_word);
    let mut mask = vec![false; secret_word_chars.len()];
    let mut num = 0;
    loop {

        print!("Please guess a letter: ");

        // Make sure the prompt from the previous line gets displayed:
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        //print!("{}",guess);

        let guess:char = guess.chars().next().unwrap();
        if !guess.is_ascii_lowercase() { break; }

        let mut newly_guessed = false;
        /*
        while i < secret_word_chars.len() {
            if ch == Some(secret_word_chars[i]) {
                if mask[i] == false {
                    newly_guessed = true;
                    mask[i] = true;
                }
            }
            i += 1;
        }
        */

        for (i, &item) in secret_word_chars.iter().enumerate() {
            if item == guess {
                if !mask[i] { 
                    newly_guessed = true;
                    mask[i] = true;
                }
            }
        }
        if !newly_guessed {
            num += 1;
        }
        if num >= NUM_INCORRECT_GUESSES {
            println!("YOU FAILED");
            break;
        }
        let mut all_guessed = true;
        for (i, &item) in secret_word_chars.iter().enumerate() {
            if mask[i] {
                print!("{}",item);
            }
            else {
                all_guessed = false;
                print!("_");
            }
        }
        /*
        while i < secret_word_chars.len() {
            if mask[i] {
                print!("{}",secret_word_chars[i]);
            }
            else {
                all_guessed = false;
                print!("_");
            }
            i += 1;
        }
        */
        print!("\n");
        if all_guessed {
            println!("YOU WIN");
            break;
        } else {
            println!("chances left:{}",NUM_INCORRECT_GUESSES-num);
        }
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
    }

}
