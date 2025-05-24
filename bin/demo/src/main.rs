use zkbootstrap::*;
use stage0::Assets;

use std::time::Instant;

fn main() -> Result<()> {
    let mut store = InMemoryStore::new();

    let hex0 = store.add_blob(Assets::get("jhex0").unwrap().data);
    let hello_src = store.add_blob(Assets::get("hello.hex0").unwrap().data);

    let hello_recipe = store.add_node(hex0.into(), hello_src.into());

    let prove_start = Instant::now();
    let (output_ref, receipt) = store.prove(hello_recipe, Some(&mut std::io::stderr()))?;
    let elapsed_time = prove_start.elapsed();
    eprintln!("Generated a proof in {} secs", elapsed_time.as_secs());

    let hello_output = store.get_blob(output_ref).unwrap();

    serde_json::to_writer_pretty(std::io::stdout(), &[&hello_recipe])?;
    serde_json::to_writer_pretty(std::io::stdout(), &[store.get_blob(hex0).unwrap(), store.get_blob(hello_src).unwrap(), &hello_output])?;
    serde_json::to_writer_pretty(std::io::stdout(), &[&receipt])?;

    Ok(())
}
