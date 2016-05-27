use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

fn calculate(input: String, incr: u32) -> HashMap<char, u32> {
    let mut counts = HashMap::with_capacity(4);

    for ch in input.chars() {
        let counter = counts.entry(ch).or_insert(0);
        *counter += 1;
    }

    return counts;
}

fn main() {
    let stdin = io::stdin();
    let input:String = stdin.lock().lines()
        .map(|line| line.unwrap())
        .skip_while(|line| !line.starts_with(">THREE"))
        .skip(1)
        .take_while(|line| !line.starts_with(">"))
        // .skip_while(|line| !line.starts_with(">THREE"))
        // .take_while(|line| match line { Ok(l) => !l.starts_with(">"), Err(e) => false })
        // .map(|line| line.unwrap())
        .collect::<Vec<_>>()
        .concat()
        .to_uppercase();

    let counts = calculate(input, 1);

    for (ch, count) in &counts {
        println!("{}\t{}", ch, count);
    }
}
