use std::time::Instant;

use flate2::{Compression, write::GzEncoder};

use zkbootstrap::*;
use stage0::Assets;

fn main() -> Result<()> {
    let Some(dst) = std::env::args().nth(1) else {
        anyhow::bail!("Usage: jhex0 <output file>");
    };
    let mut store = store::in_memory();

    // FIXME: use hex0
    //let hex0 = store.add_blob(Assets::get("jhex0").unwrap().data);
    let hex0 = store.add_blob(methods::JHEX0_ELF.into());
    let hello_src = store.add_blob(Assets::get("hello.hex0").unwrap().data);

    let hello = store.add_node(hex0, hello_src);

    let prove_start = Instant::now();
    let (output_ref, receipt) = store.prove(hello, Some(&mut std::io::stderr()))?;
    let elapsed_time = prove_start.elapsed();
    eprintln!("Generated a proof in {} secs", elapsed_time.as_secs());

    let hello_output = store.get_blob(output_ref).unwrap();

    let mut w = GzEncoder::new(std::fs::File::create(dst)?, Compression::default());
    rmp_serde::encode::write(&mut w, &vec![store.get_node(hello).unwrap()])?;
    rmp_serde::encode::write(&mut w, &vec![store.get_blob(hex0).unwrap(), store.get_blob(hello_src).unwrap(), hello_output])?;
    rmp_serde::encode::write(&mut w, &vec![(hello, output_ref, &receipt)])?;

    Ok(())
}
