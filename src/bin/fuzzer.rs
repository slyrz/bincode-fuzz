extern crate bincode_fuzz;

use std::env;

use bincode_fuzz::corpus::Corpus;
use bincode_fuzz::fuzzer::Fuzzer;

fn main() {
    let path = env::args().skip(1).next().unwrap();
    let corpus = Corpus::open_or_new(path).unwrap();
    let fuzzer = Fuzzer::new(corpus);
    fuzzer.run().unwrap();
}
