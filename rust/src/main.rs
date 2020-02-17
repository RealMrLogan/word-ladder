use std::io;
use std::io::Write; // bring flush() into scope
use std::collections::HashMap;
use std::fs;
use std::convert::TryInto;

fn main() {
    println!("Welcome to the Word Ladder!\n");
    let first_word = get_word_from_user("Please enter the first word: ");
    let last_word = get_word_from_user("Please enter the last word: ");
    if first_word.chars().count() != last_word.chars().count() {
        println!("The two words are NOT the same length!");
    }

    let lexicon = fs::read_to_string("../lexicon.txt").expect("Unable to read file");
    let lexicon_map = put_lexicon_into_hashmap(lexicon); // would this be inefficient? I could just iterate through the string once?
    // TODO: pick up here.
}

fn get_word_from_user(message: &str) -> std::string::String {
    print!("{}", message);
    io::stdout().flush().unwrap(); // flushes stdout so that the message appears

    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    return word.trim().to_string();
}

fn put_lexicon_into_hashmap(lexicon: String) -> HashMap<i32, Vec<String>> {
    let mut lexicon_map: HashMap<i32, Vec<String>> = HashMap::new();
    let split: Vec<&str> = lexicon.split("\n").collect();
    for word in split {
        let num_chars: i32 = word.chars().count().try_into().unwrap();
        // if the key (entry) is there, push onto the vector, if it's not there, create an new vector and push
        lexicon_map.entry(num_chars).or_insert(Vec::new()).push(word.to_string());
    }
    return lexicon_map;
}

// *get the start word;
// *get end word;
// *make sure both words are the same length
// *get num chars of word
// *read in lexicon into a map wit hthe num chars as a key and array of words as value
// start ladder; create new queue (stack?)
// write function to determine if there is a one word difference between two words
// if there is only one word difference, add that word to the stack
// copy stack
// put that stack into a vector
// continue for next word until we reach the end of the lexicon
// start over with each ladder in vector until the end word is found in one stack
// we found the shortest ladder
// display ladder to user