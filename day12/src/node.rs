use crate::coordinate::ICoordinate;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub index: ICoordinate,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(coordinate: &ICoordinate) -> Node {
        Node {
            index: coordinate.clone(),
            children: Vec::new(),
        }
    }

    pub fn is_adjacent(coordinate_a: &ICoordinate, coordinate_b: &ICoordinate) -> bool {
        let x_diff = (coordinate_a.x - coordinate_b.x).abs();
        let y_diff = (coordinate_a.y - coordinate_b.y).abs();
        x_diff + y_diff == 1
    }

    pub fn insert(&mut self, coordinate: &ICoordinate) -> bool {
        let parent_cord_a = ICoordinate {
            x: coordinate.x - 1,
            y: coordinate.y,
        };
        let parent_cord_b = ICoordinate {
            x: coordinate.x + 1,
            y: coordinate.y,
        };
        if (self.index == parent_cord_a || self.index == parent_cord_b)
            && !self.children.contains(&Node::new(coordinate))
        {
            self.children.push(Node::new(coordinate));
            return true;
        }

        if let Some(parent_node) = self.traverse(coordinate) {
            parent_node.children.push(Node::new(&coordinate));
            return true;
        }

        false
    }

    pub fn traverse(&mut self, coordinate: &ICoordinate) -> Option<&mut Node> {
        let parent_cord_a = ICoordinate {
            x: coordinate.x - 1,
            y: coordinate.y,
        };
        let parent_cord_b = ICoordinate {
            x: coordinate.x + 1,
            y: coordinate.y,
        };
        if (self.index == parent_cord_a || self.index == parent_cord_b)
            && !self.children.contains(&Node::new(coordinate))
        {
            return Some(self);
        }

        for child in self.children.iter_mut() {
            if let Some(parent_node) = child.traverse(coordinate) {
                if (parent_node.index == parent_cord_a || parent_node.index == parent_cord_b)
                    && !parent_node.children.contains(&Node::new(coordinate))
                {
                    return Some(parent_node);
                }
            }
        }

        None
    }
}
