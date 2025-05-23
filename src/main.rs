/**
 * i4 - a grid-like navigator for i3wm
 */
extern crate i3ipc;
use core::fmt;

use i3ipc::reply::Node as I3Node;
use i3ipc::reply::NodeType as I3NodeType;
use i3ipc::I3Connection;
use I3NodeType::{Con as I3Con, Output as I3Output, Workspace as I3Workspace};

mod macros;

macro_rules! dbg_node_opt {
    ($node:expr) => {
        match $node {
            Some(node) => format!("{}", node),
            None => "None".to_string(),
        }
    };
}

macro_rules! dbg_vec_node {
    ($vec:expr) => {
        $vec.iter()
            .map(|node| format!("{}", node))
            .collect::<Vec<_>>()
            .join(", ")
    };
    () => {};
}

#[derive(Clone, Debug)]
pub struct Node {
    current: I3Node,
    parent: Option<Box<Node>>,
}

enum NodeType {
    Root,
    Output,
    DockArea,
    Workspace,
    Split,
    FloatingContainer,
    Window,
    Unknown,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Root => write!(f, "{}", style!("bold,blue", "Root")),
            NodeType::Output => write!(f, "{}", style!("bold,green", "Output")),
            NodeType::DockArea => write!(f, "{}", style!("bold,black", "DockArea")),
            NodeType::Workspace => write!(f, "{}", style!("bold,yellow", "Workspace")),
            NodeType::Split => write!(f, "{}", style!("bold,magenta", "Split")),
            NodeType::Window => write!(f, "{}", style!("bold,cyan", "Window")),
            NodeType::FloatingContainer => {
                write!(f, "{}", style!("bold,red", "FloatingContainer"))
            }
            NodeType::Unknown => write!(f, "{}", style!("bold,white", "Unknown")),
        }
    }
}

impl Node {
    fn new(node: &I3Node, parent: Option<Node>) -> Self {
        Node {
            current: node.clone(),
            parent: parent.map(Box::new),
        }
    }

    fn print(&self) {
        fn print_tree(node: &Node, depth: usize, is_last: bool) {
            let indent = if depth == 0 {
                String::new()
            } else {
                let prefix = if is_last { "    " } else { "│   " };
                format!(
                    "{}{}",
                    prefix.repeat(depth - 1),
                    if is_last { "└── " } else { "├── " }
                )
            };

            let mut node_info = format!(
                "{}\t{:?} {:?}",
                node.current.id, node.current.name, node.current.rect
            );
            if node.current.focused {
                node_info = style!("bold,white", "{}", node_info);
            } else {
                node_info = style!("dim,white", "{}", node_info);
            }

            println!("{}[{}] {}", indent, node.get_node_type(), node_info);

            let children = node.children();
            for (i, child) in children.iter().enumerate() {
                print_tree(child, depth + 1, i == children.len() - 1);
            }
        }

        let focused_node = self.get_focused();
        match focused_node {
            Some(focused_node) => {
                let focused_workspace = focused_node.get_parent_workspace();
                let focused_output = focused_node.get_parent_output();
            }
            None => {}
        }

        print_tree(self, 0, true);
    }

    fn get_node_type(&self) -> NodeType {
        match self.current.nodetype {
            I3NodeType::Root => NodeType::Root,
            I3NodeType::Output => NodeType::Output,
            I3NodeType::Workspace => NodeType::Workspace,
            I3NodeType::Con => {
                if self.current.window.is_some() {
                    NodeType::Window
                } else {
                    NodeType::Split
                }
            }
            I3NodeType::DockArea => NodeType::DockArea,
            I3NodeType::FloatingCon => NodeType::FloatingContainer,
            I3NodeType::Unknown => NodeType::Unknown,
        }
    }

    fn children(&self) -> Vec<Node> {
        self.current
            .nodes
            .iter()
            .map(|child_node| Node::new(child_node, Some(self.clone())))
            .collect()
    }

    fn previous(&self) -> Option<Node> {
        if let Some(parent) = &self.parent() {
            let _id = self.current.id;
            let siblings = parent.children();
            let index = siblings
                .iter()
                .position(|n| n.current.id == self.current.id);
            if let Some(index) = index {
                if index > 0 {
                    return Some(siblings[index - 1].clone());
                } else {
                    println!("No previous node");
                }
            } else {
                println!("Failed to find index ");
            }
        }
        None
    }

    fn next(&self) -> Option<Node> {
        if let Some(parent) = &self.parent() {
            let _id = self.current.id;
            let siblings = parent.children();
            let index = siblings
                .iter()
                .position(|n| n.current.id == self.current.id);
            if let Some(index) = index {
                if index < siblings.len() - 1 {
                    return Some(siblings[index + 1].clone());
                } else {
                    println!("No next node");
                }
            }
        }
        None
    }

    pub fn get_parent_workspace(&self) -> Option<Node> {
        if self.current.nodetype == I3Workspace {
            return Some(self.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.get_parent_workspace();
        }

        None
    }

    fn get_parent_output(&self) -> Option<Node> {
        if self.current.nodetype == I3Output {
            return Some(self.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.get_parent_output();
        }

        None
    }

    fn get_focused(&self) -> Option<Node> {
        if self.current.focused {
            return Some(self.clone());
        }

        for child in self.children() {
            if let Some(focused_child) = child.get_focused() {
                return Some(focused_child);
            }
        }

        None
    }

    fn get_windows(&self) -> Vec<Node> {
        let mut windows = Vec::new();
        fn collect_windows(node: &Node, windows: &mut Vec<Node>) {
            match node.get_node_type() {
                NodeType::Window => windows.push(node.clone()),
                _ => {}
            }
            for child in node.children() {
                collect_windows(&child, windows);
            }
        }
        collect_windows(self, &mut windows);
        windows
    }

    pub fn previous_window(&self) -> Option<Node> {
        fn find_last_window_in_subtree(node: &Node) -> Option<Node> {
            let mut stack = vec![node.clone()];
            let mut last_valid_window = None;

            while let Some(current) = stack.pop() {
                if current.current.nodetype == I3Con && current.current.window.is_some() {
                    last_valid_window = Some(current.clone());
                }

                let children = current.children();
                stack.extend(children);
                stack.reverse();
            }

            last_valid_window
        }

        let mut current = self.clone();
        let _id = current.current.id;

        while let Some(parent) = current.parent() {
            let siblings = parent.children();
            for i in (0..siblings.len()).rev() {
                if siblings[i].current.id == current.current.id {
                    if i > 0 {
                        return find_last_window_in_subtree(&siblings[i - 1]);
                    }
                }
            }
            current = parent.clone();
        }

        None
    }

    pub fn next_window(&self) -> Option<Node> {
        fn find_first_window_in_subtree(node: &Node) -> Option<Node> {
            let mut stack = vec![node.clone()];

            while let Some(current) = stack.pop() {
                if current.current.nodetype == I3Con && current.current.window.is_some() {
                    return Some(current);
                }

                let children = current.children();
                stack.extend(children.into_iter().rev());
            }

            None
        }

        let mut current = self.clone();
        let _id = current.current.id;

        while let Some(parent) = current.parent() {
            let siblings = parent.children();
            for i in 0..siblings.len() {
                if siblings[i].current.id == current.current.id {
                    if i + 1 < siblings.len() {
                        return find_first_window_in_subtree(&siblings[i + 1]);
                    }
                }
            }
            current = parent.clone();
        }

        None
    }

    fn parent(&self) -> Option<&Node> {
        self.parent.as_ref().map(|boxed_node| &**boxed_node)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node_type = self.get_node_type();
        let node_id = self.current.id;
        let node_name = match self.current.name.as_ref() {
            Some(name) => String::from(name),
            None => "".to_string(),
        };
        let parent_workspace = match self.get_parent_workspace() {
            Some(parent) => match parent.current.name {
                Some(name) => name,
                None => "".to_string(),
            },
            None => "".to_string(),
        };
        let parent_output = match self.get_parent_output() {
            Some(parent) => match parent.current.name {
                Some(name) => name,
                None => "".to_string(),
            },
            None => "".to_string(),
        };

        write!(
            f,
            "[{}] {} \"{}\" {:?} {} {}",
            node_type, node_id, node_name, self.current.rect, parent_workspace, parent_output
        )
    }
}

fn print_usage() {
    println!(
        "{} A grid like navigator for i3wm",
        style!("bold,blue", "i4")
    );
    println!("Usage: i4 [-d] [-h] [-v] command [args]");
    println!("Options:");
    println!("  -d, --debug       Print debug information");
    println!("  -h, --help        Print this help message");
    println!("  -v, --version     Print version information");
    println!("Commands:");
    println!("  list             List nodes");
    println!("                   [all, focused, visible]");
    println!("  focus            Focus a window");
    println!("                   [left, right, up, down, previous, next]");
    println!("  move             Move a window");
    println!("                   [left, right, up, down, previous, next]");
}

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();
    let mut debug_mode = false;

    if args.len() < 2 || (args.len() == 2 && (args[1] == "-h" || args[1] == "--help")) {
        print_usage();
        return;
    }

    if args[1] == "-v" || args[1] == "--version" {
        println!("i4 version 0.1.0");
        return;
    }

    if args[1] == "-d" || args[1] == "--debug" {
        args.remove(1);
        debug_mode = true;
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
                    if debug_mode {
                        println!("I3 tree: {:#?}", i3tree);
                    } else {
                        root_node.print();
                    }
                }
                "focused" => {
                    println!("Listing focused node...");
                    let focused_node = root_node.get_focused();
                    if let Some(focused_node) = &focused_node {
                        println!("Focused node: {}", focused_node);
                        let parent_workspace = focused_node.get_parent_workspace();
                        println!(" | Parent workspace: {}", dbg_node_opt!(parent_workspace));
                        let parent_output = focused_node.get_parent_output();
                        println!(" | Parent output: {}", dbg_node_opt!(parent_output));
                        println!(
                            " | Previous window: {}",
                            dbg_node_opt!(focused_node.previous_window())
                        );
                        println!(
                            " | Next node: {}",
                            dbg_node_opt!(focused_node.next_window())
                        );
                    } else {
                        println!("No node in focus");
                    }
                }
                "visible" => println!("Listing visible nodes..."),
                "windows" => {
                    println!("Listing windows...");
                    let windows = root_node.get_windows();
                    for window in windows {
                        println!("{}", window);
                    }
                }
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
            let focused_node = root_node.get_focused();
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
            let focused_node = root_node.get_focused();
            if let Some(focused_node) = &focused_node {
                println!("Focused node: {}", focused_node);
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
                    "previous" => {
                        if let Some(previous_node) = focused_node.previous_window() {
                            println!("Previous node: {}", previous_node);
                            connection
                                .run_command(&format!(
                                    "[con_id={}] focus",
                                    previous_node.current.id
                                ))
                                .unwrap();
                        } else {
                            println!("No previous node");
                        }
                    }
                    "next" => {
                        if let Some(next_node) = focused_node.next_window() {
                            println!("Next node: {}", next_node);
                            connection
                                .run_command(&format!("[con_id={}] focus", next_node.current.id))
                                .unwrap();
                        } else {
                            println!("No next node");
                        }
                    }
                    _ => {
                        println!("Error: Unknown argument for focus command");
                    }
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
