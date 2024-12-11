use std::collections::HashMap;
use std::fs;
use crate::node::Node;
use crate::metadata::Metadata;

#[derive(Debug)]
pub struct Filesystem {
    pub feed: Vec<usize>,
    pub nodes: Vec<Node>,
    pub file_map: HashMap<usize, Metadata>,
}

impl Filesystem {
    pub fn new(path: &str) -> Filesystem {
        let contents = fs::read_to_string(path)
            .expect("Something went wrong reading the file");

        let feed = contents
            .split("")
            .filter_map(|u| u.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        let nodes = Node::list_from_feed(&feed);
        let file_map = Filesystem::file_map(&nodes);

        Filesystem {
            feed,
            nodes,
            file_map,
        }
    }

    pub fn calculate_checksum(&self) -> usize {
        let mut nodes = self.nodes.iter().enumerate();
        let mut acc: usize = 0;
        while let Some((index, node)) = nodes.next() {
            if let Some(id) = node.id {
                if let Some(mulled) = id.checked_mul(index) {
                    acc = acc.saturating_add(mulled);
                }
            }
        }
        acc
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn info(&self) {
        println!(
            "Filesystem:\n- Size: {}\n- Free space: {}",
            self.size(),
            self.free_space()
        );
    }

    pub fn free_space(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| !n.file)
            .collect::<Vec<&Node>>()
            .len()
    }

    fn blocks_reordered(&self, last_free_space: &usize) -> bool {
        *last_free_space == self.size() - self.free_space()
    }

    pub fn reorder_files(&mut self) {
        let max_id: usize = *self.file_map.keys().max().unwrap();
        for id in (0..=max_id).rev() {
            if self.move_file(id) {
                self.clear_file(id);
            }
        }
    }

    fn move_file(&mut self, id: usize) -> bool {
        let metadata = self.file_map.get(&id).unwrap();
        let nodes = self.nodes.clone();
        if let Some(free_space_index) = Filesystem::search_free_space(&nodes, metadata) {
            let mut size = metadata.size;
            for i in free_space_index..free_space_index + metadata.size {
                if let Some(node) = self.nodes.get_mut(i) {
                    let head = if i == free_space_index { true } else { false };
                    node.id = Some(id);
                    node.file = true;
                    node.size = size;
                    node.head = head;
                }

                size -= 1;
            }

            return true;
        }

        false
    }

    fn search_free_space(nodes: &Vec<Node>, metadata: &Metadata) -> Option<usize> {
        for (index, node) in nodes.iter().enumerate() {
            if !node.file && node.size >= metadata.size {
                if index < metadata.index {
                    return Some(index);
                }
            }
        };

        None
    }

    fn clear_file(&mut self, id: usize) {
        let metadata = self.file_map.get(&id).unwrap();
        let mut index = metadata.index;

        while let Some(node) = self.nodes.get_mut(index) {
            if index >= (metadata.index + metadata.size) {
                break;
            }

            node.id = None;
            node.head = false;
            node.file = false;

            index += 1;
        }
    }



    pub fn reorder_blocks(&mut self) {
        let mut index_offset = 0;
        let node_feed = self.nodes.clone();
        let mut last_free_space = 0;
        for i in 0..self.size() - 1 {
            if self.blocks_reordered(&last_free_space) {
                break;
            }

            let node = self.nodes.get_mut(i);
            match node {
                Some(node) => {
                    if !node.file {
                        let next_node = Filesystem::next_node(&node_feed, &mut index_offset);
                        match next_node {
                            Some((id, index)) => {
                                node.id = Some(id);
                                node.file = true;

                                let old_node = self.nodes.get_mut(index).unwrap();
                                old_node.file = false;
                                old_node.id = None;
                                last_free_space = index;
                                self.print();
                            }
                            None => ()
                        }

                    }
                }
                None => ()
            }
        }
    }

    pub fn next_node(nodes: &Vec<Node>, index_offset: &mut usize) -> Option<(usize, usize)> {
        let length = nodes.len();
        while *index_offset < length {
            if let Some(node) = nodes.get(length - 1 - *index_offset) {
                *index_offset += 1;

                if node.file {
                    if let Some(id) = node.id {
                        return Some((id, length - *index_offset));
                    }
                }
            } else {
                break;
            }
        }

        None
    }

    pub fn print(&self) {
        let nodes: String = self.nodes
            .iter()
            .map(|n| {
                match n.id {
                    Some(id) => id.to_string(),
                    None => ".".to_string(),
                }
            }).collect::<Vec<String>>()
        .join("");

        println!("{}", nodes);
    }

    fn file_map(nodes: &Vec<Node>) -> HashMap<usize, Metadata> {
        let mut hash: HashMap<usize, Metadata> = HashMap::new();

        nodes
            .iter()
            .enumerate()
            .for_each(|(index, node)| {
                if node.file && node.head {
                    if let Some(id) = node.id {
                        let size = node.size;
                        let metadata = Metadata {
                            index,
                            size,
                        };
                        hash.entry(id).or_insert(metadata);
                    }
                }
            });

        hash
    }
}
