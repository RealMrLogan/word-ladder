use std::collections::HashMap;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::fs;
use std::io;
use std::io::Write; // bring flush() into scope

fn main() {
    println!("Welcome to the Word Ladder!\n");
    let first_word = get_word_from_user("Please enter the first word: ");
    let last_word = get_word_from_user("Please enter the last word: ");
    if first_word.chars().count() != last_word.chars().count() {
        println!("The two words are NOT the same length!");
    }

    let lexicon = fs::read_to_string("../lexicon.txt").expect("Unable to read file");
    let lexicon_map = put_lexicon_into_hashmap(lexicon); // would this be inefficient? I could just iterate through the string once?

    let word_ladder = find_word_ladder(lexicon_map, first_word, last_word);
    println!("Word Ladder: {:?}", word_ladder);
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
        lexicon_map
            .entry(num_chars)
            .or_insert(Vec::new())
            .push(word.to_string());
    }
    return lexicon_map;
}

fn find_word_ladder(
    lexicon_map: HashMap<i32, Vec<String>>,
    first_word: String,
    last_word: String,
) -> Vec<String> {
    let key: i32 = first_word.chars().count().try_into().unwrap();
    // returns an array of all the words that are the same length
    let lexicon_value = lexicon_map.get(&key).unwrap();

    let mut word_ladders: VecDeque<Vec<String>> = VecDeque::new();
    // add the first vector
    let mut first_vec: Vec<String> = Vec::new();
    first_vec.push(first_word);
    word_ladders.push_back(first_vec);
    let mut found_last_word = false;
    // we want to add one or none words to each ladder for each loop in the VecDeque
    while !found_last_word {
        for word_ladder in &mut word_ladders {
            // this has something to do with borrowing... I don't quite understand it yet
            let temp = &word_ladder.clone();
            let last_ladder_word = temp.last().unwrap();

            for word in lexicon_value {
                if is_one_letter_different(&last_ladder_word, &word) && !is_already_in_ladder(word_ladder.clone(), &word) {
                    &word_ladder.push(word.to_string());
                    if word == &last_word {
                        // we found the end!
                        found_last_word = true;
                    }
                    let word_ladder_copy = word_ladder.clone();
                    // word_ladders.push_back(word_ladder_copy);
                    // TODO: add more Vecs to the VecDeque
                    // TODO: what to do if there is no word ladder
                    break;
                }
            }
        }
    }

    return find_shortest_word_ladder(word_ladders);
}

/**
 * Assumes both parameters are the same number of characters
 */
fn is_one_letter_different(word: &str, comparer: &str) -> bool {
    // if they are the same word, they are not one letter different
    if word == comparer {
        return false;
    }

    let mut characters_different: i32 = 0;
    for (i, c) in word.chars().enumerate() {
        let comparer_char = comparer.chars().nth(i).unwrap();
        if c != comparer_char {
            characters_different += 1;
            if characters_different > 1 {
                return false;
            }
        }
    }
    return true;
}

fn is_already_in_ladder(word_ladder: Vec<String>, word_to_find: &str) -> bool {
    for word in word_ladder {
        if word == word_to_find {
            return true;
        }
    }
    return false;
}

fn find_shortest_word_ladder(mut word_ladders: VecDeque<Vec<String>>) -> Vec<String> {
    println!("VecDeque: {:?}", word_ladders);
    let mut shortest_ladder = word_ladders.pop_front().unwrap();
    for word_ladder in word_ladders {
        println!("Comparing {} to {}", word_ladder.len(), shortest_ladder.len());
        if word_ladder.len() < shortest_ladder.len() {
            shortest_ladder = word_ladder;
        }
    }
    return shortest_ladder;
}

// *get the start word;
// *get end word;
// *make sure both words are the same length
// *get num chars of word
// *read in lexicon into a map wit hthe num chars as a key and array of words as value
// *start ladder; create new queue (stack?)
// *write function to determine if there is a one letter difference between two words
// *if there is only one letter difference, add that word to the stack
// *copy stack
// put that stack into a vector
// continue for next word until we reach the end of the lexicon
// start over with each ladder in vector until the end word is found in one stack
// we found the shortest ladder
// display ladder to user
