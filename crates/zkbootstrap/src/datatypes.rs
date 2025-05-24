use std::borrow::Cow;
use std::fmt::Display;

use sha2::{Digest, Sha256};

use serde::{Deserialize, Serialize};


#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlobRef {
    #[serde(with = "serde_bytes")]
    pub hash: [u8; 32],
}

impl Display for BlobRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlobRef({})", hex::encode(self.hash))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blob<'a> {
    //FIXME #[serde(with = "serde_bytes")]
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
    #[serde(with = "serde_bytes")]
    pub hash: [u8; 32],
}

impl Display for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeRef({})", hex::encode(self.hash))
    }
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

impl Display for BlobOrOutputRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // passthrough to the inner types' Display implementation because they are unambiguous
            BlobOrOutputRef::OutputRef(r) => r.fmt(f),
            BlobOrOutputRef::BlobRef(r) => r.fmt(f),
        }
    }
}
