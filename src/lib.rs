use sha2::{Digest, Sha256};

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "$OUT_DIR"]
pub struct Artifacts;

use std::collections::HashMap;
use std::io::Write;

pub use anyhow::Result;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct BlobRef {
    pub hash: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct Blob {
    pub bytes: Vec<u8>,
}

impl Blob {
    pub fn compute_ref(&self) -> BlobRef {
        let mut hasher = Sha256::new();
        hasher.update(&self.bytes);
        BlobRef {
            hash: hasher.finalize().into(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct NodeRef {
    pub hash: [u8; 32],
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub program: BlobOrOutputRef,
    pub input: BlobOrOutputRef,
}

impl Node {
    pub fn compute_ref(&self) -> NodeRef {
        let mut hasher = Sha256::new();
        hasher.update(Sha256::digest(b"zkbootstrap::Node"));
        hasher.update(self.program.digest());
        hasher.update(self.input.digest());
        NodeRef {
            hash: hasher.finalize().into(),
        }
    }
}

impl From<BlobRef> for BlobOrOutputRef {
    fn from(blob_ref: BlobRef) -> Self {
        Self::BlobRef(blob_ref)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BlobOrOutputRef {
    OutputRef(NodeRef),
    BlobRef(BlobRef),
}
impl BlobOrOutputRef {
    pub fn digest(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        match self {
            BlobOrOutputRef::OutputRef(r) => {
                hasher.update(Sha256::digest(b"zkbootstrap::BlobOrOutputRef::OutputRef"));
                hasher.update(r.hash);
            }
            BlobOrOutputRef::BlobRef(r) => {
                hasher.update(Sha256::digest(b"zkbootstrap::BlobOrOutputRef::BlobRef"));
                hasher.update(r.hash);
            }
        }
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone)]
pub struct InMemoryStore {
    nodes: HashMap<NodeRef, Node>,
    blobs: HashMap<BlobRef, Blob>,
    outputs: HashMap<NodeRef, BlobRef>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        InMemoryStore {
            nodes: HashMap::new(),
            blobs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    pub fn reexecute(
        self: &mut Self,
        node_ref: &NodeRef,
        stderr: Option<&mut impl Write>,
    ) -> Result<BlobRef> {
        let node = self.nodes.get(node_ref).expect("node unavailable");
        let (_, program_blob) = self.resolve_blob(&node.program);
        let (_, input_blob) = self.resolve_blob(&node.input);

        let output_bytes = execute(&program_blob.bytes, &input_blob.bytes, stderr)?;

        let output_blob = Blob {
            bytes: output_bytes,
        };
        let output_ref = self.add_blob(output_blob);

        self.add_output_trusted(node_ref, &output_ref);
        Ok(output_ref)
    }

    pub fn prove(
        self: &mut Self,
        node_ref: &NodeRef,
        stderr: Option<&mut impl Write>,
    ) -> Result<(BlobRef, Receipt)> {
        let node = self.nodes.get(node_ref).expect("node unavailable");
        let (_, program_blob) = self.resolve_blob(&node.program);
        let (_, input_blob) = self.resolve_blob(&node.input);

        let (output_bytes, receipt) = prove(&program_blob.bytes, &input_blob.bytes, stderr)?;

        let output_blob = Blob {
            bytes: output_bytes,
        };
        let output_ref = self.add_blob(output_blob);

        self.add_output_trusted(node_ref, &output_ref);
        Ok((output_ref, receipt))
    }

    pub fn verify(
        self: &mut Self,
        node_ref: &NodeRef,
        output_ref: &BlobRef,
        receipt: &Receipt,
    ) -> Result<()> {
        let node = self.nodes.get(node_ref).expect("node unavailable");
        let (_, program_blob) = self.resolve_blob(&node.program);
        let (input_ref, _) = self.resolve_blob(&node.input);

        let image_id = compute_image_id(&program_blob.bytes).unwrap();
        receipt.verify(image_id).unwrap();

        check_journal_consistency(&receipt.journal, &input_ref.hash, &output_ref.hash)?;
        self.add_output_trusted(node_ref, &output_ref);
        Ok(())
    }

    pub fn add_node(&mut self, node: &Node) -> NodeRef {
        let r = node.compute_ref();
        self.nodes.insert(r, *node);
        r
    }
    pub fn add_blob(&mut self, blob: Blob) -> BlobRef {
        let r = blob.compute_ref();
        self.blobs.insert(r, blob);
        r
    }
    pub fn get_blob(&self, blob_ref: BlobRef) -> Option<&Blob> {
        self.blobs.get(&blob_ref)
    }
    pub fn add_output_trusted(&mut self, node: &NodeRef, output: &BlobRef) {
        self.outputs.insert(*node, *output);
    }

    pub fn resolve_blob<'a>(self: &'a Self, r: &BlobOrOutputRef) -> (BlobRef, &'a Blob) {
        let r = match r {
            BlobOrOutputRef::OutputRef(r) => *self.outputs.get(r).expect("output unavailable"),
            BlobOrOutputRef::BlobRef(r) => *r,
        };
        (r, self.blobs.get(&r).expect("blob unavailable"))
    }
}

use risc0_zkvm::{
    compute_image_id, default_executor, default_prover, ExecutorEnv, Journal, Receipt,
};

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
