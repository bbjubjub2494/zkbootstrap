use std::borrow::Cow;
use std::collections::HashMap;

use crate::datatypes::{Blob, BlobOrOutputRef, BlobRef, Node, NodeRef};
use crate::store::Backend;

#[derive(Debug, Clone)]
pub struct InMemory<'a> {
    nodes: HashMap<NodeRef, Node>,
    blobs: HashMap<BlobRef, Blob<'a>>,
    outputs: HashMap<NodeRef, BlobRef>,
}

impl <'a> InMemory<'a> {
    pub fn new() -> Self {
        InMemory {
            nodes: HashMap::new(),
            blobs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }
}

impl <'a> Backend<'a> for InMemory<'a> {
    fn add_node(&mut self, program: impl Into<BlobOrOutputRef>, input: impl Into<BlobOrOutputRef>, output_name: String) -> NodeRef {
        let node = Node { program: program.into(), input: input.into(), output_name };
        let r = node.compute_ref();
        self.nodes.insert(r, node);
        r
    }
    fn add_blob(&mut self, bytes: Cow<'a, [u8]>, name: String) -> BlobRef {
        let blob = Blob { bytes, name };
        let r = blob.compute_ref();
        self.blobs.insert(r.clone(), blob);
        r
    }
    fn get_node(&self, node_ref: NodeRef) -> Option<Cow<'_, Node>> {
        let node = self.nodes.get(&node_ref)?;
        Some(Cow::Borrowed(node))
    }
    fn get_blob(&self, blob_ref: &BlobRef) -> Option<Cow<'_, Blob<'a>>> {
        let blob = self.blobs.get(blob_ref)?;
        Some(Cow::Borrowed(blob))
    }
    fn add_output_trusted(&mut self, node: NodeRef, output: &BlobRef) {
        self.outputs.insert(node, output.clone());
    }

    fn resolve_blob(self: &Self, r: &BlobOrOutputRef) -> Option<BlobRef> {
        Some(match r {
            BlobOrOutputRef::OutputRef(r) => self.outputs.get(&r)?.clone(),
            BlobOrOutputRef::BlobRef(r) => r.clone(),
        })
    }
}
