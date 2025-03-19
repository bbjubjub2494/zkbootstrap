use zkbootstrap::*;

use std::fs::File;
use std::io::{Read, Write};

fn main() -> anyhow::Result<()> {
    let program = {
        let mut buf = vec![];
        let mut f = File::open(std::env::args().nth(1).unwrap())?;
        f.read_to_end(&mut buf)?;
        buf
    };

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

    let output_hash = reexecute(&mut store, &node_ref);

    let output_blob = store.get_blob(output_hash).unwrap();

    std::io::stdout().write_all(&output_blob.bytes)?;

    Ok(())
}
