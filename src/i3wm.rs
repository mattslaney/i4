use i3ipc::reply::Node as I3Node;

/*
pub struct Node { // is I3Node

    pub focus: Vec<i64>,
    pub nodes: Vec<Node>,
    pub floating_nodes: Vec<Node>,
    pub id: i64,
    pub name: Option<String>,
    pub nodetype: NodeType,
    pub border: NodeBorder,
    pub current_border_width: i32,
    pub layout: NodeLayout,
    pub percent: Option<f64>,
    pub rect: (i32, i32, i32, i32),
    pub window_rect: (i32, i32, i32, i32),
    pub deco_rect: (i32, i32, i32, i32),
    pub geometry: (i32, i32, i32, i32),
    pub window: Option<i32>,
    pub window_properties: Option<HashMap<WindowProperty, String>>,
    pub urgent: bool,
    pub focused: bool,
}
 */
use i3ipc::reply::NodeType as I3NodeType;
/*
pub enum NodeType { //Is I3NodeType
    Root,
    Output,
    Con,
    FloatingCon,
    Workspace,
    DockArea,
    Unknown,
}
 */
use i3ipc::reply::{Output as I3Output, Outputs as I3Outputs};
/*
pub struct Output { // is I3Output
    pub name: String,
    pub active: bool,
    pub primary: bool,
    pub current_workspace: Option<String>,
    pub rect: (i32, i32, i32, i32),
}
pub struct Outputs { // is I3Outputs
    pub outputs: Vec<Output>,
}
     */
use i3ipc::reply::{Workspace as I3Workspace, Workspaces as I3Workspaces};
/*
pub struct Workspace { //is I3Workspace
    pub num: i32,
    pub name: String,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub rect: (i32, i32, i32, i32),
    pub output: String,
}
pub struct Workspaces { //is I3Workspaces
    pub workspaces: Vec<Workspace>,
}
*/
use i3ipc::I3Connection;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn as_str(&self) -> &str {
        match self {
            Direction::Left => "left",
            Direction::Right => "right",
            Direction::Up => "up",
            Direction::Down => "down",
        }
    }
}

pub struct Root {
    outputs: I3Outputs,
    workspaces: I3Workspaces,
    pub node: I3Node,
    size: i32, // This is the maximum number of horizontal workspaces
}

pub trait I3ConnectionTrait {
    type ResultSuccess;
    type ResultError: std::fmt::Debug;
    type EstablishError: std::fmt::Debug;

    fn connect(path: Option<std::path::PathBuf>) -> Result<Self, Self::EstablishError>
    where
        Self: Sized;
    fn run_command(&mut self, command: &str) -> Result<Self::ResultSuccess, Self::ResultError>;
    fn get_outputs(&mut self) -> Result<i3ipc::reply::Outputs, Self::ResultError>;
    fn get_workspaces(&mut self) -> Result<i3ipc::reply::Workspaces, Self::ResultError>;
    fn get_tree(&mut self) -> Result<i3ipc::reply::Node, Self::ResultError>;
}

impl I3ConnectionTrait for I3Connection {
    type ResultSuccess = i3ipc::reply::Command;
    type ResultError = i3ipc::MessageError;
    type EstablishError = i3ipc::EstablishError;

    fn connect(path: Option<std::path::PathBuf>) -> Result<Self, Self::EstablishError> {
        I3Connection::connect()
    }

    fn run_command(&mut self, command: &str) -> Result<Self::ResultSuccess, Self::ResultError> {
        self.run_command(command)
    }

    fn get_outputs(&mut self) -> Result<i3ipc::reply::Outputs, Self::ResultError> {
        self.get_outputs()
    }

    fn get_workspaces(&mut self) -> Result<i3ipc::reply::Workspaces, Self::ResultError> {
        self.get_workspaces()
    }

    fn get_tree(&mut self) -> Result<i3ipc::reply::Node, Self::ResultError> {
        self.get_tree()
    }
}

pub struct Util<C: I3ConnectionTrait> {
    connection: C,
}

impl<C: I3ConnectionTrait> Util<C> {
    pub fn connect(path: Option<std::path::PathBuf>) -> Self {
        match path {
            Some(path) => Util {
                connection: C::connect(Some(path)).expect("Failed to connect to i3"),
            },
            None => Util {
                connection: C::connect(None).expect("Failed to connect to i3"),
            },
        }
    }

    pub fn get_root(&mut self, size: i32) -> Root {
        let outputs = self
            .connection
            .get_outputs()
            .expect("Failed to get outputs");
        let workspaces = self
            .connection
            .get_workspaces()
            .expect("Failed to get workspaces");
        let node = self.connection.get_tree().expect("Failed to get i3 tree");
        Root::new(outputs, workspaces, node, size)
    }

    //https://i3wm.org/docs/userguide.html#list_of_commands
    pub fn focus_window(&mut self, window: Window) {
        println!("Focus window: {}", window.node.id);
        self.connection
            .run_command(&format!("[con_id={}] focus", window.node.id))
            .expect("Failed to focus window");
    }

    pub fn focus_workspace(&mut self, workspace: &String) {
        self.connection
            .run_command(&format!("workspace {}", workspace))
            .expect("Failed to focus workspace");
    }

    pub fn create_workspace(&mut self, name: &String) {
        println!("Create workspace: {}", name);
        self.connection
            .run_command(&format!("workspace {}", name))
            .expect("Failed to create workspace");
    }

    pub fn move_window(&mut self, direction: Direction) {
        self.connection
            .run_command(&format!("move {}", direction.as_str()))
            .expect("Failed to move window");
    }

    pub fn move_window_to_workspace(&mut self, workspace_name: &String) {
        self.connection
            .run_command(&format!("move container to workspace {}", workspace_name))
            .expect("Failed to focus window");
    }
}

impl Root {
    pub fn new(outputs: I3Outputs, workspaces: I3Workspaces, node: I3Node, size: i32) -> Self {
        Root {
            outputs: outputs,
            workspaces: workspaces,
            node: node,
            size: size,
        }
    }

    fn get_active_outputs(&self) -> Vec<Output> {
        let mut outputs = Vec::new();
        for output in self.outputs.outputs.iter() {
            if output.active {
                if let Some(node) = Root::get_matching_output_node(&self.node, output) {
                    outputs.push(Output::new(
                        output,
                        node.clone(),
                        self.get_output_workspaces(output),
                        self.size,
                    ));
                }
            }
        }
        outputs
    }

    pub fn get_focused_output(&self) -> Option<Output> {
        for output in self.get_active_outputs() {
            if output.get_focused_workspace().is_some() {
                return Some(output);
            }
        }
        None
    }

    fn get_output_workspaces(&self, output: &I3Output) -> Vec<Workspace> {
        let mut workspaces = Vec::new();
        for workspace in self.workspaces.workspaces.iter() {
            if workspace.output == output.name {
                if let Some(node) = Root::get_matching_workspace_node(&self.node, workspace) {
                    workspaces.push(Workspace::new(workspace, node));
                }
            }
        }
        workspaces
    }

    fn get_visible_workspaces(&self) -> Vec<Workspace> {
        let mut workspaces = Vec::new();
        for workspace in self.workspaces.workspaces.iter() {
            if workspace.visible {
                if let Some(node) = Root::get_matching_workspace_node(&self.node, workspace) {
                    workspaces.push(Workspace::new(workspace, node));
                }
            }
        }
        workspaces
    }

    fn get_visible_windows(&self, node: Option<I3Node>) -> Vec<Window> {
        let node = node.unwrap_or(self.node.clone());
        let mut windows = Vec::new();
        fn dfs(root: &Root, node: &I3Node, windows: &mut Vec<Window>) {
            if node.nodetype == I3NodeType::Workspace {
                for workspace in root.workspaces.workspaces.iter() {
                    if Some(workspace.name.clone()) == node.name && workspace.visible == false {
                        return;
                    }
                }
            }
            if node.nodetype == I3NodeType::Con && node.window.is_some() {
                windows.push(Window::new(node.clone()));
            }
            for child in &node.nodes {
                dfs(root, child, windows);
            }
        }
        dfs(self, &node, &mut windows);
        windows
    }

    fn get_matching_output_node<'a>(node: &'a I3Node, output: &'a I3Output) -> Option<&'a I3Node> {
        if node.nodetype == I3NodeType::Output && node.name == Some(output.name.clone()) {
            return Some(&node);
        }

        if node.nodes.len() > 0 {
            for child in node.nodes.iter() {
                if let Some(matching_node) = Root::get_matching_output_node(child, output) {
                    return Some(matching_node);
                }
            }
        }

        None
    }

    fn get_matching_workspace_node<'a>(
        node: &'a I3Node,
        workspace: &'a I3Workspace,
    ) -> Option<&'a I3Node> {
        if node.nodetype == I3NodeType::Workspace && node.name == Some(workspace.name.clone()) {
            return Some(&node);
        }

        if node.nodes.len() > 0 {
            for child in node.nodes.iter() {
                if let Some(matching_node) = Root::get_matching_workspace_node(child, workspace) {
                    return Some(matching_node);
                }
            }
        }

        None
    }
}

pub struct Output<'a> {
    pub data: &'a I3Output,
    pub node: I3Node,
    workspaces: Vec<Workspace<'a>>,
    size: i32, // This is the maximum number of horizontal workspaces
}

impl<'a> Output<'a> {
    fn new(output: &'a I3Output, node: I3Node, workspaces: Vec<Workspace<'a>>, size: i32) -> Self {
        Output {
            data: output,
            node: node,
            workspaces: workspaces,
            size: size,
        }
    }

    pub fn get_focused_workspace(&self) -> Option<Workspace> {
        for workspace in self.workspaces.iter() {
            if workspace.is_focused() {
                return Some(workspace.clone());
            }
        }
        None
    }

    fn get_workspaces(&self) -> Vec<Workspace> {
        self.workspaces.clone()
    }

    pub fn get_previous_workspace(&self) -> Option<Workspace> {
        if self.get_focused_workspace().is_some() {
            let workspaces = self.get_workspaces();
            let index = workspaces.iter().position(|w| w.is_focused());
            if let Some(i) = index {
                if i > 1 {
                    return Some(workspaces[i - 1].clone());
                }
            }
        }
        None
    }

    pub fn get_next_workspace(&self) -> Option<Workspace> {
        if self.get_focused_workspace().is_some() {
            let workspaces = self.get_workspaces();
            let index = workspaces.iter().position(|w| w.is_focused());
            if let Some(i) = index {
                if i < workspaces.len() - 1 {
                    return Some(workspaces[i + 1].clone());
                }
            }
        }
        None
    }

    pub fn get_adjacent_workspace(&self, direction: Direction) -> Option<Workspace> {
        if let Some(current_workspace) = self.get_focused_workspace() {
            let current_name = current_workspace.data.name.parse::<i32>().unwrap_or(-1);
            match direction {
                Direction::Left => {
                    for workspace in self.workspaces.iter() {
                        if let Ok(name) = workspace.data.name.parse::<i32>() {
                            if name == current_name - 1 {
                                return Some(workspace.clone());
                            }
                        }
                    }
                }
                Direction::Right => {
                    for workspace in self.workspaces.iter() {
                        if let Ok(name) = workspace.data.name.parse::<i32>() {
                            if name == current_name + 1 {
                                return Some(workspace.clone());
                            }
                        }
                    }
                }
                Direction::Up => {
                    for workspace in self.workspaces.iter() {
                        if let Ok(name) = workspace.data.name.parse::<i32>() {
                            if name == current_name - self.size as i32 {
                                return Some(workspace.clone());
                            }
                        }
                    }
                }
                Direction::Down => {
                    for workspace in self.workspaces.iter() {
                        if let Ok(name) = workspace.data.name.parse::<i32>() {
                            if name == current_name + self.size as i32 {
                                return Some(workspace.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Workspace<'a> {
    pub data: &'a I3Workspace,
    pub node: &'a I3Node,
}

impl<'a> Workspace<'a> {
    fn new(workspace: &'a I3Workspace, node: &'a I3Node) -> Self {
        Workspace {
            data: workspace,
            node: node,
        }
    }

    fn is_focused(&self) -> bool {
        self.data.focused
    }

    fn is_visible(&self) -> bool {
        self.data.visible
    }

    pub fn get_focused_window(&self) -> Option<Window> {
        fn dfs(node: &I3Node) -> Option<Window> {
            if node.focused && node.nodetype == I3NodeType::Con && node.window.is_some() {
                return Some(Window::new(node.clone()));
            }
            for child in &node.nodes {
                if let Some(found_window) = dfs(child) {
                    return Some(found_window);
                }
            }
            None
        }
        dfs(self.node)
    }

    fn get_windows(&self) -> Vec<Window> {
        let mut windows = Vec::new();
        fn dfs(node: &I3Node, windows: &mut Vec<Window>) {
            if node.nodetype == I3NodeType::Con && node.window.is_some() {
                windows.push(Window::new(node.clone()));
            }
            for child in &node.nodes {
                dfs(child, windows);
            }
        }
        dfs(self.node, &mut windows);
        windows
    }

    pub fn get_next_window(&self) -> Option<Window> {
        if let Some(focused_window) = self.get_focused_window() {
            let windows = self.get_windows();
            let index = windows
                .iter()
                .position(|w| w.node.id == focused_window.node.id);
            if let Some(i) = index {
                if i < windows.len() - 1 {
                    return Some(windows[i + 1].clone());
                }
            }
        }
        None
    }

    pub fn get_previous_window(&self) -> Option<Window> {
        if let Some(focused_window) = self.get_focused_window() {
            let windows = self.get_windows();
            let index = windows
                .iter()
                .position(|w| w.node.id == focused_window.node.id);
            if let Some(i) = index {
                if i > 0 {
                    return Some(windows[i - 1].clone());
                }
            }
        }
        None
    }

    pub fn get_adjacent_window(&self, direction: Direction) -> Option<Window> {
        if let Some(focused_window) = self.get_focused_window() {
            let windows = self.get_windows();
            let (f_x, f_y, f_w, f_h) = focused_window.node.rect.clone();
            for window in windows.iter() {
                if window.node.id != focused_window.node.id {
                    let (n_x, n_y, n_w, n_h) = window.node.rect.clone();
                    match direction {
                        Direction::Left => {
                            if n_x + n_w < f_x {
                                return Some(window.clone());
                            }
                        }
                        Direction::Right => {
                            if n_x > f_x + f_w {
                                return Some(window.clone());
                            }
                        }
                        Direction::Up => {
                            if n_y + n_h < f_y {
                                return Some(window.clone());
                            }
                        }
                        Direction::Down => {
                            if n_y > f_y + f_h {
                                return Some(window.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

pub struct Container {
    pub node: I3Node,
}

impl Container {
    fn new(node: I3Node) -> Self {
        Container { node: node }
    }
}

#[derive(Clone)]
pub struct Window {
    pub node: I3Node,
}

impl Window {
    fn new(node: I3Node) -> Self {
        Window { node: node }
    }
}

#[cfg(test)]
mod tests {
    pub use crate::tests::mock_i3ipc;
    use std::str::FromStr;

    use super::*;

    use mock_i3ipc::MockI3Connection;

    #[test]
    fn get_active_outputs_returns_all_active_outputs() {
        let testfiles = std::path::PathBuf::from_str("src/tests/scenarios/basic")
            .expect("Failed to parse path");
        let mut i3 = MockI3Connection::connect(testfiles).expect("Failed to connect to mock i3");
        let mock_outputs = i3.get_outputs().expect("Failed to get mock outputs");
        let mock_workspaces = i3.get_workspaces().expect("Failed to get mock workspaces");
        let mock_tree = i3.get_tree().expect("Failed to get mock tree");

        let root_node = Root::new(mock_outputs, mock_workspaces, mock_tree, 10);

        let active_outputs: Vec<Output> = root_node.get_active_outputs();
        assert_eq!(active_outputs.len(), 2);
        assert_eq!(active_outputs[0].data.name, "DP-1-0");
        assert_eq!(active_outputs[1].data.name, "eDP-1");
    }
}
