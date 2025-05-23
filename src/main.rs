/**
 * i4 - a grid-like navigator for i3wm
 */
extern crate i3ipc;
use core::fmt;

use i3ipc::reply::Node as I3Node;
use i3ipc::reply::NodeType as I3NodeType;
use i3ipc::I3Connection;
use I3NodeType::{Con as I3Con, Output as I3Output, Workspace as I3Workspace};

#[derive(Clone, Debug)]
pub struct Node {
    current: I3Node,
    parent: Option<Box<Node>>,
}

impl Node {
    fn new(node: &I3Node, parent: Option<Node>) -> Self {
        Node {
            current: node.clone(),
            parent: parent.map(Box::new),
        }
    }

    fn children<'s>(&'s self) -> Vec<Node> {
        self.current
            .nodes
            .iter()
            .map(|child_node| Node::new(child_node, Some(self.clone())))
            .collect()
    }

    fn parent(&self) -> Option<&Node> {
        self.parent.as_ref().map(|boxed_node| &**boxed_node)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\"id\": {}, \"nodetype\": \"{:?}\", \"name\": \"{:?}\", \"focused\": {}, \"rect\": {:?}, \"parent\": {}}}",
            self.current.id,
            self.current.nodetype,
            self.current.name,
            self.current.focused,
            self.current.rect,
            self.parent.as_ref().map(|p| p.current.id).unwrap_or(0)
        )
    }
}

macro_rules! dbg_node_opt {
    ($node:expr) => {
        match $node {
            Some(node) => format!("{}", node),
            None => "None".to_string(),
        }
    };
}

fn print_usage() {
    println!("Usage: i4 [-d] [-h] [-v] command [args]");
    println!("Options:");
    println!("  -d, --debug       Print debug information");
    println!("  -h, --help        Print this help message");
    println!("  -v, --version     Print version information");
    println!("Commands:");
    println!("  list             List nodes");
    println!("                   [all, focused, visible]");
    println!("  focus            Focus a window");
    println!("                   [left, right, up, down]");
    println!("  move             Move a window");
    println!("                   [left, right, up, down]");
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("Arguments: {:?}", args);
    if args.len() < 2 || (args.len() == 2 && (args[1] == "-h" || args[1] == "--help")) {
        print_usage();
        return;
    }

    if args[1] == "-v" || args[1] == "--version" {
        println!("i4 version 0.1.0");
        return;
    }

    if args[1] == "-d" || args[1] == "--debug" {
        println!("Debug mode enabled");
    }

    let mut connection = I3Connection::connect().unwrap();
    let i3tree = connection.get_tree().unwrap();
    let root_node = Node::new(&i3tree, None);

    match args[1].as_str() {
        "list" => {
            if args.len() < 3 {
                println!("Error: Missing argument for list command");
                return;
            }
            match args[2].as_str() {
                "all" => {
                    println!("Listing all nodes...");
                    print_i3_tree(&i3tree, 0);
                }
                "focused" => {
                    println!("Listing focused node...");
                    let focused_node = get_focused(&root_node);
                    if let Some(focused_node) = &focused_node {
                        println!("Focused node: {}", focused_node);
                        let parent_workspace = find_parent_workspace(focused_node);
                        println!(" | Parent workspace: {}", dbg_node_opt!(parent_workspace));
                        let parent_output = find_parent_output(focused_node);
                        println!(" | Parent output: {}", dbg_node_opt!(parent_output));
                    } else {
                        println!("No node in focus");
                    }
                }
                "visible" => println!("Listing visible nodes..."),
                _ => {
                    println!("Error: Unknown argument for list command");
                }
            }
        }
        "get" => {
            if args.len() < 3 {
                println!("Error: Missing argument for get command");
                return;
            }
            let focused_node = get_focused(&root_node);
            if let Some(focused_node) = &focused_node {
                println!("Focused node: {}", focused_node);
                match args[2].as_str() {
                    "left" => {
                        println!("Getting left node...");
                    }
                    "right" => {
                        println!("Getting right node...");
                    }
                    "up" => {
                        println!("Getting up node...");
                    }
                    "down" => {
                        println!("Getting down node...");
                    }
                    _ => {
                        println!("Error: Unknown argument for get command");
                    }
                }
            } else {
                println!("No node in focus");
            }
        }
        "focus" => {
            if args.len() < 3 {
                println!("Error: Missing argument for focus command");
                return;
            }
            match args[2].as_str() {
                "left" => {
                    println!("Focusing left...")
                }
                "right" => {
                    println!("Focusing right...")
                }
                "up" => {
                    println!("Focusing up...")
                }
                "down" => {
                    println!("Focusing down...")
                }
                _ => {
                    println!("Error: Unknown argument for focus command");
                }
            }
        }
        "move" => {
            if args.len() < 3 {
                println!("Error: Missing argument for move command");
                return;
            }
            match args[2].as_str() {
                "left" => {
                    println!("Moving left...")
                }
                "right" => {
                    println!("Moving right...")
                }
                "up" => {
                    println!("Moving up...")
                }
                "down" => {
                    println!("Moving down...")
                }
                _ => {
                    println!("Error: Unknown argument for move command");
                }
            }
        }
        _ => {
            println!("Error: Unknown command");
        }
    }
}

fn get_focused(node: &Node) -> Option<Node> {
    if node.current.focused {
        return Some(node.clone());
    }

    let children_of_current_node = node.children();

    for child_node in children_of_current_node {
        if let Some(focused_descendant) = get_focused(&child_node) {
            return Some(focused_descendant);
        }
    }

    None
}

fn find_parent_output(node: &Node) -> Option<Node> {
    if node.current.nodetype == I3Output {
        return Some(node.clone());
    }

    if let Some(parent) = node.parent() {
        return find_parent_output(parent);
    }

    None
}

fn find_parent_workspace(node: &Node) -> Option<Node> {
    if node.current.nodetype == I3Workspace {
        return Some(node.clone());
    }

    if let Some(parent) = node.parent() {
        return find_parent_workspace(parent);
    }

    None
}

fn print_i3_tree(node: &I3Node, depth: usize) {
    let indent = "    ".repeat(depth);
    println!("{}{{\"id\": {}, \"nodetype\": \"{:?}\", \"name\": \"{:?}\", \"focused\": {}, \"rect\": {:?}}}",
        indent,
        node.id,
        node.nodetype,
        node.name,
        node.focused,
        node.rect
    );

    for child in &node.nodes {
        print_i3_tree(child, depth + 1);
    }
}
