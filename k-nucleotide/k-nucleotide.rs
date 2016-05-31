#![feature(str_char)]

#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use std::hash::{Hash, Hasher};

use std::cmp::Ordering;

use std::fmt;

lazy_static! {
    static ref TONUM: [u8; 256] = {
        let mut m: [u8; 256] = [0; 256];
        m['A' as usize] = 0;
        m['C' as usize] = 1;
        m['T' as usize] = 2;
        m['G' as usize] = 3;
        m
    };
}
static TOCHAR: [char; 4] = ['A', 'C', 'T', 'G'];


#[derive(Copy, Clone)]
struct T {
    data: u64,
    size: usize,
}

impl T {
    fn blank() -> T {
        T::new("")
    }

    fn new(s: &str) -> T {
        let mut t = T {
            data: 0,
            size: s.len()
        };
        t.reset(s, 0, s.len());
        t
    }

    fn reset(&mut self, s: &str, begin: usize, end: usize) -> &T {
        self.data = 0;
        self.size = end - begin;
        for i in 0..self.size {
            self.data <<= 2;
            self.data |= TONUM[s.char_at(i) as usize] as u64;
        }
        self
    }
}

impl PartialEq for T {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl Eq for T {}

impl PartialOrd for T {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl Ord for T {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl Hash for T {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::from("");
        let mut tmp = self.data;
        for _ in 0..self.size {
            out.push(TOCHAR[(tmp & 3) as usize]);
            tmp >>= 2;
        }
        out = out.chars().rev().collect();
        write!(f, "{}", out)
    }
}




fn calculate(input: String, tsize: usize, begin: usize, incr: u32) -> HashMap<T, u32> {
    let mut counts = HashMap::with_capacity(4);

    let mut tmp = T::blank();
    for i in begin..(input.len() + 1 - tsize) {
        tmp.reset(&input[i..(i+tsize)], 0, 1);
        let counter = counts.entry(tmp).or_insert(0);
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

    let counts = calculate(input, 1, 0, 1);

    for (ch, count) in &counts {
        println!("{}\t{}", ch, count);
    }
}
