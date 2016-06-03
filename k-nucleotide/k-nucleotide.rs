#![feature(step_by)]

#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use std::hash::{Hash, Hasher};

use std::cmp::Ordering;

use std::fmt;

use std::sync::{Arc, Mutex};
use std::thread;

use std::mem;

use std::str;

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


#[derive(Copy, Clone, Eq)]
struct T {
    data: u64,
    size: usize,
}

impl T {
    fn blank() -> T {
        T::new(b"")
    }

    fn new(s: &[u8]) -> T {
        let mut t = T {
            data: 0,
            size: s.len(),
        };
        t.reset(s);
        t
    }

    fn reset(&mut self, s: &[u8]) -> &T {
        self.data = 0;
        self.size = s.len();
        for c in s {
            self.data <<= 2;
            self.data |= TONUM[*c as usize] as u64;
        }
        self
    }
}

impl PartialEq for T {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

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


fn calculate(input: &[u8], tsize: usize, begin: usize, incr: usize) -> HashMap<T, u32> {
    let mut counts = HashMap::new();

    let mut tmp = T::blank();
    for i in (begin..(input.len() + 1 - tsize)).step_by(incr) {
        tmp.reset(&input[i..(i + tsize)]);
        let counter = counts.entry(tmp).or_insert(0);
        *counter += 1;
    }

    counts
}


fn parallel_calculate(input: &[u8], tsize: usize) -> HashMap<T, u32> {
    let num_cpus = 4;
    let mut children = vec![];

    let combined_counts = Arc::new(Mutex::new(HashMap::new()));
    let input: &'static [u8] = unsafe { mem::transmute(input) };

    for n in 0..num_cpus {
        let combined_counts = combined_counts.clone();
        children.push(thread::spawn(move || {
            let counts = calculate(&input, tsize, n, num_cpus);
            let mut combined_counts = combined_counts.lock().unwrap();
            for (t, count) in &counts {
                let counter = combined_counts.entry(*t).or_insert(0);
                *counter += *count;
            }
        }));
    }

    for child in children {
        child.join().unwrap();
    }

    Arc::try_unwrap(combined_counts).ok().expect("foobar").into_inner().unwrap()
}


fn write_frequencies(input: &[u8], tsize: usize) {
    let sum: usize = input.len() + 1 - tsize;
    let counts = parallel_calculate(input, tsize);

    let mut counts_descending: Vec<(&T, &u32)> = counts.iter().collect();
    counts_descending.sort_by(|a, b| a.1.cmp(b.1).reverse());

    for (ch, count) in counts_descending {
        let frequency: f64 = if sum != 0 {
            (100 * count) as f64 / sum as f64
        } else {
            0.0
        };
        println!("{}\t{:.3}", ch, frequency);
    }
    println!("");
}


fn write_count(input: &[u8], tstr: &[u8]) {
    let size = tstr.len();
    let counts = parallel_calculate(input, size);

    let count = counts.get(&T::new(tstr)).cloned().unwrap_or(0);
    println!("{}\t{}", count, str::from_utf8(tstr).unwrap());
}


fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .map(|line| line.unwrap())
        .skip_while(|line| !line.starts_with(">THREE"))
        .skip(1)
        .take_while(|line| !line.starts_with('>'))
        .collect::<Vec<_>>()
        .concat()
        .to_uppercase();
    let input_bytes = input.as_bytes();

    for i in 1..3 {
        write_frequencies(&input_bytes, i);
    }

    let sequences: Vec<&[u8]> = vec![b"GGT", b"GGTA", b"GGTATT", b"GGTATTTTAATT", b"GGTATTTTAATTTATAGT"];
    for t in sequences.into_iter() {
        write_count(&input_bytes, t);
    }
}
