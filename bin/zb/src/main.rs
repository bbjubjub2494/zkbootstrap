use std::path::{Path,PathBuf};

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
    Deps {
        node_path: String,
    },
    DepsTree {
        node_path: String,
    },
}

fn main() -> Result<()> {
    let args = ZbCli::parse();

    match args {
        ZbCli::Import { path } => import(&path),
        ZbCli::Deps { node_path } => { deps(&node_path) },
        ZbCli::DepsTree { node_path } => { deps_tree(&node_path) },
    }
}

fn import(path: &str) -> Result<()> {
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
    Ok(())
}

fn deps(node_path: &str) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let store_path: PathBuf = cwd.join("store");
    let node_path = cwd.join(node_path);
    let Ok(name) = node_path.strip_prefix(&store_path.join("node")) else {
        anyhow::bail!("invalid node path")
    };
    let Ok(hash) = hex::decode(name.to_str().ok_or(anyhow::anyhow!("invalid node path"))?) else {
        anyhow::bail!("invalid node path")
    };
    let r = BlobOrOutputRef::OutputRef(NodeRef{ hash: hash.try_into().unwrap() });
    let store = store::filesystem(store_path);
    store.walk(r, |_, r| { match r {
        store::WalkStep::Blob(blob_ref) => 
            println!("blob: {}", blob_ref),
        _ => (),
    };
    Ok(())
    })?;
    Ok(())
}

fn deps_tree(node_path: &str) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let store_path: PathBuf = cwd.join("store");
    let node_path = cwd.join(node_path);
    let Ok(name) = node_path.strip_prefix(&store_path.join("node")) else {
        anyhow::bail!("invalid node path")
    };
    let Ok(hash) = hex::decode(name.to_str().ok_or(anyhow::anyhow!("invalid node path"))?) else {
        anyhow::bail!("invalid node path")
    };
    let r = BlobOrOutputRef::OutputRef(NodeRef{ hash: hash.try_into().unwrap() });
    // FIXME dummy top of the tree
    let mut tb = ptree::TreeBuilder::new("tree".to_string());
    let store = store::filesystem(store_path);
    store.walk(r, |_, r| { match r {
        store::WalkStep::Blob(blob_ref) => 
            tb.add_empty_child(blob_ref.to_string()),
        store::WalkStep::BeginNode(node_ref) =>
            tb.begin_child(node_ref.to_string()),
        store::WalkStep::StopNode(_) =>
            tb.end_child(),
    };
    Ok(())
    })?;
    ptree::print_tree(&tb.build())?;
    Ok(())
}
