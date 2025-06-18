pub use anyhow::Result;

pub mod datatypes;
pub mod zkvm;
pub mod store;
pub mod format;

pub use datatypes::{Blob, BlobRef, Node, NodeRef, BlobOrOutputRef};
pub use zkvm::Receipt;
