use methods::STAGE0_ELF;
use risc0_zkvm::{compute_image_id, default_prover, sha, sha::Sha256, Bytes, ExecutorEnv};

struct Derivation {
    pub program: Bytes,
    pub input: Bytes,
}

struct Receipt {
    pub derivation: Derivation,
    pub output: Bytes,
    inner: risc0_zkvm::Receipt,
}

impl Derivation {
    fn prove(self) -> anyhow::Result<Receipt> {
        let mut output = vec![];
        let env = ExecutorEnv::builder()
            .stdin(self.input.as_ref())
            .stdout(&mut output)
            .build()?;

        // Obtain the default prover.
        let prover = default_prover();

        // Proof information by proving the specified ELF binary.
        // This struct contains the receipt along with statistics about execution of the guest
        let prove_info = prover.prove(env, &self.program)?;

        Ok(Receipt {
            derivation: self,
            output: output.into(),
            inner: prove_info.receipt,
        })
    }
}

impl Receipt {
    fn verify(&self) -> anyhow::Result<()> {
        let input_hash = sha::Impl::hash_bytes(&self.derivation.input);
        let output_hash = sha::Impl::hash_bytes(&self.output);
        let expected_journal_bytes = [input_hash.as_bytes(), output_hash.as_bytes()].concat();
        if self.inner.journal.bytes != expected_journal_bytes {
            eprintln!(
                "expected = {:?}, actual = {:?}",
                expected_journal_bytes, self.inner.journal.bytes
            );
            return Err(anyhow::Error::msg("journal bytes mismatch"));
        }

        let image_id = compute_image_id(STAGE0_ELF)?;
        self.inner.verify(image_id).map_err(anyhow::Error::new)
    }
}

fn main() -> anyhow::Result<()> {
    let derivation = Derivation {
        program: STAGE0_ELF.into(),
        input: "48 65 6c 6c 6f 20 77 6f 72 6c 64 21".into(),
    };
    let receipt = derivation.prove()?;
    receipt.verify()?;
    println!("output = {:?}", receipt.output);
    Ok(())
}
