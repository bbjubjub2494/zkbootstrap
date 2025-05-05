use zkbootstrap::*;

use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::borrow::Cow;

pub fn jhex0_program() -> Cow<'static, [u8]> {
    Artifacts::get("jhex0_program").unwrap().data
}

fn slurp(path: &str) -> anyhow::Result<Vec<u8>> {
    let mut buf = vec![];
    let mut f = File::open(path)?;
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() -> anyhow::Result<()> {
    let program = jhex0_program();

    let stdin = {
        let mut buf = vec![];
        let mut f = std::io::stdin();
        f.read_to_end(&mut buf)?;
        buf
    };

    let mut store = InMemoryStore::new();
    let program_ref = store.add_blob(Blob { bytes: program.to_vec() });
    let stdin_ref = store.add_blob(Blob { bytes: stdin });
    let node_ref = store.add_node(&Node {
        program: program_ref.into(),
        input: stdin_ref.into(),
    });

    let prove_start = Instant::now();
    let (output_ref, receipt) = store.prove(&node_ref, Some(&mut std::io::stderr()))?;
    let elapsed_time = prove_start.elapsed();
    eprintln!("Generated a proof in {} secs", elapsed_time.as_secs());

    store.verify(&node_ref, &output_ref, &receipt)?;

    let output_blob = store.get_blob(output_ref).unwrap();

    Ok(())
}
