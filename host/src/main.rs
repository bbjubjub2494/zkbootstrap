use risc0_zkvm::{compute_image_id, default_prover, sha, sha::Sha256, Bytes, ExecutorEnv};

use std::io::Read;

fn main() -> anyhow::Result<()> {
    let mut program = vec![];
    {
        let mut f = std::fs::File::open("hello").unwrap();
        f.read_to_end(&mut program).unwrap();
    }

    let mut output = vec![];
        let env = ExecutorEnv::builder()
            .stdin(&[1,0,0,1u8] as &[u8])
            .stdout(&mut output)
            .build()?;

        // Obtain the default prover.
        let prover = default_prover();

        // Proof information by proving the specified ELF binary.
        // This struct contains the receipt along with statistics about execution of the guest
        let prove_info = prover.prove(env, &program)?;

        let image_id = compute_image_id(&program)?;
        println!("output = {:?}", output);
        prove_info.receipt.verify(image_id).map_err(anyhow::Error::new)?;
    Ok(())
}
