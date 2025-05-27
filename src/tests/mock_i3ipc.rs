extern crate i3ipc;
extern crate serde;
extern crate serde_json;

use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

use i3ipc::reply;
use serde_json as json;

fn build_rect(jrect: &json::Value) -> (i32, i32, i32, i32) {
    let x = jrect.get("x").unwrap().as_i64().unwrap() as i32;
    let y = jrect.get("y").unwrap().as_i64().unwrap() as i32;
    let width = jrect.get("width").unwrap().as_i64().unwrap() as i32;
    let height = jrect.get("height").unwrap().as_i64().unwrap() as i32;
    (x, y, width, height)
}

fn build_window_properties(
    j: Option<&json::Value>,
) -> Option<HashMap<reply::WindowProperty, String>> {
    match j {
        None => None,
        Some(props) => {
            let properties = props.as_object().unwrap();
            let mut map = HashMap::new();
            for (key, val) in properties {
                let window_property = match key.as_ref() {
                    "class" => Some(reply::WindowProperty::Class),
                    "instance" => Some(reply::WindowProperty::Instance),
                    "window_role" => Some(reply::WindowProperty::WindowRole),
                    "title" => Some(reply::WindowProperty::Title),
                    "transient_for" => Some(reply::WindowProperty::TransientFor),
                    _other => {
                        return None;
                    }
                };
                if let Some(window_property) = window_property {
                    map.insert(
                        window_property,
                        val.as_str().unwrap_or_default().to_string(),
                    );
                }
            }
            Some(map)
        }
    }
}

fn build_tree(val: &json::Value) -> reply::Node {
    reply::Node {
        focus: match val.get("focus") {
            Some(xs) => xs
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_i64().unwrap())
                .collect(),
            None => vec![],
        },
        nodes: match val.get("nodes") {
            Some(nds) => nds
                .as_array()
                .unwrap()
                .iter()
                .map(|n| build_tree(n))
                .collect(),
            None => vec![],
        },
        floating_nodes: match val.get("floating_nodes") {
            Some(nds) => nds
                .as_array()
                .unwrap()
                .iter()
                .map(|n| build_tree(n))
                .collect(),
            None => vec![],
        },
        id: val.get("id").unwrap().as_i64().unwrap(),
        name: match val.get("name") {
            Some(n) => match n.as_str() {
                Some(s) => Some(s.to_owned()),
                None => None,
            },
            None => None,
        },
        nodetype: match val.get("type").unwrap().as_str().unwrap() {
            "root" => reply::NodeType::Root,
            "output" => reply::NodeType::Output,
            "con" => reply::NodeType::Con,
            "floating_con" => reply::NodeType::FloatingCon,
            "workspace" => reply::NodeType::Workspace,
            "dockarea" => reply::NodeType::DockArea,
            _other => reply::NodeType::Unknown,
        },
        border: match val.get("border").unwrap().as_str().unwrap() {
            "normal" => reply::NodeBorder::Normal,
            "none" => reply::NodeBorder::None,
            "pixel" => reply::NodeBorder::Pixel,
            _other => reply::NodeBorder::Unknown,
        },
        current_border_width: val.get("current_border_width").unwrap().as_i64().unwrap() as i32,
        layout: match val.get("layout").unwrap().as_str().unwrap() {
            "splith" => reply::NodeLayout::SplitH,
            "splitv" => reply::NodeLayout::SplitV,
            "stacked" => reply::NodeLayout::Stacked,
            "tabbed" => reply::NodeLayout::Tabbed,
            "dockarea" => reply::NodeLayout::DockArea,
            "output" => reply::NodeLayout::Output,
            _other => reply::NodeLayout::Unknown,
        },
        percent: match *val.get("percent").unwrap() {
            json::Value::Number(ref f) => Some(f.as_f64().unwrap()),
            json::Value::Null => None,
            _ => unreachable!(),
        },
        rect: build_rect(val.get("rect").unwrap()),
        window_rect: build_rect(val.get("window_rect").unwrap()),
        deco_rect: build_rect(val.get("deco_rect").unwrap()),
        geometry: build_rect(val.get("geometry").unwrap()),
        window: match val.get("window").unwrap().clone() {
            json::Value::Number(i) => Some(i.as_i64().unwrap() as i32),
            json::Value::Null => None,
            _ => unreachable!(),
        },
        window_properties: build_window_properties(val.get("window_properties")),
        urgent: val.get("urgent").unwrap().as_bool().unwrap(),
        focused: val.get("focused").unwrap().as_bool().unwrap(),
    }
}

#[derive(Debug)]
pub struct MockI3Connection {
    // stream: UnixStream,
    path: PathBuf,
}

impl MockI3Connection {
    /// Rather than establishing a real IPC connection, we'll expect a path to a directory
    /// that contains mock data json files for testing purposes.
    pub fn connect(path: PathBuf) -> Result<MockI3Connection, std::io::Error> {
        match fs::read_dir(&path) {
            Ok(_) => Ok(MockI3Connection { path: path }),
            Err(e) => Err(e),
        }
    }

    /// return the command that would have been executed.
    pub fn run_command(&mut self, string: &str) -> Result<String, std::io::Error> {
        Ok(string.to_string())
    }

    /// Gets the workspaces from a json file.
    pub fn get_workspaces(&mut self) -> Result<reply::Workspaces, std::io::Error> {
        let filename = "workspaces.json";
        let filepath = PathBuf::from(&self.path).join(filename);
        let file = match File::open(filepath) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let j: json::Value = match serde_json::from_reader(file) {
            Ok(json) => json,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        };
        let jworkspaces = j.as_array().unwrap();
        let workspaces: Vec<_> = jworkspaces
            .iter()
            .map(|w| reply::Workspace {
                num: w.get("num").unwrap().as_i64().unwrap() as i32,
                name: w.get("name").unwrap().as_str().unwrap().to_owned(),
                visible: w.get("visible").unwrap().as_bool().unwrap(),
                focused: w.get("focused").unwrap().as_bool().unwrap(),
                urgent: w.get("urgent").unwrap().as_bool().unwrap(),
                rect: build_rect(w.get("rect").unwrap()),
                output: w.get("output").unwrap().as_str().unwrap().to_owned(),
            })
            .collect();
        Ok(reply::Workspaces { workspaces })
    }

    /// Gets the outputs from a json file.
    pub fn get_outputs(&mut self) -> Result<reply::Outputs, std::io::Error> {
        let filename = "outputs.json";
        let filepath = PathBuf::from(&self.path).join(filename);
        let file = match File::open(filepath) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let j: json::Value = match serde_json::from_reader(file) {
            Ok(json) => json,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        };
        let joutputs = j.as_array().unwrap();
        let outputs: Vec<_> = joutputs
            .iter()
            .map(|o| reply::Output {
                name: o.get("name").unwrap().as_str().unwrap().to_owned(),
                active: o.get("active").unwrap().as_bool().unwrap(),
                primary: o.get("primary").unwrap().as_bool().unwrap(),
                current_workspace: match o.get("current_workspace").unwrap().clone() {
                    json::Value::String(c_w) => Some(c_w),
                    json::Value::Null => None,
                    _ => unreachable!(),
                },
                rect: build_rect(o.get("rect").unwrap()),
            })
            .collect();
        Ok(reply::Outputs { outputs })
    }

    /// Gets the layout tree from a json file.
    pub fn get_tree(&mut self) -> Result<reply::Node, std::io::Error> {
        let filename = "tree.json";
        let filepath = PathBuf::from(&self.path).join(filename);
        let file = match File::open(filepath) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let val: json::Value = match serde_json::from_reader(file) {
            Ok(json) => json,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        };
        Ok(build_tree(&val))
    }
}
