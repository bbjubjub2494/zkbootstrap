use std::io::Write;

use risc0_zkvm::{
    compute_image_id, default_executor, default_prover, ExecutorEnv, Journal,
};
pub use risc0_zkvm::Receipt;
use sha2::{Digest, Sha256};

use crate::Result;


pub fn execute(
    program_bytes: &[u8],
    input_bytes: &[u8],
    stderr: Option<&mut impl Write>,
) -> Result<Vec<u8>> {
    let mut output_buffer = vec![];

    let env = build_executor_env(input_bytes, &mut output_buffer, stderr)?;

    let executor = default_executor();

    let session_info = executor.execute(env, program_bytes)?;

    let output_bytes = output_buffer;

    check_journal_consistency(
        &session_info.journal,
        &Sha256::digest(input_bytes).into(),
        &Sha256::digest(&output_bytes).into(),
    )?;

    Ok(output_bytes)
}

pub fn prove(
    program_bytes: &[u8],
    input_bytes: &[u8],
    stderr: Option<&mut impl Write>,
) -> Result<(Vec<u8>, Receipt)> {
    let mut output_buffer = vec![];

    let env = build_executor_env(input_bytes, &mut output_buffer, stderr)?;

    let prover = default_prover();
    let prove_info = prover.prove(env, &program_bytes)?;

    let output_bytes = output_buffer;

    check_journal_consistency(
        &prove_info.receipt.journal,
        &Sha256::digest(input_bytes).into(),
        &Sha256::digest(&output_bytes).into(),
    )?;

    Ok((output_bytes, prove_info.receipt))
}

pub fn verify(
    receipt: &Receipt,
    program_bytes: &[u8],
    input_hash: &[u8; 32],
    output_hash: &[u8; 32],
) -> Result<()> {
        let image_id = compute_image_id(&program_bytes)?;
        receipt.verify(image_id)?;

        check_journal_consistency(&receipt.journal, input_hash, output_hash)?;

        Ok(())
}

pub fn build_executor_env<'a>(
    input_bytes: &'a [u8],
    output_buffer: &'a mut Vec<u8>,
    stderr: Option<&'a mut impl Write>,
) -> Result<ExecutorEnv<'a>> {
    let mut builder = ExecutorEnv::builder();
    builder.stdin(input_bytes);
    builder.stdout(output_buffer);
    if let Some(stderr) = stderr {
        builder.stderr(stderr);
    }
    builder.build()
}

pub fn check_journal_consistency(
    journal: &Journal,
    input_hash: &[u8; 32],
    output_hash: &[u8; 32],
) -> Result<()> {
    let expected = [*input_hash, *output_hash].concat();
    let actual = &journal.bytes;
    if actual != &expected {
        anyhow::bail!(format!(
            "journal mismatch\nactual = {}\nexpected = {}",
            hex::encode(actual),
            hex::encode(&expected)
        ));
    }
    Ok(())
}
