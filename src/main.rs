extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io;
use std::io::prelude::*;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    revealed: bool
}

enum GameProgress {
    InProgress,
    Won,
    Lost,
}

fn select_word() -> String {
    //open file
    let mut file = File::open("words.txt").expect("Could not open file.");

    //load file contents
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("An error has occured while reading the file.");

    //get individual words
    let available_words: Vec<&str> = file_contents.trim().split(',').collect();

    //generate random index
    let random_index = rand::thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter> {
    //create empty vector of letters
    let mut letters: Vec<Letter> = Vec::new();

    //wrap each character in a Letter struct
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>) {

    let mut display_string = String::from("Progress: ");

    for letter in letters {
        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

fn read_user_input_character() -> char {

    let mut user_input = String::new();

    //get user input
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        Err(_) => { return '*'; }
    }
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    
    let mut all_revealed = true;

    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    } else if turns_left > 0 {
        return GameProgress::InProgress;
    }

    GameProgress::Lost
}

fn main() {

    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);
    let mut turns_left = ALLOWED_ATTEMPTS;

    println!("Welcome to Hangman! You start with 5 turns and can enter * to quit the game");

    loop {
        println!("\nYou have {} turns left.", turns_left);
        display_progress(&letters);

        println!("\nPlease, enter a letter to guess: ");
        let user_char = read_user_input_character();

        //exit if user enters and asterisk '*'
        if user_char == '*' {
            break;
        }

        //update the 'revealed' state of each letter.
        //if the user has guesses a correct letter, at least one revealed is changed to true
        let mut at_least_one_revealed = false;

        for letter in letters.iter_mut() {
            if letter.character  == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        //if they have guessed incorrectly, they lose one turn
        if !at_least_one_revealed {
            turns_left -= 1;
        }

        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats, you won! The word was {}.", selected_word);
                break;
            }
            GameProgress::Lost => {
                println!("Sorry, you lost!");
                break;
            }
        }
    }

    

}
