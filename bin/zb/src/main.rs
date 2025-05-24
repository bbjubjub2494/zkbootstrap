use std::path::PathBuf;

use clap::Parser;
use flate2::read::GzDecoder;

use zkbootstrap::*;

#[derive(Parser, Debug)]
#[command(name = "zb")]
enum ZbCli {
    Import {
        /// The path to the zkbootstrap archive to import
        path: String,
    },
}

fn main() -> Result<()> {
    let args = ZbCli::parse();

    match args {
        ZbCli::Import { path } => {
            let mut dec = GzDecoder::new(std::fs::File::open(path)?);
            let nodes: Vec<Node> = rmp_serde::decode::from_read(&mut dec)?;
            let blobs: Vec<Blob> = rmp_serde::decode::from_read(&mut dec)?;
            let rcpts: Vec<(NodeRef, BlobRef, Receipt)> = rmp_serde::decode::from_read(&mut dec)?;
            let store_path: PathBuf = std::env::current_dir()?.join("store");
            let mut store = store::filesystem(store_path);
            for node in nodes {
                store.add_node(node.program, node.input);
            }
            for blob in blobs {
                store.add_blob(blob.bytes);
            }
            for (node_ref, output_ref, receipt) in rcpts {
                store.verify(node_ref, output_ref, &receipt)?;
            }
        }
    }

    Ok(())
}
