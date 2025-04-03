use std::{io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let sections = crate::read_sections(19, "Artifacts at Atlantis")?;
    let artifacts = sections[0]
        .iter()
        .map(|line| line.parse::<Artifact>().unwrap())
        .collect::<Vec<_>>();

    let mut root: Option<Box<BinaryTree>> = None;

    for artifact in artifacts {
        insert(&mut root, artifact, 1);
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
    // 1146586780

    let missing_one = "asdfgh | 500000".parse::<Artifact>().unwrap();
    let mut codelist = Vec::new();
    find(&root, &missing_one, &mut codelist);
    println!("  part 2 = {}", codelist.join("-"));
    // invlzfI-gecqIRf-NpYaQSb-batqlnG-ZzpMEmN-XUasQMx-KZRhJMk-RBWFcpT-yLgLIBv-TXZsSvS-SEviNEC

    let artifact1 = sections[1][0].parse::<Artifact>().unwrap();
    let artifact2 = sections[1][1].parse::<Artifact>().unwrap();
    let mut path1 = Vec::new();
    let mut path2 = Vec::new();

    find(&root, &artifact1, &mut path1);
    find(&root, &artifact2, &mut path2);

    let first_mismatch = path1
        .iter()
        .zip(path2.iter())
        .enumerate()
        .find(|(_, (p1, p2))| p1 != p2)
        .unwrap();

    println!("  part 3 = {}", path1[first_mismatch.0 - 1]);

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

fn find(tree: &Option<Box<BinaryTree>>, node: &Artifact, codelist: &mut Vec<String>) {
    if let Some(entry) = tree {
        codelist.push(entry.node.code.clone());
        if node.id < entry.node.id {
            find(&entry.left, node, codelist);
        } else {
            find(&entry.right, node, codelist);
        }
    }
}

fn insert(tree: &mut Option<Box<BinaryTree>>, mut node: Artifact, layer: usize) {
    if let Some(entry) = tree {
        if node.id < entry.node.id {
            insert(&mut entry.left, node, layer + 1);
        } else {
            insert(&mut entry.right, node, layer + 1);
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
