use zkbootstrap::*;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "blob/"]
struct Blobs;

fn jcat_program() -> Vec<u8> {
    Blobs::get("derived/jcat").unwrap().data.to_vec()
}

#[test]
pub fn test_jcat() -> Result<()> {
    let output = execute(&jcat_program(), b"hello", None::<&mut std::io::Stderr>)?;
    assert_eq!(output, b"hello");
    Ok(())
}

#[test]
pub fn test_jcat_reference() -> Result<()> {
    let program = methods::STAGE0_ELF;
    // NOTE: bug in the reference code: it pads the input with zeroes up to a multiple of 4 bytes
    let input_bytes = b"hello\0\0\0";
    let output_bytes = execute(program, input_bytes, None::<&mut std::io::Stderr>)?;
    assert_eq!(output_bytes, input_bytes);
    Ok(())
}
