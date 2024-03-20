use crate::*;
use ethnum::prelude::*;

// We need to discern between leaf and intermediate nodes to prevent trivial second
// pre-image attacks.
// https://flawed.net.nz/2018/02/21/attacking-merkle-trees-with-a-second-preimage-attack
const LEAF_PREFIX: &[u8] = &[0];
const INTERMEDIATE_PREFIX: &[u8] = &[1];

macro_rules! hash_leaf {
    {$d:expr} => {
        hash_to_u256!(&[LEAF_PREFIX, $d].concat())
    }
}

macro_rules! hash_intermediate {
    {$l:ident, $r:ident} => {
        hash_to_u256!(&[INTERMEDIATE_PREFIX, &$l.to_be_bytes(), &$r.to_be_bytes()].concat())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MerkleProofEntry<'a>(&'a U256, Option<&'a U256>, Option<&'a U256>);

impl<'a> MerkleProofEntry<'a> {
    pub fn new(
        target: &'a U256,
        left_sibling: Option<&'a U256>,
        right_sibling: Option<&'a U256>,
    ) -> Self {
        assert!(left_sibling.is_none() ^ right_sibling.is_none());
        MerkleProofEntry(target, left_sibling, right_sibling)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MerkleProof<'a>(Vec<MerkleProofEntry<'a>>);

impl<'a> MerkleProof<'a> {
    pub fn push(&mut self, entry: MerkleProofEntry<'a>) {
        self.0.push(entry);
    }

    pub fn verify(&self, candidate: U256) -> bool {
        let mut result = self.0.iter().try_fold(candidate, |prev_hash, curr_node| {
            let left_node = curr_node.1.unwrap_or(&prev_hash);
            let right_node = curr_node.2.unwrap_or(&prev_hash);
            let hash = hash_intermediate!(left_node, right_node);

            if hash == *curr_node.0 {
                Some(hash)
            } else {
                None
            }
        });

        result.is_some()
    }
}

#[derive(Debug)]
pub struct MerkleTree {
    leaf_count: usize,
    nodes: Vec<U256>,
}

impl MerkleTree {
    pub fn new<T: AsRef<[u8]>>(items: &[T]) -> Self {
        let cap = MerkleTree::calculate_capacity(items.len());
        let mut tree = MerkleTree {
            leaf_count: items.len(),
            nodes: Vec::with_capacity(cap),
        };

        for item in items {
            let item = item.as_ref();
            let hash = hash_leaf!(&item);
            tree.nodes.push(hash);
        }

        let mut level_len = MerkleTree::next_level_len(items.len());
        let mut level_start = items.len();
        let mut prev_level_len = items.len();
        let mut prev_level_start = 0;
        while level_len > 0 {
            for i in 0..level_len {
                let prev_level_idx = 2 * i;
                let left_node = &tree.nodes[prev_level_start + prev_level_idx];
                let right_node = if prev_level_idx + 1 < prev_level_len {
                    &tree.nodes[prev_level_start + prev_level_idx + 1]
                } else {
                    &tree.nodes[prev_level_start + prev_level_idx]
                };

                let hash = hash_intermediate!(left_node, right_node);
                tree.nodes.push(hash);
            }

            prev_level_start = level_start;
            prev_level_len = level_len;
            level_start += level_len;
            level_len = MerkleTree::next_level_len(level_len);
        }

        tree
    }

    pub fn find_path(&self, index: usize) -> Option<MerkleProof> {
        if index >= self.leaf_count {
            return None;
        }

        let mut level_len = self.leaf_count;
        let mut level_start = 0;
        let mut path = MerkleProof::default();
        let mut node_index = index;
        let mut left_node = None;
        let mut right_node = None;
        while level_len > 0 {
            let level = &self.nodes[level_start..(level_start + level_len)];

            let target = &level[node_index];
            if left_node.is_some() || right_node.is_some() {
                path.push(MerkleProofEntry::new(target, left_node, right_node));
            }
            if node_index % 2 == 0 {
                left_node = None;
                right_node = if node_index + 1 < level.len() {
                    Some(&level[node_index + 1])
                } else {
                    Some(&level[node_index])
                };
            } else {
                left_node = Some(&level[node_index - 1]);
                right_node = None;
            }

            node_index /= 2;
            level_start += level_len;
            level_len = MerkleTree::next_level_len(level_len);
        }

        Some(path)
    }

    pub fn get_root(&self) -> Option<&U256> {
        self.nodes.last()
    }

    #[inline]
    pub fn next_level_len(level_len: usize) -> usize {
        if level_len == 1 {
            0
        } else {
            (level_len + 1) / 2
        }
    }

    pub fn calculate_capacity(leaf_count: usize) -> usize {
        if leaf_count > 0 {
            usize::ilog2(leaf_count) as usize + 2 * leaf_count + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &[&[u8]] = &[
        b"my", b"very", b"eager", b"mother", b"just", b"served", b"us", b"nine", b"pizzas",
        b"make", b"prime",
    ];
    const BAD: &[&[u8]] = &[b"bad", b"missing", b"false"];

    #[test]
    fn test_tree_from_empty() {
        let mt = MerkleTree::new::<[u8; 0]>(&[]);
        assert_eq!(mt.get_root(), None);
    }

    #[test]
    fn test_tree_from_one() {
        let input = b"test";
        let mt = MerkleTree::new(&[input]);
        let expected = hash_leaf!(input);
        assert_eq!(mt.get_root(), Some(&expected));
    }

    #[test]
    fn test_tree_from_many() {
        let mt = MerkleTree::new(TEST);
        // This golden hash will need to be updated whenever the contents of `TEST` change in any
        // way, including addition, removal and reordering or any of the tree calculation algo
        // changes
        let expected = U256::from_str_hex(
            "0xb40c847546fdceea166f927fc46c5ca33c3638236a36275c1346d3dffb84e1bc",
        )
        .unwrap();
        assert_eq!(mt.get_root(), Some(&expected));
    }

    #[test]
    fn test_path_creation() {
        let mt = MerkleTree::new(TEST);
        for (i, _s) in TEST.iter().enumerate() {
            let _path = mt.find_path(i).unwrap();
        }
    }

    #[test]
    fn test_path_creation_bad_index() {
        let mt = MerkleTree::new(TEST);
        assert_eq!(mt.find_path(TEST.len()), None);
    }

    #[test]
    fn test_path_verify_good() {
        let mt = MerkleTree::new(TEST);
        for (i, s) in TEST.iter().enumerate() {
            let hash = hash_leaf!(s);
            let path = mt.find_path(i).unwrap();
            println!("HASH: {:?}", hash);
            println!("TREE: {:?}", mt);
            println!("PATH: {:#?}", path);
            assert!(path.verify(hash));
        }
    }

    #[test]
    fn test_path_verify_bad() {
        let mt = MerkleTree::new(TEST);
        for (i, s) in BAD.iter().enumerate() {
            let hash = hash_leaf!(s);
            let path = mt.find_path(i).unwrap();
            assert!(!path.verify(hash));
        }
    }

    #[test]
    fn test_proof_entry_instantiation_lsib_set() {
        MerkleProofEntry::new(&U256::default(), Some(&U256::default()), None);
    }

    #[test]
    fn test_proof_entry_instantiation_rsib_set() {
        MerkleProofEntry::new(&U256::default(), None, Some(&U256::default()));
    }

    #[test]
    fn test_nodes_capacity_compute() {
        let iteration_count = |mut leaf_count: usize| -> usize {
            let mut capacity = 0;
            while leaf_count > 0 {
                capacity += leaf_count;
                leaf_count = MerkleTree::next_level_len(leaf_count);
            }
            capacity
        };

        // test max 64k leaf nodes compute
        for leaf_count in 0..65536 {
            let math_count = MerkleTree::calculate_capacity(leaf_count);
            let iter_count = iteration_count(leaf_count);
            assert!(math_count >= iter_count);
        }
    }

    #[test]
    #[should_panic]
    fn test_proof_entry_instantiation_both_clear() {
        MerkleProofEntry::new(&U256::default(), None, None);
    }

    #[test]
    #[should_panic]
    fn test_proof_entry_instantiation_both_set() {
        MerkleProofEntry::new(
            &U256::default(),
            Some(&U256::default()),
            Some(&U256::default()),
        );
    }
}
