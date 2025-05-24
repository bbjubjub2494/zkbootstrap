use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;

use crate::datatypes::{Blob, BlobOrOutputRef, BlobRef, Node, NodeRef};
use crate::zkvm;
use crate::Result;

#[derive(Debug, Clone)]
pub struct InMemoryStore<'a> {
    nodes: HashMap<NodeRef, Node>,
    blobs: HashMap<BlobRef, Blob<'a>>,
    outputs: HashMap<NodeRef, BlobRef>,
}

impl <'a> InMemoryStore<'a> {
    pub fn new() -> Self {
        InMemoryStore {
            nodes: HashMap::new(),
            blobs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    pub fn reexecute(
        self: &mut Self,
        node_ref: NodeRef,
        stderr: Option<&mut impl Write>,
    ) -> Result<BlobRef> {
        let node = self.nodes.get(&node_ref).expect("node unavailable");
        let (_, program_blob) = self.resolve_blob(&node.program);
        let (_, input_blob) = self.resolve_blob(&node.input);

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
        let node = self.nodes.get(&node_ref).expect("node unavailable");
        let (_, program_blob) = self.resolve_blob(&node.program);
        let (_, input_blob) = self.resolve_blob(&node.input);

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
        let node = self.nodes.get(&node_ref).expect("node unavailable");
        let (_, program_blob) = self.resolve_blob(&node.program);
        let (input_ref, _) = self.resolve_blob(&node.input);

        zkvm::verify(
            receipt,
            &program_blob.bytes,
            &input_ref.hash,
            &output_ref.hash,
        )?;

        self.add_output_trusted(node_ref, output_ref);
        Ok(())
    }

    pub fn add_node(&mut self, program: BlobOrOutputRef, input: BlobOrOutputRef) -> NodeRef {
        let node = Node { program, input };
        let r = node.compute_ref();
        self.nodes.insert(r, node);
        r
    }
    pub fn add_blob(&mut self, bytes: Cow<'a, [u8]>) -> BlobRef {
        let blob = Blob { bytes };
        let r = blob.compute_ref();
        self.blobs.insert(r, blob);
        r
    }
    pub fn get_blob(&self, blob_ref: BlobRef) -> Option<&Blob<'a>> {
        self.blobs.get(&blob_ref)
    }
    pub fn add_output_trusted(&mut self, node: NodeRef, output: BlobRef) {
        self.outputs.insert(node, output);
    }

    pub fn resolve_blob(self: &Self, r: &BlobOrOutputRef) -> (BlobRef, &Blob<'a>) {
        let r = match r {
            BlobOrOutputRef::OutputRef(r) => *self.outputs.get(r).expect("output unavailable"),
            BlobOrOutputRef::BlobRef(r) => *r,
        };
        (r, self.blobs.get(&r).expect("blob unavailable"))
    }
}
