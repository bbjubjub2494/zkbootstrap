use clap::Parser;
use flate2::read::GzDecoder;

use zkbootstrap::*;
use zkbootstrap::store::Store;
use zkbootstrap::store::backends;

use std::path::Path;

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

fn get_store() -> Result<Store<backends::FileSystem>> {
    // TODO CLI option to use a non-cwd store
    let path = std::env::current_dir()?.join("store");
    Ok(store::filesystem(path))
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
    let mut store = get_store()?;
    for node in nodes {
        store.add_node(node.program, node.input, node.output_name);
    }
    for blob in blobs {
        store.add_blob(blob.bytes, blob.name);
    }
    for (node_ref, output_ref, receipt) in rcpts {
        store.verify(node_ref, output_ref, &receipt)?;
    }
    Ok(())
}

fn deps(node_path: &str) -> Result<()> {
    let store = get_store()?;
    let Some(node_ref) = store.backend.parse_node_path(Path::new(node_path)) else {
        anyhow::bail!("not a node path: {}", node_path);
    };
    store.walk(&BlobOrOutputRef::OutputRef(node_ref), |_, r| {
        if let store::WalkStep::Blob(blob_ref) = r {
            println!("{}", pretty_path(&store.backend.get_blob_path(&blob_ref)));
        };
        Ok(())
    })
}

fn deps_tree(node_path: &str) -> Result<()> {
    let store = get_store()?;
    let Some(node_ref) = store.backend.parse_node_path(Path::new(node_path)) else {
        anyhow::bail!("not a node path: {}", node_path);
    };
    // FIXME dummy top of the tree
    let mut tb = ptree::TreeBuilder::new("tree".to_string());
    store.walk(&BlobOrOutputRef::OutputRef(node_ref), |_, r| { match r {
        store::WalkStep::Blob(blob_ref) => 
            tb.add_empty_child(pretty_path(&store.backend.get_blob_path(&blob_ref))),
        store::WalkStep::BeginNode(node_ref) =>
            tb.begin_child(pretty_path(&store.backend.get_blob_path(&store.resolve_blob(&BlobOrOutputRef::OutputRef(node_ref))?))),
        store::WalkStep::StopNode(_) =>
            tb.end_child(),
    };
    Ok(())
    })?;
    ptree::print_tree(&tb.build())?;
    Ok(())
}

fn pretty_path(p: &Path) -> String {
    let cwd = std::env::current_dir().expect("unable to cwd");
    if let Ok(p) = p.strip_prefix(cwd) {
        p.to_str().unwrap_or("<unrepresentable path>").to_owned()
    } else {
        p.to_str().unwrap_or("<unrepresentable path>").to_owned()
    }
}
