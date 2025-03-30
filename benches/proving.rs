use zkbootstrap::*;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "blob/"]
struct Blobs;

fn jcat_program() -> Vec<u8> {
    Blobs::get("derived/jcat").unwrap().data.to_vec()
}

fn jhex0_program() -> Vec<u8> {
    Blobs::get("derived/jhex").unwrap().data.to_vec()
}

fn hex1_source() -> Vec<u8> {
    // ~27kB
    Blobs::get("stage0-posix-riscv32/hex1_riscv32.hex0").unwrap().data.to_vec()
}


pub fn main() {
    std::env::remove_var("RISC0_DEV_MODE");

    let hex1_source= hex1_source();
    let mut input_bytes = hex1_source.clone();
    input_bytes.extend(&hex1_source);
    //let program = jhex0_program();
    let program = methods::JHEX0_ELF;

    let instant = std::time::Instant::now();
    let _ = prove(&program, &input_bytes, None::<&mut std::io::Stderr>);
    let elapsed_time = instant.elapsed();
    println!("Generated a proof in {} secs", elapsed_time.as_secs_f64());
}
