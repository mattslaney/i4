pub mod I3WM {
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

    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    pub struct Root {
        connection: I3Connection,
        outputs: I3Outputs,
        workspaces: I3Workspaces,
        root_node: I3Node,
    }

    impl Root {
        pub fn new(
            connection: I3Connection,
            outputs: I3Outputs,
            workspaces: I3Workspaces,
            root_node: I3Node,
        ) -> Self {
            Root {
                connection: connection,
                outputs: outputs,
                workspaces: workspaces,
                root_node: root_node,
            }
        }

        //TODO: Will use self.outputs to find all outputs (monitors) that are active (on)
        fn get_active_outputs(&self) -> Vec<Output> {
            let mut outputs = Vec::new();
            for output in self.outputs.outputs.iter() {
                if output.active {
                    if let Some(node) = Root::get_matching_output_node(output, &self.root_node) {
                        outputs.push(Output::new(output, node, Root::get_output_workspaces()));
                    }
                }
            }
            outputs
        }

        //TODO: Will get the active outputs and work out if it's current workspace is in focus
        fn get_focused_output(&self) -> Option<Output> {
            for output in self.get_active_outputs() {
                if output.get_focused_workspace().is_some() {
                    return Some(output);
                }
            }
            None
        }

        //TODO: The I3Output has a matching I3Node, this method will find and return it
        fn get_matching_output_node<'a>(
            &self,
            output: &'a I3Output,
        ) -> Option<&'a I3Node> {
            if root_node.nodetype == I3NodeType::Output
                && root_node.name == Some(output.name.clone())
            {
                return Some(root_node);
            }

            if root_node.nodes.len() > 0 {
                for child in root_node.nodes.iter() {
                    if let Some(matching_node) = Root::get_matching_output_node(child, output) {
                        return Some(matching_node);
                    }
                }
            }

            None
        }

        fn get_output_workspaces(&self) -> Vec<Workspace> {
            let mut workspaces = Vec::new();
            for workspace in self.workspaces.workspaces.iter() {
                Root::get

        // TODO: The I3Workspace has a matching I3Node, this method will find and return it
        fn get_matching_workspace_node<'a>(
            &self,
            workspace: &'a I3Workspace
        ) -> Option<&'a I3Node> {
            if root_node.nodetype == I3NodeType::Workspace
                && root_node.name == Some(workspace.name.clone())
            {
                return Some(root_node);
            }

            if root_node.nodes.len() > 0 {
                for child in root_node.nodes.iter() {
                    if let Some(matching_node) = get_matching_workspace_node(child, workspace)
                    {
                        return Some(matching_node);
                    }
                }
            }

            None
        }
    }

    pub struct Output<'a> {
        data: &'a I3Output,
        node: &'a I3Node,
        workspaces: Vec<Workspace>,
    }

    impl<'a> Output<'a> {
        fn new(output: &'a I3Output, node: &'a I3Node, workspaces: Vec<Workspace>) -> Self {
            Output {
                data: output,
                node: node,
                workspaces: workspaces,
            }
        }

        fn get_focused_workspace(&self, workspaces: I3Workspaces) -> Option<Workspace> {}

        fn get_visible_workspaces(&self) -> Vec<Workspace> {
            let mut workspaces = Vec::new();
        }

        // fn get_workspaces(&self) -> Vec<&Workspace> {
        //     let mut workspaces = Vec::new();
        //     for node in self.node.nodes.iter() {
        //         if node.nodetype == I3NodeType::Workspace {
        //             workspaces.push(Workspace::new(

        //     workspaces
        // }

        fn get_previous_workspace(&self) -> Option<&Workspace> {
            None
        }

        fn get_next_workspace(&self) -> Option<&Workspace> {
            None
        }

        fn get_adjacent_workspace(&self, direction: Direction) -> Option<&Workspace> {
            None
        }
    }

    pub struct Workspace {
        data: I3Workspace,
        node: I3Node,
    }

    impl Workspace {
        fn new(workspace: I3Workspace, node: I3Node) -> Self {
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

        fn get_focused_container(&self) -> Option<&Container> {
            None
        }

        fn collect_windows(&self) -> Vec<&Window> {
            let mut windows = Vec::new();
            windows
        }

        fn get_next_window(&self) -> Option<&Window> {
            None
        }

        fn get_previous_window(&self) -> Option<&Window> {
            None
        }

        fn get_adjacent_window(&self, direction: Direction) -> Option<&Window> {
            None
        }
    }

    pub struct Container {
        node: I3Node,
    }

    impl Container {
        fn new(node: I3Node, workspace: &'static Workspace) -> Self {
            Container { node: node }
        }

        fn collect_windows(&self) -> Vec<&Window> {
            let mut windows = Vec::new();
            windows
        }
    }

    pub struct Window {
        node: I3Node,
    }

    impl Window {
        fn new(node: I3Node, container: &'static Container) -> Self {
            Window { node: node }
        }
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

/*
example response from i3-msg -t get_tree
{"id":108000481855584,"type":"root","orientation":"horizontal","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":0,"width":1920,"height":1080},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"root","window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000481774368,"type":"output","orientation":"none","scratchpad_state":"none","percent":0.5,"urgent":false,"marks":[],"focused":false,"layout":"output","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":0,"width":1920,"height":1080},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"__i3","window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000481859808,"type":"con","orientation":"horizontal","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"output":"__i3","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":0,"width":0,"height":0},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"content","window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000481862928,"type":"workspace","orientation":"none","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"output":"__i3","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":0,"width":0,"height":0},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"__i3_scratch","window_icon_padding":-1,"num":-1,"gaps":{"inner":0,"outer":0,"top":0,"right":0,"bottom":0,"left":0},"window":null,"window_type":null,"nodes":[],"floating_nodes":[{"id":108000482003952,"type":"floating_con","orientation":"horizontal","scratchpad_state":"fresh","percent":null,"urgent":false,"marks":[],"focused":false,"output":"__i3","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":480,"y":135,"width":960,"height":810},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000481965824,"type":"con","orientation":"none","scratchpad_state":"none","percent":1.0,"urgent":false,"marks":[],"focused":false,"output":"__i3","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":480,"y":135,"width":960,"height":810},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":950,"height":800},"geometry":{"x":0,"y":0,"width":800,"height":600},"name":"matt@Matt-Latitude-5400: ~","window_icon_padding":-1,"window":65011715,"window_type":"normal","window_properties":{"class":"Alacritty","instance":"Alacritty","machine":"Matt-Latitude-5400","title":"matt@Matt-Latitude-5400: ~","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"user_on","swallows":[]}],"floating_nodes":[],"focus":[108000481965824],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"focus":[108000482003952],"fullscreen_mode":1,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000481862928],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000481859808],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000481866048,"type":"output","orientation":"none","scratchpad_state":"none","percent":0.5,"urgent":false,"marks":[],"focused":false,"layout":"output","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":0,"width":1920,"height":1080},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"eDP-1","window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000481869168,"type":"dockarea","orientation":"none","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"dockarea","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":0,"width":1920,"height":32},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"topdock","window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000481892528,"type":"con","orientation":"none","scratchpad_state":"none","percent":1.0,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":2,"rect":{"x":0,"y":0,"width":1920,"height":32},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":1920,"height":32},"geometry":{"x":0,"y":0,"width":1920,"height":32},"name":"polybar-top_eDP-1","window_icon_padding":-1,"window":14680066,"window_type":"unknown","window_properties":{"class":"Polybar","instance":"polybar","title":"polybar-top_eDP-1","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":true,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000481892528],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[{"dock":2,"insert_where":2}]},{"id":108000481872288,"type":"con","orientation":"horizontal","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":1920,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"content","window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000481878960,"type":"workspace","orientation":"horizontal","scratchpad_state":"none","percent":0.16666666666666665741,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":1920,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"1","window_icon_padding":-1,"num":1,"gaps":{"inner":0,"outer":0,"top":0,"right":0,"bottom":0,"left":0},"window":null,"window_type":null,"nodes":[{"id":108000481517312,"type":"con","orientation":"none","scratchpad_state":"none","percent":1.0,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":20,"y":52,"width":1880,"height":976},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":1870,"height":966},"geometry":{"x":25,"y":57,"width":1870,"height":966},"name":"i3wm.rs - Untitled (Workspace) - CRust - Visual Studio Code","window_icon_padding":-1,"window":27262980,"window_type":"normal","window_properties":{"class":"Code","instance":"code","window_role":"browser-window","machine":"Matt-Latitude-5400","title":"i3wm.rs - Untitled (Workspace) - CRust - Visual Studio Code","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000481517312],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000481782336,"type":"workspace","orientation":"horizontal","scratchpad_state":"none","percent":0.16666666666666665741,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":1920,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"2","window_icon_padding":-1,"num":2,"gaps":{"inner":0,"outer":0,"top":0,"right":0,"bottom":0,"left":0},"window":null,"window_type":null,"nodes":[{"id":108000481810480,"type":"con","orientation":"horizontal","scratchpad_state":"none","percent":1.0,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":1920,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000481797248,"type":"con","orientation":"none","scratchpad_state":"none","percent":0.5,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":20,"y":52,"width":930,"height":976},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":920,"height":966},"geometry":{"x":0,"y":0,"width":920,"height":966},"name":"Test Organization - The Rust Programming Language — Mozilla Firefox","window_icon_padding":-1,"window":41943084,"window_type":"normal","window_properties":{"class":"firefox","instance":"Navigator","window_role":"browser","machine":"Matt-Latitude-5400","title":"Test Organization - The Rust Programming Language — Mozilla Firefox","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000482016160,"type":"con","orientation":"vertical","scratchpad_state":"none","percent":0.5,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splitv","workspace_layout":"default","last_split_layout":"splitv","border":"normal","current_border_width":-1,"rect":{"x":960,"y":32,"width":960,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000481967408,"type":"con","orientation":"none","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":970,"y":52,"width":930,"height":976},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":920,"height":966},"geometry":{"x":0,"y":0,"width":800,"height":600},"name":"matt@Matt-Latitude-5400: ~","window_icon_padding":-1,"window":75497475,"window_type":"normal","window_properties":{"class":"Alacritty","instance":"Alacritty","machine":"Matt-Latitude-5400","title":"matt@Matt-Latitude-5400: ~","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000481967408],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482016160,108000481797248],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000481810480],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000481816384,"type":"workspace","orientation":"horizontal","scratchpad_state":"none","percent":0.16666666666666665741,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":1920,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"3","window_icon_padding":-1,"num":3,"gaps":{"inner":0,"outer":0,"top":0,"right":0,"bottom":0,"left":0},"window":null,"window_type":null,"nodes":[{"id":108000482074800,"type":"con","orientation":"vertical","scratchpad_state":"none","percent":0.5,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splitv","workspace_layout":"default","last_split_layout":"splitv","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":960,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000482093408,"type":"con","orientation":"horizontal","scratchpad_state":"none","percent":0.5,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":960,"height":508},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000481810960,"type":"con","orientation":"none","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":20,"y":52,"width":930,"height":478},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":920,"height":468},"geometry":{"x":0,"y":0,"width":800,"height":600},"name":"matt@Matt-Latitude-5400: ~","window_icon_padding":-1,"window":44040195,"window_type":"normal","window_properties":{"class":"Alacritty","instance":"Alacritty","machine":"Matt-Latitude-5400","title":"matt@Matt-Latitude-5400: ~","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000481810960],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000482071600,"type":"con","orientation":"horizontal","scratchpad_state":"none","percent":0.5,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":540,"width":960,"height":508},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000482095328,"type":"con","orientation":"vertical","scratchpad_state":"none","percent":0.5,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splitv","workspace_layout":"default","last_split_layout":"splitv","border":"normal","current_border_width":-1,"rect":{"x":0,"y":540,"width":480,"height":508},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000482083200,"type":"con","orientation":"none","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":20,"y":550,"width":450,"height":478},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":440,"height":468},"geometry":{"x":0,"y":0,"width":734,"height":792},"name":"Volume Control","window_icon_padding":-1,"window":100663303,"window_type":"normal","window_properties":{"class":"Pavucontrol","instance":"pavucontrol","machine":"Matt-Latitude-5400","title":"Volume Control","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482083200],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000482072704,"type":"con","orientation":"vertical","scratchpad_state":"none","percent":0.5,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splitv","workspace_layout":"default","last_split_layout":"splitv","border":"normal","current_border_width":-1,"rect":{"x":480,"y":540,"width":480,"height":508},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000482036784,"type":"con","orientation":"none","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":490,"y":550,"width":460,"height":478},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":450,"height":468},"geometry":{"x":0,"y":0,"width":920,"height":966},"name":"matt - Thunar","window_icon_padding":-1,"window":85983239,"window_type":"normal","window_properties":{"class":"Thunar","instance":"thunar","window_role":"Thunar-1748172622-1175109936","machine":"Matt-Latitude-5400","title":"matt - Thunar","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482036784],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482095328,108000482072704],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482071600,108000482093408],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000482042544,"type":"con","orientation":"vertical","scratchpad_state":"none","percent":0.5,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splitv","workspace_layout":"default","last_split_layout":"splitv","border":"normal","current_border_width":-1,"rect":{"x":960,"y":32,"width":960,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000482072112,"type":"con","orientation":"none","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":970,"y":52,"width":930,"height":976},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":920,"height":966},"geometry":{"x":0,"y":0,"width":1920,"height":1080},"name":"Minecraft Launcher","window_icon_padding":-1,"window":88080388,"window_type":"unknown","window_properties":{"class":"Minecraft Launcher","instance":"minecraft-launcher","machine":"Matt-Latitude-5400","title":"Minecraft Launcher","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482072112],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482042544,108000482074800],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000481958464,"type":"workspace","orientation":"horizontal","scratchpad_state":"none","percent":0.16666666666666665741,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":1920,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"4","window_icon_padding":-1,"num":4,"gaps":{"inner":0,"outer":0,"top":0,"right":0,"bottom":0,"left":0},"window":null,"window_type":null,"nodes":[{"id":108000482106608,"type":"con","orientation":"horizontal","scratchpad_state":"none","percent":1.0,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":1920,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000482172656,"type":"con","orientation":"vertical","scratchpad_state":"none","percent":0.33333333333333331483,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splitv","workspace_layout":"default","last_split_layout":"splitv","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":640,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000482105680,"type":"con","orientation":"none","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":20,"y":52,"width":610,"height":976},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":600,"height":966},"geometry":{"x":0,"y":0,"width":800,"height":600},"name":"matt@Matt-Latitude-5400: ~","window_icon_padding":-1,"window":113246211,"window_type":"normal","window_properties":{"class":"Alacritty","instance":"Alacritty","machine":"Matt-Latitude-5400","title":"matt@Matt-Latitude-5400: ~","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482105680],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000482165328,"type":"con","orientation":"none","scratchpad_state":"none","percent":0.33333333333333331483,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":650,"y":52,"width":620,"height":976},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":610,"height":966},"geometry":{"x":0,"y":0,"width":800,"height":600},"name":"matt@Matt-Latitude-5400: ~","window_icon_padding":-1,"window":134217731,"window_type":"normal","window_properties":{"class":"Alacritty","instance":"Alacritty","machine":"Matt-Latitude-5400","title":"matt@Matt-Latitude-5400: ~","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000482145648,"type":"con","orientation":"vertical","scratchpad_state":"none","percent":0.33333333333333331483,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splitv","workspace_layout":"default","last_split_layout":"splitv","border":"normal","current_border_width":-1,"rect":{"x":1280,"y":32,"width":640,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":null,"window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000482134256,"type":"con","orientation":"none","scratchpad_state":"none","percent":1.0,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":1290,"y":52,"width":610,"height":976},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":600,"height":966},"geometry":{"x":0,"y":0,"width":800,"height":600},"name":"matt@Matt-Latitude-5400: ~","window_icon_padding":-1,"window":123731971,"window_type":"normal","window_properties":{"class":"Alacritty","instance":"Alacritty","machine":"Matt-Latitude-5400","title":"matt@Matt-Latitude-5400: ~","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482134256],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482172656,108000482145648,108000482165328],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482106608],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000482132048,"type":"workspace","orientation":"horizontal","scratchpad_state":"none","percent":0.16666666666666665741,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":1920,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"5","window_icon_padding":-1,"num":5,"gaps":{"inner":0,"outer":0,"top":0,"right":0,"bottom":0,"left":0},"window":null,"window_type":null,"nodes":[{"id":108000482090976,"type":"con","orientation":"none","scratchpad_state":"none","percent":1.0,"urgent":false,"marks":[],"focused":true,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":20,"y":52,"width":1880,"height":976},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":1870,"height":966},"geometry":{"x":0,"y":0,"width":800,"height":600},"name":"matt@Matt-Latitude-5400: ~","window_icon_padding":-1,"window":102760451,"window_type":"normal","window_properties":{"class":"Alacritty","instance":"Alacritty","machine":"Matt-Latitude-5400","title":"matt@Matt-Latitude-5400: ~","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482090976],"fullscreen_mode":1,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000482107088,"type":"workspace","orientation":"horizontal","scratchpad_state":"none","percent":0.16666666666666665741,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":32,"width":1920,"height":1016},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"6","window_icon_padding":-1,"num":6,"gaps":{"inner":0,"outer":0,"top":0,"right":0,"bottom":0,"left":0},"window":null,"window_type":null,"nodes":[{"id":108000481817968,"type":"con","orientation":"none","scratchpad_state":"none","percent":1.0,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"pixel","current_border_width":5,"rect":{"x":20,"y":52,"width":1880,"height":976},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":5,"y":5,"width":1870,"height":966},"geometry":{"x":0,"y":0,"width":800,"height":600},"name":"matt@Matt-Latitude-5400: ~","window_icon_padding":-1,"window":54525955,"window_type":"normal","window_properties":{"class":"Alacritty","instance":"Alacritty","machine":"Matt-Latitude-5400","title":"matt@Matt-Latitude-5400: ~","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000481817968],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000482132048,108000481878960,108000481958464,108000481782336,108000481816384,108000482107088],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]},{"id":108000481875552,"type":"dockarea","orientation":"none","scratchpad_state":"none","percent":null,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"dockarea","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":-1,"rect":{"x":0,"y":1048,"width":1920,"height":32},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":0,"height":0},"geometry":{"x":0,"y":0,"width":0,"height":0},"name":"bottomdock","window_icon_padding":-1,"window":null,"window_type":null,"nodes":[{"id":108000481889408,"type":"con","orientation":"none","scratchpad_state":"none","percent":1.0,"urgent":false,"marks":[],"focused":false,"output":"eDP-1","layout":"splith","workspace_layout":"default","last_split_layout":"splith","border":"normal","current_border_width":2,"rect":{"x":0,"y":1048,"width":1920,"height":32},"deco_rect":{"x":0,"y":0,"width":0,"height":0},"window_rect":{"x":0,"y":0,"width":1920,"height":32},"geometry":{"x":0,"y":1048,"width":1920,"height":32},"name":"polybar-bottom_eDP-1","window_icon_padding":-1,"window":16777218,"window_type":"unknown","window_properties":{"class":"Polybar","instance":"polybar","title":"polybar-bottom_eDP-1","transient_for":null},"nodes":[],"floating_nodes":[],"focus":[],"fullscreen_mode":0,"sticky":true,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000481889408],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[{"dock":3,"insert_where":2}]}],"floating_nodes":[],"focus":[108000481872288,108000481869168,108000481875552],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}],"floating_nodes":[],"focus":[108000481866048,108000481774368],"fullscreen_mode":0,"sticky":false,"floating":"auto_off","swallows":[]}

example response from i3-msg -t get_workspaces
[{"id":108000481878960,"num":1,"name":"1","visible":false,"focused":false,"rect":{"x":0,"y":32,"width":1920,"height":1016},"output":"eDP-1","urgent":false},{"id":108000481782336,"num":2,"name":"2","visible":false,"focused":false,"rect":{"x":0,"y":32,"width":1920,"height":1016},"output":"eDP-1","urgent":false},{"id":108000481816384,"num":3,"name":"3","visible":false,"focused":false,"rect":{"x":0,"y":32,"width":1920,"height":1016},"output":"eDP-1","urgent":false},{"id":108000481958464,"num":4,"name":"4","visible":false,"focused":false,"rect":{"x":0,"y":32,"width":1920,"height":1016},"output":"eDP-1","urgent":false},{"id":108000482132048,"num":5,"name":"5","visible":true,"focused":true,"rect":{"x":0,"y":32,"width":1920,"height":1016},"output":"eDP-1","urgent":false},{"id":108000482107088,"num":6,"name":"6","visible":false,"focused":false,"rect":{"x":0,"y":32,"width":1920,"height":1016},"output":"eDP-1","urgent":false}]

example respnse from i3-msg -t get_outputs
[{"name":"eDP-1","active":true,"primary":true,"rect":{"x":0,"y":0,"width":1920,"height":1080},"current_workspace":"5"},{"name":"xroot-0","active":false,"primary":false,"rect":{"x":0,"y":0,"width":1920,"height":1080},"current_workspace":null}]

Bear in mind

pub struct Node {

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
