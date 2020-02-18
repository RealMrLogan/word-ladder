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
    // TODO: what if the two words are the same
    if first_word.chars().count() != last_word.chars().count() {
        println!("The two words are NOT the same length!");
        return;
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

    // we want to add one or none words to each ladder for each loop in the VecDeque
    let mut found_last_word = false;
    while !found_last_word {
        for i in 0..word_ladders.len() {
            // this has something to do with borrowing... I don't quite understand it yet
            let temp = word_ladders[i].clone();
            let last_ladder_word = temp.last().unwrap();

            for word in lexicon_value {
                if is_one_letter_different(&last_ladder_word, &word) && !is_already_in_ladder(word_ladders[i].clone(), &word) {
                    let mut temp = word_ladders[i].clone();
                    temp.push(word.to_string());
                    word_ladders.push_back(temp);
                    if word == &last_word {
                        // we found the end!
                        found_last_word = true;
                        break;
                    }
                    // TODO: what to do if there is no word ladder
                }
            }
        }
    }

    return find_shortest_word_ladder(word_ladders, &last_word);
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

fn find_shortest_word_ladder(word_ladders: VecDeque<Vec<String>>, last_word: &str) -> Vec<String> {
    let mut shortest_ladder: Vec<String> = Vec::new();
    shortest_ladder.resize(100, "".to_string()); // arbitrary placeholder
    for word_ladder in word_ladders {
        if word_ladder.len() < shortest_ladder.len() && word_ladder.last().unwrap() == last_word {
            shortest_ladder = word_ladder;
        }
    }
    return shortest_ladder;
}