use std::collections::HashMap;
use std::io;

fn main() {
    println!("Enter some text:");
    let mut sample = String::new();
    io::stdin().read_line(&mut sample).expect("Failed to read text");
    let trimmed = sample.trim();

    let mut count = HashMap::new();
    for word in trimmed.split_whitespace() {
        let word = word.to_lowercase();
        *count.entry(word).or_insert(0) += 1;
    }

    for (key,value) in &count {
        println!("{key} appears {value} times");
    }

    // println!("Word counts: {count}");
}

