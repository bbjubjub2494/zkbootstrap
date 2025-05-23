use zkbootstrap::*;
use stage0::Assets;

static SAMPLES: &[&[u8]] = &[
    b"",
    b"hello",
    b"testtesttesttest",
    b"testtesttesttesttesttesttesttes", // 63 bytes
    b"testtesttesttesttesttesttesttest", // 64 bytes (block boundary in sha2)
    b"testtesttesttesttesttesttesttestt", // 65 bytes
];

/* FIXME continuous allocation version crashes
#[test]
pub fn test_jcat() -> Result<()> {
    let program = Assets::get("jcat").unwrap().data;
    for &sample in SAMPLES {
        let output_bytes = execute(&program, sample, None::<&mut std::io::Stderr>)?;
        assert_eq!(output_bytes, sample);
    }
    for &sample in SAMPLES {
        let (output_bytes, _) = prove(&program, sample, None::<&mut std::io::Stderr>)?;
        assert_eq!(output_bytes, sample);
    }
    Ok(())
}
*/

#[test]
pub fn test_jcat_reference() -> Result<()> {
    // note: jcat_reference is very slow due to M2-Planet not optimizing, so it is only ran once
    let program = Assets::get("cat_reference").unwrap().data;
    let sample = b"hello";
        let output_bytes = execute(&program, sample, None::<&mut std::io::Stderr>)?;
        assert_eq!(output_bytes, sample);
        let (output_bytes, _) = prove(&program, sample, None::<&mut std::io::Stderr>)?;
        assert_eq!(output_bytes, sample);
    Ok(())
}

#[test]
pub fn test_jhex0() -> Result<()> {
    let program = Assets::get("jhex0").unwrap().data;
    let input_bytes = b"7465 7374 0a";
    let output_bytes = execute(&program, input_bytes, None::<&mut std::io::Stderr>)?;
    assert_eq!(output_bytes, b"test\n");
    let (output_bytes, _) = prove(&program, input_bytes, None::<&mut std::io::Stderr>)?;
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
