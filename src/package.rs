use std::{collections::HashMap, path::PathBuf};

use petgraph::{adj::NodeIndex, graph::DiGraph};

use crate::{
    build_config::dependencies::{Dependencies, RemoteDependency},
    error::Error,
};

struct DependencyGraph {
    graph: DiGraph<String, ()>,
    node_map: HashMap<String, NodeIndex>,
}

pub struct PackageManager {
    dep_graph: DependencyGraph,
}

#[derive(Clone, Debug)]
pub struct Package<'a> {
    remote_dependency: &'a RemoteDependency,
    local_path: PathBuf,
    deps: Option<&'a [Package<'a>]>,
}

impl PackageManager {}
