pub mod i3wm {
    extern crate i3ipc;

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

    pub struct Root {
        outputs: I3Outputs,
        workspaces: I3Workspaces,
        pub node: I3Node,
        size: i32, // This is the maximum number of horizontal workspaces
    }

    impl Root {
        pub fn new(size: i32) -> Self {
            let mut connection = I3Connection::connect().expect("Failed to connect to i3");
            let outputs = connection.get_outputs().expect("Failed to get outputs");
            let workspaces = connection
                .get_workspaces()
                .expect("Failed to get workspaces");
            let node = connection.get_tree().expect("Failed to get i3 tree");
            Root {
                outputs: outputs,
                workspaces: workspaces,
                node: node,
                size: size,
            }
        }

        // TODO: This should return an instance of Output (I3Output/I3Node) for all outputs (monitors) that are active (turned on)
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

        // TODO: This should return an instance of Output (I3Output/I3Node) output (monitor) which contains the current focused I3Workspace/I3Node
        pub fn get_focused_output(&self) -> Option<Output> {
            for output in self.get_active_outputs() {
                if output.get_focused_workspace().is_some() {
                    return Some(output);
                }
            }
            None
        }

        //TODO: This should return a vector of Workspace (I3Workspace/I3Node) instances for all workspaces that exist on the output (monitor) provided as a parameter
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

        // TODO: This should return a vector of Workspace (I3Workspace/I3Node) instances that are currently visible in an i3 tree
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

        //TODO: This should return a vector of Window (I3Node) instances that are currently visible in an i3 tree
        fn get_visible_windows(&self, node: Option<I3Node>) -> Vec<Window> {
            let node = node.unwrap_or(self.node.clone());
            let mut windows = Vec::new();
            fn dfs(node: &I3Node, windows: &mut Vec<Window>) {
                if node.focused && node.nodetype == I3NodeType::Con && node.window.is_some() {
                    windows.push(Window::new(node.clone()));
                }
                for child in &node.nodes {
                    dfs(child, windows);
                }
            }
            dfs(&node, &mut windows);
            windows
        }

        // TODO: This should return the matching I3Node for the supplied I3Output. I3 output just contains data about the output (monitor) and not the actual I3Node. Can be linked together by name (e.g. eDP-1, HDMI-1, etc.)
        fn get_matching_output_node<'a>(
            node: &'a I3Node,
            output: &'a I3Output,
        ) -> Option<&'a I3Node> {
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

        //TODO: This should return the matching I3Node for the supplied I3Workspace. I3 workspace just contains data about the workspace and not the actual I3Node. Can be linked together by name (e.g. 1, 2, 3, etc.)
        fn get_matching_workspace_node<'a>(
            node: &'a I3Node,
            workspace: &'a I3Workspace,
        ) -> Option<&'a I3Node> {
            if node.nodetype == I3NodeType::Workspace && node.name == Some(workspace.name.clone()) {
                return Some(&node);
            }

            if node.nodes.len() > 0 {
                for child in node.nodes.iter() {
                    if let Some(matching_node) = Root::get_matching_workspace_node(child, workspace)
                    {
                        return Some(matching_node);
                    }
                }
            }

            None
        }

        //https://i3wm.org/docs/userguide.html#list_of_commands
        pub fn focus_window(&self, window: Window) {
            println!("Focus window: {}", window.node.id);
            let mut connection = I3Connection::connect().expect("Failed to connect to i3");
            connection
                .run_command(&format!("[con_id={}] focus", window.node.id))
                .expect("Failed to focus window");
        }

        pub fn focus_workspace(&self, workspace: Workspace) {
            println!("Focus workspace: {}", workspace.data.name);
            let mut connection = I3Connection::connect().expect("Failed to connect to i3");
            connection
                .run_command(&format!("workspace {}", workspace.data.name))
                .expect("Failed to focus workspace");
        }

        pub fn create_workspace(&self, name: String) {
            println!("Create workspace: {}", name);
            let mut connection = I3Connection::connect().expect("Failed to connect to i3");
            connection
                .run_command(&format!("workspace {}", name))
                .expect("Failed to create workspace");
        }

        pub fn move_window_to_workspace(&self, window: Window, workspace: Workspace) {
            println!(
                "Move window: {} to workspace: {}",
                window.node.id, workspace.data.name
            );
            let mut connection = I3Connection::connect().expect("Failed to connect to i3");
            connection
                .run_command(&format!(
                    "move [con_id={}] to workspace {}",
                    window.node.id, workspace.data.name
                ))
                .expect("Failed to focus window");
        }
    }

    pub struct Output<'a> {
        pub data: &'a I3Output,
        pub node: I3Node,
        workspaces: Vec<Workspace<'a>>,
        size: i32, // This is the maximum number of horizontal workspaces
    }

    impl<'a> Output<'a> {
        fn new(
            output: &'a I3Output,
            node: I3Node,
            workspaces: Vec<Workspace<'a>>,
            size: i32,
        ) -> Self {
            Output {
                data: output,
                node: node,
                workspaces: workspaces,
                size: size,
            }
        }

        // TODO: This should return an instance of Workspace (I3Workspace/I3Node) which is currently focused on this output (monitor)
        pub fn get_focused_workspace(&self) -> Option<Workspace> {
            for workspace in self.workspaces.iter() {
                if workspace.is_focused() {
                    return Some(workspace.clone());
                }
            }
            None
        }

        // TODO: This should return a vector of Workspace (I3Workspace/I3Node) instances that exist on this output (monitor)
        fn get_workspaces(&self) -> Vec<Workspace> {
            self.workspaces.clone()
        }

        // TODO: This should return the previous workspace (I3Workspace/I3Node) on this output (monitor)
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

        // TODO: This should return the next workspace (I3Workspace/I3Node) on this output (monitor)
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

        // TODO: This should return the adjacent workspace (I3Workspace/I3Node) in the specified direction (left, right, up, down)
        pub fn get_adjacent_workspace(&self, direction: Direction) -> Option<Workspace> {
            if let Some(current_workspace) = self.get_focused_workspace() {
                let current_name = current_workspace.data.name.parse::<i32>().unwrap_or(-1);
                match direction {
                    Direction::Left => {
                        // TODO: Should find the workspace in self.workspaces with a name -1 more than the current workspace name
                        for workspace in self.workspaces.iter() {
                            if let Ok(name) = workspace.data.name.parse::<i32>() {
                                if name == current_name - 1 {
                                    return Some(workspace.clone());
                                }
                            }
                        }
                    }
                    Direction::Right => {
                        // TODO: Should find the workspace in self.workspaces with a name +1 more than the current workspace name
                        for workspace in self.workspaces.iter() {
                            if let Ok(name) = workspace.data.name.parse::<i32>() {
                                if name == current_name + 1 {
                                    return Some(workspace.clone());
                                }
                            }
                        }
                    }
                    Direction::Up => {
                        // TODO: Should find the workspace in self.workspaces with a name -100 more than the current workspace name
                        for workspace in self.workspaces.iter() {
                            if let Ok(name) = workspace.data.name.parse::<i32>() {
                                if name == current_name - self.size as i32 {
                                    return Some(workspace.clone());
                                }
                            }
                        }
                    }
                    Direction::Down => {
                        // TODO: Should find the workspace in self.workspaces with a name +100 more than the current workspace name
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

        //TODO: Should return the focused window (I3Node) in this workspace
        pub fn get_focused_window(&self) -> Option<Window> {
            fn dfs(node: &I3Node) -> Option<Window> {
                if node.focused && node.nodetype == I3NodeType::Con && node.window.is_some() {
                    return Some(Window::new(node.clone()));
                }
                for child in &node.nodes {
                    return dfs(child);
                }
                None
            }
            dfs(self.node)
        }

        // TODO: Should return a vector of all windows (I3Node) in this workspace
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

        // TODO: Should return the next window (I3Node) in this workspace
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

        // TODO: Should return the previous window (I3Node) in this workspace
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

        // TODO: Should return the adjacent window (I3Node) in the specified direction (left, right, up, down) based on the window rect (x, y, width, height)
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
        use super::*;

        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }
    }
}
