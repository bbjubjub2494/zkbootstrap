use zkbootstrap::*;

pub fn jcat_program() -> &'static [u8] {
    include_bytes!(concat!(env!("OUT_DIR"), "/jcat"))
}

pub fn jhex0_program() -> &'static [u8] {
    include_bytes!(concat!(env!("OUT_DIR"), "/jhex0"))
}

static SAMPLES: &[&[u8]] = &[
    b"",
    b"hello",
    b"testtesttesttest",
    b"testtesttesttesttesttesttesttes", // 63 bytes
    b"testtesttesttesttesttesttesttest", // 64 bytes (block boundary in sha2)
    b"testtesttesttesttesttesttesttestt", // 65 bytes
];

#[test]
pub fn test_jcat() -> Result<()> {
    for &sample in SAMPLES {
        let output_bytes = execute(&jcat_program(), sample, None::<&mut std::io::Stderr>)?;
        assert_eq!(output_bytes, sample);
    }
    for &sample in SAMPLES {
        let (output_bytes, _) = prove(&jcat_program(), sample, None::<&mut std::io::Stderr>)?;
        assert_eq!(output_bytes, sample);
    }
    Ok(())
}

#[test]
pub fn test_jhex0() -> Result<()> {
    let program = jhex0_program();
    let input_bytes = b"7465 7374 0a";
    let output_bytes = execute(program, input_bytes, None::<&mut std::io::Stderr>)?;
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
