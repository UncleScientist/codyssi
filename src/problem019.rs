use std::{io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let lines = crate::read_sections(19, "Artifacts at Atlantis")?;
    let artifacts = lines[0]
        .iter()
        .map(|line| line.parse::<Artifact>().unwrap())
        .collect::<Vec<_>>();

    let mut root: Option<Box<BinaryTree>> = None;

    let mut _codelist = Vec::new();
    for artifact in artifacts {
        insert(&mut root, artifact, 1, &mut _codelist);
    }

    let mut layer_sum = Vec::new();
    traverse(&root, &mut |node: &Artifact| {
        if node.layer >= layer_sum.len() {
            layer_sum.resize(node.layer + 1, 0)
        }
        layer_sum[node.layer] += node.id;
    });
    println!(
        "  part 1 = {}",
        layer_sum.iter().max().unwrap() * (layer_sum.len() - 1)
    );

    let missing_one = "asdfgh | 500000".parse::<Artifact>().unwrap();
    let mut codelist = Vec::new();
    insert(&mut root, missing_one, 1, &mut codelist);
    println!("  part 2 = {}", codelist.join("-"));

    Ok(())
}

#[derive(Debug)]
struct BinaryTree {
    node: Artifact,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    fn new(node: Artifact) -> Box<Self> {
        Box::new(Self {
            node,
            left: None,
            right: None,
        })
    }
}

fn traverse<F: FnMut(&Artifact)>(tree: &Option<Box<BinaryTree>>, callback: &mut F) {
    if let Some(entry) = tree {
        traverse(&entry.left, callback);
        callback(&entry.node);
        traverse(&entry.right, callback);
    }
}

fn insert(
    tree: &mut Option<Box<BinaryTree>>,
    mut node: Artifact,
    layer: usize,
    codelist: &mut Vec<String>,
) {
    if let Some(entry) = tree {
        codelist.push(entry.node.code.clone());
        if node.id < entry.node.id {
            insert(&mut entry.left, node, layer + 1, codelist);
        } else {
            insert(&mut entry.right, node, layer + 1, codelist);
        }
    } else {
        node.layer = layer;
        let _ = tree.insert(BinaryTree::new(node));
    }
}

#[derive(Debug)]
struct Artifact {
    code: String,
    id: usize,
    layer: usize,
}

impl FromStr for Artifact {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (left, right) = line.split_once(" | ").unwrap();
        Ok(Self {
            code: left.into(),
            id: right.parse().unwrap(),
            layer: 0,
        })
    }
}
