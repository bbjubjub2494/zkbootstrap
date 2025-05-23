#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::guest::sha::{Sha256,self};
use std::io::Read;
use std::io::Write;


risc0_zkvm::guest::entry!(main);

fn main() {
    // Ingest input
    let mut input = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input).unwrap();

    // Hash input and commit to hash
    let input_hash = sha::Impl::hash_bytes(&input);
    env::journal().write_all(input_hash.as_bytes()).unwrap();

    let output = input;

    // Hash output and commit to hash
    let output_hash = input_hash;
    env::journal().write_all(output_hash.as_bytes()).unwrap();

    // Send output to host
    env::stdout().write_all(&output).unwrap();
}
