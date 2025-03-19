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
    let program_hash = store.add_blob(Blob { content: program });
    let stdin_hash = store.add_blob(Blob { content: stdin });
    let node_hash = store.add_node(&Node {
        program: BlobReference(program_hash),
        stdin: BlobReference(stdin_hash),
    });

    let output_hash = reexecute(&mut store, &node_hash);

    let output_blob = resolve_blob(&store, &BlobReference(output_hash));

    std::io::stdout().write_all(&output_blob.content)?;

    Ok(())
}
