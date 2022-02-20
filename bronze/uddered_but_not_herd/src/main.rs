use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    // hashmap of indices
    let mut alphabet: HashMap<char, usize> = HashMap::new();
    let mut first_line = String::new();
    stdin_lock.read_line(&mut first_line).unwrap();
    for (i, c) in first_line.chars().enumerate() {
        alphabet.insert(c, i);
    }
    let mut second_line = String::new();
    stdin_lock.read_line(&mut second_line).unwrap();
    let mut result = 0;
    let mut index = usize::MAX;
    for c in second_line.chars() {
        if index >= alphabet[&c] {
            result += 1;
        }
        index = alphabet[&c];
    }
    println!("{}", result);
}
