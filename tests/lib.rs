use zkbootstrap::*;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "blob/"]
struct Blobs;

fn jcat_program() -> Vec<u8> {
    Blobs::get("derived/jcat").unwrap().data.to_vec()
}

#[test]
pub fn test_blob() -> Result<()> {
    let output = execute(&jcat_program(), b"hello", None::<&mut std::io::Stderr>)?;
    assert_eq!(output, b"hello");
    Ok(())
}
