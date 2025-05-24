pub use anyhow::Result;

pub mod datatypes;
pub mod zkvm;
pub mod store;

pub use datatypes::{Blob, BlobRef, Node, NodeRef};
pub use store::InMemoryStore;
pub use zkvm::Receipt;

#[cfg(test)]
mod zkvm_test;
