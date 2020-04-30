extern crate rand;
use rand::Rng;

mod user_input;

use std::fs::File;
use std::io::prelude::*;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    revealed : bool
}

enum GameProgress {
    OnGoing,
    WON,
    LOST
}

fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);

    //Main game loop
    loop {
        println!("\nYou have {} turns left!", turns_left);
        display_progress(&letters);

        println!("\nPlease enter a character");
        let user_char = user_input::read_char();
        let mut one_revealed  = false;

        match user_char {
            '*' => return,

            _   => {
                for letter in letters.iter_mut() {
                    if letter.character == user_char {
                        letter.revealed = true;
                        one_revealed    = true;
                    }
                }

                if !one_revealed {
                    turns_left -= 1;
                }
            }
        }

        match check_progress(turns_left, &letters) {
            GameProgress::OnGoing => continue,
            GameProgress::WON     => {
                println!("\nCongratulations You won!! The word was {}", selected_word);
                break;
            },
            GameProgress::LOST   => {
                println!("\nSorry you lost!!");
                break;
            }
        }
    }
}

fn select_word() -> String {
    let mut file_pointer = File::open("words.txt").expect("Couldnt open file");

    let mut file_content = String::new();

    file_pointer.read_to_string(&mut file_content)
                .expect("Couldnt read file");

    let words_in_file: Vec<&str> = file_content.trim().split(",").collect();

    let random_index = rand::thread_rng().gen_range(0, words_in_file.len());

    return String::from(words_in_file[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed : false
        });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
            continue;
        }

        display_string.push('_');
    }

    println!("{}", display_string);
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    let mut all_reavealed = true;

    for letter in letters {
        if !letter.revealed {
            all_reavealed = false;
        }
    }

    if all_reavealed {
        return GameProgress::WON;
    }

    if turns_left > 0 {
        return GameProgress::OnGoing;
    }

    return GameProgress::LOST;
}
