extern crate i3ipc;
use core::fmt;

use i3ipc::I3Connection;

#[derive(Clone, Debug)]
pub struct Node<'a, 'p> {
    current: &'a i3ipc::reply::Node,
    parent: Option<&'p Node<'a, 'p>>,
}

impl<'a, 'p> Node<'a, 'p> {
    pub fn new(node: &'a i3ipc::reply::Node, parent: Option<&'p Node<'a, 'p>>) -> Self {
        Node {
            current: node,
            parent,
        }
    }

    pub fn children<'s>(&'s self) -> Vec<Node<'a, 's>>
    where
        'p: 's,
    {
        self.current
            .nodes
            .iter()
            .map(|child_node| Node::new(child_node, Some(self)))
            .collect()
    }

    pub fn parent(&self) -> Option<&'p Node<'a, 'p>> {
        self.parent
    }
}

impl fmt::Display for Node<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\"id\": {}, \"nodetype\": \"{:?}\", \"name\": \"{:?}\", \"focused\": {}, \"rect\": {:#?}, \"parent\": {}}}",
            self.current.id,
            self.current.nodetype,
            self.current.name,
            self.current.focused,
            self.current.rect,
            self.parent.unwrap().current.id
        )
    }
}

fn main() {
    let mut connection = I3Connection::connect().unwrap();
    let i3tree = connection.get_tree().unwrap();
    let root_node = Node::new(&i3tree, None);
    //println!("Root Node: {:#?}", root_node);

    if let Some(focused_node) = get_focused(&root_node) {
        println!("Focused Node: {}", focused_node);
        println!("Focused Node's Parent: {}", focused_node.parent().unwrap());
    } else {
        println!("No focused node found by get_focused.");
    }
}

fn get_focused<'a, 'p>(node: &'p Node<'a, 'p>) -> Option<Node<'a, 'p>> {
    if node.current.focused {
        println!("Found focused node: {}", node);
        println!("Found focused node's parent: {}", node.parent().unwrap());
        return Some(node.clone());
    }

    let children_of_current_node = &node.children();

    for child_node in children_of_current_node.iter() {
        if let Some(focused_descendant) = get_focused(child_node) {
            return Some(focused_descendant);
        }
    }

    // for child_node_wrapper in children_of_current_node.iter() {
    //     if let Some(focused_descendant) = get_focused(child_node_wrapper) {
    //         return Some(focused_descendant);
    //         //return Some(Node::new(focused_descendant.current, Some(node)));
    //     }
    // }
    None
}
