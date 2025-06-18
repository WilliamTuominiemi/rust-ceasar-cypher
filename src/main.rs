fn main() {
    let alphabet: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let word: &'static str = "HELLO";

    println!("{}", word);

    let cyphered_word: String = cypher(alphabet, word);

    println!("{}", cyphered_word)
}

fn cypher(alphabet: [char; 26], word: &'static str) -> String {
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
