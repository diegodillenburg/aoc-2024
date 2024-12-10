use std::fs;

#[derive(Debug)]
pub struct Filesystem {
    pub feed: Vec<usize>,
    pub nodes: Vec<Node>,
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

        Filesystem {
            feed,
            nodes,
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
                                // self.print();
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
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Option<usize>,
    pub file: bool,
}

impl Node {
    fn list_from_feed(feed: &Vec<usize>) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];
        let mut file: bool = true;

        let mut file_index: usize = 0;
        feed
            .iter()
            .enumerate()
            .for_each(|(i, &size)| {
                if (i + 1) % 2 == 0 {
                    file = false;
                } else {
                    file = true;
                }

                for _ in 0..size {
                    let id = if file {
                        Some(file_index)
                    } else {
                        None
                    };

                    let node = Node {
                        id,
                        file,
                    };

                    nodes.push(node);
                }

                if file {
                    file_index += 1;
                }
            });

        nodes
    }

}
