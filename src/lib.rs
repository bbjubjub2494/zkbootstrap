use sha2::{Digest, Sha256};

use std::collections::HashMap;

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
}

pub fn resolve_blob<'a>(store: &'a InMemoryStore, r: &BlobOrOutputRef) -> (BlobRef, &'a Blob) {
    let r = match r {
        BlobOrOutputRef::OutputRef(r) => *store.outputs.get(r).expect("output unavailable"),
        BlobOrOutputRef::BlobRef(r) => *r,
    };
    (r, store.blobs.get(&r).expect("blob unavailable"))
}

use risc0_zkvm::{compute_image_id, default_prover, Executor, ExecutorEnv, LocalProver, Receipt};

pub fn reexecute(store: &mut InMemoryStore, node_ref: &NodeRef) -> BlobRef {
    let node = store.nodes.get(node_ref).expect("node unavailable");
    let (_, program_blob) = resolve_blob(store, &node.program);
    let (input_ref, input_blob) = resolve_blob(store, &node.input);
    let mut output_buffer = vec![];

    let env = ExecutorEnv::builder()
        .stdin(input_blob.bytes.as_slice())
        .stdout(&mut output_buffer)
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = LocalProver::new("local");

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let session_info = prover
        .execute(env, &program_blob.bytes)
        .expect("execution failed");

    let output_blob = Blob {
        bytes: output_buffer,
    };
    let output_ref = store.add_blob(output_blob);

    assert_eq!(
        session_info.journal.bytes,
        [input_ref.hash, output_ref.hash].concat()
    );
    store.add_output_trusted(node_ref, &output_ref);

    //let image_id = compute_image_id(&program)?;
    //prove_info.receipt.verify(image_id).map_err(anyhow::Error::new)?;
    output_ref
}

pub fn prove(store: &mut InMemoryStore, node_ref: &NodeRef) -> (BlobRef, Receipt) {
    let node = store.nodes.get(node_ref).expect("node unavailable");
    let (_, program_blob) = resolve_blob(store, &node.program);
    let (input_ref, input_blob) = resolve_blob(store, &node.input);
    let mut output_buffer = vec![];

    let env = ExecutorEnv::builder()
        .stdin(input_blob.bytes.as_slice())
        .stdout(&mut output_buffer)
        .stderr(std::io::stderr())
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover
        .prove(env, &program_blob.bytes)
        .expect("execution failed");

    let output_blob = Blob {
        bytes: output_buffer,
    };
    let output_ref = store.add_blob(output_blob);

    assert_eq!(
        prove_info.receipt.journal.bytes,
        [input_ref.hash, output_ref.hash].concat()
    );
    store.add_output_trusted(node_ref, &output_ref);

    (output_ref, prove_info.receipt)
}

pub fn verify(store: &mut InMemoryStore, node_ref: &NodeRef, output_ref: &BlobRef, receipt: Receipt) {
    let node = store.nodes.get(node_ref).expect("node unavailable");
    let (_, program_blob) = resolve_blob(store, &node.program);
    let (input_ref, _) = resolve_blob(store, &node.input);

    let image_id = compute_image_id(&program_blob.bytes).unwrap();
    receipt.verify(image_id).unwrap();

    assert_eq!(
        receipt.journal.bytes,
        [input_ref.hash, output_ref.hash].concat()
    );
    store.add_output_trusted(node_ref, &output_ref);
}
