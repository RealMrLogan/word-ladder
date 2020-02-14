use std::io;
use std::io::Write; // bring flush() into scope

fn main() {
    println!("Welcome to the Word Ladder!\n");
    let first_word = get_word_from_user("Please enter the first word: ");
    let last_word = get_word_from_user("Please enter the last word: ");
    // this measures bytes, not number of characters?
    if first_word.chars().count() == last_word.chars().count() {
        println!("The two words are the same!");
    }
}

fn get_word_from_user(message: &str) -> std::string::String {
    print!("{}", message);
    io::stdout().flush().unwrap(); // flushes stdout so that the message appears

    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    return word;
}

// get the start word;
// get end word;
// make sure both words are the same length
// get length of word
// read in lexicon
// keep only words that are the same length; would this be going through the lexicon twice?
// start ladder; create new queue (stack?)
// write function to determine if there is a one word difference between two words
// if there is only one word difference, add that word to the stack
// copy stack
// put that stack into a vector
// continue for next word until we reach the end of the lexicon
// start over with each ladder in vector until the end word is found in one stack
// we found the shortest ladder
// display ladder to user