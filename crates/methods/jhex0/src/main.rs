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

    // Process input
    let mut toggle = false;
    let mut holder = 0;

    let mut output = Vec::new();
    let mut it = input.iter();
    while let Some(&c) = it.next() {
        let r0 = hex(c, &mut it);
        if r0 >= 0 {
            if toggle {
                let byte = (holder * 16) + r0 as u8;
                output.push(byte);
                holder = 0;
            } else {
                holder = r0 as u8;
            }
            toggle = !toggle;
        }
    }

    // Hash output and commit to hash
    let output_hash = sha::Impl::hash_bytes(&output);
    env::journal().write_all(output_hash.as_bytes()).unwrap();

    // Send output to host
    env::stdout().write_all(&output).unwrap();
}

fn hex<'a>(c: u8, it: &mut impl Iterator<Item =&'a u8>) -> i32 {
    // Clear out line comments
    if c == b';' || c == b'#' {
        line_comment(it);
        return -1;
    }

    // Deal with non-hex chars
    if c < b'0' {
        return -1;
    }

    // Deal with 0-9
    if c <= b'9' {
        return (c - b'0') as i32;
    }

    // Convert a-f to A-F
    let c = c & 0xDF;

    // Get rid of everything below A
    if c < b'A' {
        return -1;
    }

    // Deal with A-F
    if c <= b'F' {
        return (c - b'A' + 10) as i32;
    }

    // Everything else is garbage
    -1
}

fn line_comment<'a>(it: &mut impl Iterator<Item = &'a u8>) {
    loop {
        let c = *it.next().unwrap();
        if c == b'\n' || c == b'\r' {
            break;
        }
    }
}
