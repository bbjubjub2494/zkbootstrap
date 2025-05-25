pub mod backends;

use std::borrow::Cow;
use std::io::Write;
use std::path::PathBuf;

use anyhow::anyhow;

use crate::datatypes::{Blob, BlobOrOutputRef, BlobRef, Node, NodeRef};
use crate::zkvm;
use crate::Result;

pub trait Backend<'a> {
    fn add_node(&mut self, program: impl Into<BlobOrOutputRef>, input: impl Into<BlobOrOutputRef>) -> NodeRef;
    fn add_blob(&mut self, bytes: Cow<'a, [u8]>) -> BlobRef;
    fn get_node(&self, node_ref: NodeRef) -> Option<Cow<'_, Node>>;
    fn get_blob(&self, blob_ref: BlobRef) -> Option<Cow<'_, Blob<'a>>>;
    fn add_output_trusted(&mut self, node: NodeRef, output: BlobRef);
    fn resolve_blob(self: &Self, r: BlobOrOutputRef) -> Option<BlobRef>;
}

#[derive(Debug)]
pub struct Store<B> {
    backend: B,
}

pub enum WalkStep {
    BeginNode(NodeRef),
    StopNode(NodeRef),
    Blob(BlobRef),
}

pub fn in_memory<'a>() -> Store<backends::InMemory<'a>> {
    Store::new(backends::InMemory::new())
}

pub fn filesystem(store_path: PathBuf) -> Store<backends::FileSystem> {
    Store::new(backends::FileSystem::new(store_path))
}

impl <'a, B: Backend<'a>> Store<B> {
    fn new(backend: B) -> Self {
        Store{ backend }
    }

    pub fn reexecute(
        self: &mut Self,
        node_ref: NodeRef,
        stderr: Option<&mut impl Write>,
    ) -> Result<BlobRef> {
        let node = self.get_node(node_ref)?;
        let program_blob = self.get_blob(self.resolve_blob(node.program)?)?;
        let input_blob = self.get_blob(self.resolve_blob(node.input)?)?;

        let output_bytes = zkvm::execute(&program_blob.bytes, &input_blob.bytes, stderr)?;

        let output_ref = self.add_blob(output_bytes.into());

        self.add_output_trusted(node_ref, output_ref);
        Ok(output_ref)
    }

    pub fn prove(
        self: &mut Self,
        node_ref: NodeRef,
        stderr: Option<&mut impl Write>,
    ) -> Result<(BlobRef, zkvm::Receipt)> {
        let node = self.get_node(node_ref)?;
        let program_blob = self.get_blob(self.resolve_blob(node.program)?)?;
        let input_blob = self.get_blob(self.resolve_blob(node.input)?)?;

        let (output_bytes, receipt) = zkvm::prove(&program_blob.bytes, &input_blob.bytes, stderr)?;

        let output_ref = self.add_blob(output_bytes.into());

        self.add_output_trusted(node_ref, output_ref);
        Ok((output_ref, receipt))
    }

    pub fn verify(
        self: &mut Self,
        node_ref: NodeRef,
        output_ref: BlobRef,
        receipt: &zkvm::Receipt,
    ) -> Result<()> {
        let node = self.get_node(node_ref)?;
        let program_blob = self.get_blob(self.resolve_blob(node.program)?)?;
        let input_ref = self.resolve_blob(node.input)?;

        zkvm::verify(
            receipt,
            &program_blob.bytes,
            &input_ref.hash,
            &output_ref.hash,
        )?;

        self.add_output_trusted(node_ref, output_ref);
        Ok(())
    }

    pub fn add_node(&mut self, program: impl Into<BlobOrOutputRef>, input: impl Into<BlobOrOutputRef>) -> NodeRef {
        self.backend.add_node(program, input)
    }
    pub fn add_blob(&mut self, bytes: Cow<'a, [u8]>) -> BlobRef {
        self.backend.add_blob(bytes)
    }
    pub fn get_node(&self, node_ref: NodeRef) -> Result<Cow<'_, Node>> {
        self.backend.get_node(node_ref).ok_or(anyhow!("node not found: {}", node_ref))
    }
    pub fn get_blob(&self, blob_ref: BlobRef) -> Result<Cow<'_, Blob<'a>>> {
        self.backend.get_blob(blob_ref).ok_or(anyhow!("blob not found: {}", blob_ref))
    }
    pub fn add_output_trusted(&mut self, node: NodeRef, output: BlobRef) {
        self.backend.add_output_trusted(node, output);
    }
    pub fn resolve_blob(self: &Self, r: BlobOrOutputRef) -> Result<BlobRef> {
        self.backend.resolve_blob(r).ok_or(anyhow!("output unavailable for {}", r))
    }

    pub fn walk(&self, r: BlobOrOutputRef, mut f: impl FnMut(&Self, WalkStep) -> Result<()>) -> Result<()> {
        self.inner_walk(r, &mut f)
    }
    fn inner_walk(&self, r: BlobOrOutputRef, f: &mut impl FnMut(&Self, WalkStep) -> Result<()>) -> Result<()> {
        match r {
            BlobOrOutputRef::OutputRef(output_ref) => {
                f(self, WalkStep::BeginNode(output_ref))?;
                let node = self.get_node(output_ref)?;
                // FIXME: recursion
                self.inner_walk(node.program, f)?;
                self.inner_walk(node.input, f)?;
                f(self, WalkStep::StopNode(output_ref))?;
            }
            BlobOrOutputRef::BlobRef(node_ref) => {
                f(self, WalkStep::Blob(node_ref))?;
            }
        }
        Ok(())
    }
}
