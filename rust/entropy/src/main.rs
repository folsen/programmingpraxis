// The Shannon entropy of a file is a measure of the information content in
// the file; higher entropy implies more information. Shannon entropy is computed
// as H = -1 * sum(pi * log2(pi)) where pi is the frequency of each symbol i
// in the input (frequency is the percentage of the total number of symbols).
// Shannon entropy is just one facet of the study of information theory, which is
// fundamental to computer science.

// Your task is to compute the Shannon entropy of a file.

#![feature(iter_arith)]

#![feature(test)]
extern crate test;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::hash::Hash;
use std::collections::HashMap;

fn main() {
    match env::args().nth(1) {
        Some(path) => println!("Entropy of file `{}`: {:.4}",
                               path.clone(),
                               calc_file(path).unwrap()),

        None => println!("Usage: entropy <path-to-file>")
    }
}

pub fn calc_file(path: String) -> Result<f64, io::Error> {
    let f = try!(File::open(path));
    return Ok(entropy(f.bytes().map(|c| c.unwrap())))
}

pub fn entropy<T: Hash + Eq, I: Iterator<Item=T>>(xs: I) -> f64 {
    let mut map = HashMap::new();
    let mut total: i64 = 0;
    for x in xs {
        let counter = map.entry(x).or_insert(0);
        *counter += 1;
        total += 1;
    }
    // Inefficient?
    let entropies = map.values().map(|v| entropize(*v, total));
    let sum: f64 = entropies.sum();
    return -1.0 * sum
}

fn entropize(x: i64, total: i64) -> f64 {
    let y: f64 = (x as f64)/(total as f64);
    return y * y.log2()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_ints() {
        let test1 = vec![1,2,1,2,1,2];
        assert_eq!(entropy(test1.into_iter()), 1.0);
    }

    #[test]
    fn test_string1() {
        let test2 = "banana";
        assert_eq!(format!("{:.3}", entropy(test2.chars())), "1.459");
    }
    #[test]
    fn test_string2() {
        let test3 = "nanana";
        assert_eq!(format!("{:.3}", entropy(test3.chars())), "1.000");
    }

    #[bench]
    fn bench_file(b: &mut Bencher) {
        b.iter(|| calc_file("./war_and_peace.txt".to_string()))
    }
}
