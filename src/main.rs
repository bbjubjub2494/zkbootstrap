use zkbootstrap::*;

use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;

fn slurp(path: &str) -> anyhow::Result<Vec<u8>> {
    let mut buf = vec![];
    let mut f = File::open(path)?;
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() -> anyhow::Result<()> {
    let program = slurp(std::env::args().nth(1).unwrap().as_str())?;

    let stdin = {
        let mut buf = vec![];
        let mut f = std::io::stdin();
        f.read_to_end(&mut buf)?;
        buf
    };

    let mut store = InMemoryStore::new();
    let program_ref = store.add_blob(Blob { bytes: program });
    let stdin_ref = store.add_blob(Blob { bytes: stdin });
    let node_ref = store.add_node(&Node {
        program: program_ref.into(),
        input: stdin_ref.into(),
    });

    let prove_start = Instant::now();
    let (output_ref, receipt) = store.prove(&node_ref, Some(&mut std::io::stderr()))?;
    let elapsed_time = prove_start.elapsed();
    eprintln!("Generated a proof in {} secs", elapsed_time.as_secs());

    let verify_start = Instant::now();
    store.verify(&node_ref, &output_ref, receipt)?;
    let elapsed_time = verify_start.elapsed();
    eprintln!("Verified proof in {} secs", elapsed_time.as_secs());

    let output_blob = store.get_blob(output_ref).unwrap();

    std::io::stdout().write_all(&output_blob.bytes)?;

    Ok(())
}
