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

#[test]
pub fn test_jcat() -> Result<()> {
    let output = execute(&jcat_program(), b"hello", None::<&mut std::io::Stderr>)?;
    assert_eq!(output, b"hello");
    Ok(())
}

#[test]
pub fn test_jcat_reference() -> Result<()> {
    let program = methods::JCAT_ELF;
    // NOTE: the reference code pads the input with zeroes up to word boundary
    // Is this a bug in this version of the risc0 stdlib?
    let input_bytes = b"hello\0\0\0";
    let output_bytes = execute(program, input_bytes, None::<&mut std::io::Stderr>)?;
    assert_eq!(output_bytes, input_bytes);
    Ok(())
}

#[test]
pub fn test_jhex0() -> Result<()> {
    let input_bytes = b"7465 7374 0a";
    let output_bytes = execute(&jhex0_program(), input_bytes, None::<&mut std::io::Stderr>)?;
    assert_eq!(output_bytes, b"test\n");
    Ok(())
}

#[test]
pub fn test_jhex0_reference() -> Result<()> {
    let program = methods::JHEX0_ELF;
    let input_bytes = b"7465 7374 0a";
    let output_bytes = execute(program, input_bytes, None::<&mut std::io::Stderr>)?;
    assert_eq!(output_bytes, b"test\n");
    Ok(())
}
