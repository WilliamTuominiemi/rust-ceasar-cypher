use std::io;

fn main() {
    let alphabet: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    println!("Cypher or Decypher? Type 1 or 2, anything else to quit");
    let mut method = String::new();
    match io::stdin().read_line(&mut method) {
        Ok(_) => {
            let method = method.trim();
            if method == "1" {
                println!("Enter word to cypher: ");
                let mut word: String = String::new();

                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => word = input.to_ascii_uppercase().split_whitespace().collect(),
                    Err(error) => println!("error: {error}"),
                }

                let cyphered_word: String = cypher(alphabet, &word);

                println!("{}", cyphered_word);
            } else if method == "2" {
                println!("Enter word to decypher: ");
                let mut cyphered_word: String = String::new();

                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        cyphered_word = input.to_ascii_uppercase().split_whitespace().collect()
                    }
                    Err(error) => println!("error: {error}"),
                }

                let decyphered_word: String = decypher(alphabet, &cyphered_word);

                println!("{}", decyphered_word);
            }
        }
        Err(error) => println!("error: {error}"),
    }
}

fn cypher(alphabet: [char; 26], word: &str) -> String {
    let mut word_indexes: Vec<usize> = Vec::new();

    for letter in word.chars() {
        for (index, character) in alphabet.into_iter().enumerate() {
            if letter == character {
                word_indexes.push(index);
            }
        }
    }

    let mut cyphered_word_indexes: Vec<usize> = Vec::new();

    for word_index in word_indexes {
        cyphered_word_indexes.push(word_index + 1);
    }

    let mut cyphered_word: String = String::new();

    for word_index in cyphered_word_indexes {
        for (index, character) in alphabet.into_iter().enumerate() {
            if word_index == index {
                cyphered_word.push(character);
            }
        }
    }

    cyphered_word
}

fn decypher(alphabet: [char; 26], word: &str) -> String {
    let mut word_indexes: Vec<usize> = Vec::new();

    for letter in word.chars() {
        for (index, character) in alphabet.into_iter().enumerate() {
            if letter == character {
                word_indexes.push(index);
            }
        }
    }

    let mut decyphered_word_indexes: Vec<usize> = Vec::new();

    for word_index in word_indexes {
        decyphered_word_indexes.push(word_index - 1);
    }

    let mut decyphered_word: String = String::new();

    for word_index in decyphered_word_indexes {
        for (index, character) in alphabet.into_iter().enumerate() {
            if word_index == index {
                decyphered_word.push(character);
            }
        }
    }

    decyphered_word
}
