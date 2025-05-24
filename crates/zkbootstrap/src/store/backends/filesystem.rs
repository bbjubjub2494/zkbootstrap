use std::borrow::Cow;
use std::path::PathBuf;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

use crate::datatypes::{Blob, BlobOrOutputRef, BlobRef, Node, NodeRef};
use crate::store::Backend;

#[derive(Debug, Clone)]
pub struct FileSystem {
    store_path: PathBuf,
}

impl FileSystem {
    pub fn new(store_path: PathBuf) -> Self {
        FileSystem { store_path }
    }

    fn get_node_path(&self, node_ref: NodeRef) -> std::path::PathBuf {
        let dir = self.store_path.join("node");
        std::fs::create_dir_all(&dir).expect("Failed to create node directory");
        dir.join(hex::encode(&node_ref.hash[..20]))
    }

    fn get_blob_path(&self, blob_ref: BlobRef) -> std::path::PathBuf {
        let dir = self.store_path.join("blob");
        std::fs::create_dir_all(&dir).expect("Failed to create blob directory");
        dir.join(hex::encode(&blob_ref.hash[..20]))
    }

    fn get_output_path(&self, node_ref: NodeRef) -> std::path::PathBuf {
        let dir = self.store_path.join("output");
        std::fs::create_dir_all(&dir).expect("Failed to create output directory");
        dir.join(hex::encode(&node_ref.hash[..20]))
    }
}

impl <'a> Backend<'a> for FileSystem {
    fn add_node(&mut self, program: impl Into<BlobOrOutputRef>, input: impl Into<BlobOrOutputRef>) -> NodeRef {
        let node = Node { program: program.into(), input: input.into() };
        let r = node.compute_ref();
        let dst = std::fs::File::create(self.get_node_path(r)).expect("Failed to create node file");
        format::write(&node, dst).expect("Failed to write node to file system");
        r
    }
    fn add_blob(&mut self, bytes: Cow<'a, [u8]>) -> BlobRef {
        let blob = Blob { bytes };
        let r = blob.compute_ref();
        let p =self.get_blob_path(r);
        std::fs::write(&p, &blob.bytes).expect("Failed to write blob to file system");
        std::fs::set_permissions(p, Permissions::from_mode(0o755)).expect("Failed to set permissions on blob file");
        r
    }
    fn get_node(&self, node_ref: NodeRef) -> Option<Cow<'_, Node>> {
        let src = std::fs::File::open(self.get_node_path(node_ref)).expect("Failed to open node file");
        let node = format::read(&src).expect("Failed to read node from file system");
        Some(Cow::Owned(node))
    }
    fn get_blob(&self, blob_ref: BlobRef) -> Option<Cow<'_, Blob<'a>>> {
        let bytes = std::fs::read(self.get_blob_path(blob_ref)).expect("Failed to read blob from file system");
        let blob = Blob { bytes: Cow::Owned(bytes) };
        Some(Cow::Owned(blob))
    }
    fn add_output_trusted(&mut self, node: NodeRef, output: BlobRef) {
        let dst = std::fs::File::create(self.get_output_path(node)).expect("Failed to create output file");
        format::write(&output, dst).expect("Failed to write output to file system");
    }

    fn resolve_blob(self: &Self, r: BlobOrOutputRef) -> Option<BlobRef> {
        Some(match r {
            BlobOrOutputRef::OutputRef(r) => {
                let src = std::fs::File::open(self.get_output_path(r)).expect("Failed to open output file");
                format::read(&src).expect("Failed to read node from file system")
            },
            BlobOrOutputRef::BlobRef(r) => r,
        })
    }
}
