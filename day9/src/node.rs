#[derive(Debug, Clone)]
pub struct Node {
    pub id: Option<usize>,
    pub file: bool,
    pub size: usize,
    pub head: bool,
}

impl Node {
    pub fn list_from_feed(feed: &Vec<usize>) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];
        let mut file: bool = true;

        let mut file_index: usize = 0;
        feed
            .iter()
            .enumerate()
            .for_each(|(i, &size)| {
                file = (i + 1) % 2 != 0;

                for i in 0..size {
                    let id = if file {
                        Some(file_index)
                    } else {
                        None
                    };

                    let head = if file && i == 0 {
                        true
                    } else {
                        false
                    };


                    let node = Node {
                        id,
                        file,
                        size: size - i,
                        head,

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

