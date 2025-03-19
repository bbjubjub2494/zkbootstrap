use sha2::{Digest, Sha256};

use std::collections::HashMap;

type Hash = [u8; 32];

#[derive(Debug, Clone)]
pub struct Blob {
    pub content: Vec<u8>,
}

impl Blob {
    pub fn digest(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(&self.content);
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NodeOrBlobReference {
    NodeReference(Hash),
    BlobReference(Hash),
}
impl NodeOrBlobReference {
    pub fn digest(&self) -> Hash {
        let mut hasher = Sha256::new();
        match self {
            NodeOrBlobReference::NodeReference(d) => {
                hasher.update(Sha256::digest(
                    b"zkboostrap.NodeOrBlobReference::NodeReference",
                ));
                hasher.update(d);
            }
            NodeOrBlobReference::BlobReference(d) => {
                hasher.update(Sha256::digest(
                    b"zkboostrap.NodeOrBlobReference::BlobReference",
                ));
                hasher.update(d);
            }
        }
        hasher.finalize().into()
    }
}

pub use NodeOrBlobReference::{BlobReference, NodeReference};

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub program: NodeOrBlobReference,
    pub stdin: NodeOrBlobReference,
}

impl Node {
    pub fn digest(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(Sha256::digest(b"zkbootstrap.Node"));
        hasher.update(self.program.digest());
        hasher.update(self.stdin.digest());
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone)]
pub struct InMemoryStore {
    nodes: HashMap<Hash, Node>,
    blobs: HashMap<Hash, Blob>,
    outputs: HashMap<Hash, Hash>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        InMemoryStore {
            nodes: HashMap::new(),
            blobs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }
    pub fn add_node(&mut self, node: &Node) -> Hash {
        let digest = node.digest();
        self.nodes.insert(digest, *node);
        digest
    }
    pub fn add_blob(&mut self, blob: Blob) -> Hash {
        let digest = blob.digest();
        self.blobs.insert(digest, blob);
        digest
    }
    pub fn add_output_trusted(&mut self, node: &Hash, output: &Hash) {
        self.outputs.insert(*node, *output);
    }
}

pub fn resolve_blob<'a>(store: &'a InMemoryStore, r: &NodeOrBlobReference) -> &'a Blob {
    let hash = match r {
        NodeOrBlobReference::NodeReference(ref d) => {
            store.outputs.get(d).expect("output unavailable")
        }
        NodeOrBlobReference::BlobReference(ref d) => d,
    };
    store.blobs.get(hash).expect("blob unavailable")
}

use risc0_zkvm::{compute_image_id, default_prover, Executor, ExecutorEnv, LocalProver};

pub fn reexecute(store: &mut InMemoryStore, node_hash: &Hash) -> Hash {
    let node = store.nodes.get(node_hash).expect("node unavailable");
    let program_blob = resolve_blob(store, &node.program);
    let stdin_blob = resolve_blob(store, &node.stdin);
    let stdin_hash = stdin_blob.digest();
    // TODO extra input
    let mut stdout_buffer = vec![];

    let env = ExecutorEnv::builder()
        .stdin(stdin_blob.content.as_slice())
        .stdout(&mut stdout_buffer)
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = LocalProver::new("local");

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let session_info = prover
        .execute(env, &program_blob.content)
        .expect("execution failed");

    let stdout_blob = Blob {
        content: stdout_buffer,
    };
    let stdout_hash = store.add_blob(stdout_blob);

    assert_eq!(
        session_info.journal.bytes,
        [stdin_hash, stdout_hash].concat()
    );
    store.add_output_trusted(node_hash, &stdout_hash);

    //let image_id = compute_image_id(&program)?;
    //prove_info.receipt.verify(image_id).map_err(anyhow::Error::new)?;
    stdout_hash
}
