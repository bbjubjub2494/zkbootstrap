use std::borrow::Cow;

use sha2::{Digest, Sha256};

use serde::{Deserialize, Serialize};


#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlobRef {
    pub hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blob<'a> {
    pub bytes: Cow<'a, [u8]>,
}

impl<'a> Blob<'a> {
    pub fn compute_ref(&self) -> BlobRef {
        let mut hasher = Sha256::new();
        hasher.update(&self.bytes);
        BlobRef {
            hash: hasher.finalize().into(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NodeRef {
    pub hash: [u8; 32],
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
