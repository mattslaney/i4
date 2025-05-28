/**
 * i4 - a grid-like navigator for i3wm
 */
mod i4;
mod logger;
mod macros;

extern crate i3ipc;

use i4::Direction::{Down, Left, Right, Up};
use i4::{Root, Window, I4};
use logger::Logger;

const MAX_HORIZONTAL_WORKSPACES: i32 = 10;

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
    println!("  focus            Focus a window");
    println!("                   [left, right, up, down, previous, next]");
    println!("  move             Move a window");
    println!("                   [left, right, up, down, previous, next]");
}

fn process_request(args: Vec<String>) {
    let i3: I4<i3ipc::I3Connection> = I4::connect(None, MAX_HORIZONTAL_WORKSPACES);

    action_request(i3, args, std::io::stdout());
}

fn action_request<W: std::io::Write, I: i4::I3ConnectionTrait>(
    mut i3: I4<I>,
    args: Vec<String>,
    mut out: W,
) {
    let i3root = i3.get_root();
    match args[1].as_str() {
        "get" => {
            if args.len() < 3 {
                println!("Error: Missing argument for get command");
                return;
            }
            match args[2].as_str() {
                "state" => {
                    writeln!(out, "{}", i3.get_state()).expect("Failed to write state to output");
                }
                _ => {
                    println!("Error: Unknown argument for get command");
                }
            }
        }
        "focus" => {
            if args.len() < 3 {
                println!("Error: Missing argument for focus command");
                return;
            }
            match args[2].as_str() {
                "left" => {
                    println!("Focusing left...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_adjacent_window(Left) {
                                Some(window) => i3.focus_window(window),
                                None => match output.get_adjacent_workspace(Left) {
                                    Some(workspace) => i3.focus_workspace(&workspace.data.name),
                                    None => {
                                        let new_name = format!(
                                            "{}",
                                            workspace.data.name.parse::<i32>().unwrap() - 1
                                        );
                                        i3.create_workspace(&new_name)
                                    }
                                },
                            }
                        }
                    }
                }
                "right" => {
                    println!("Focusing right...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_adjacent_window(Right) {
                                Some(window) => i3.focus_window(window),
                                None => match output.get_adjacent_workspace(Right) {
                                    Some(workspace) => i3.focus_workspace(&workspace.data.name),
                                    None => {
                                        let new_name = format!(
                                            "{}",
                                            workspace.data.name.parse::<i32>().unwrap() + 1
                                        );
                                        i3.create_workspace(&new_name)
                                    }
                                },
                            }
                        }
                    }
                }
                "up" => {
                    println!("Focusing up...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_adjacent_window(Up) {
                                Some(window) => i3.focus_window(window),
                                None => match output.get_adjacent_workspace(Up) {
                                    Some(workspace) => i3.focus_workspace(&workspace.data.name),
                                    None => {
                                        let new_name = format!(
                                            "{}",
                                            workspace.data.name.parse::<i32>().unwrap()
                                                - MAX_HORIZONTAL_WORKSPACES
                                        );
                                        i3.create_workspace(&new_name)
                                    }
                                },
                            }
                        }
                    }
                }
                "down" => {
                    println!("Focusing down...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_adjacent_window(Down) {
                                Some(window) => i3.focus_window(window),
                                None => match output.get_adjacent_workspace(Down) {
                                    Some(workspace) => i3.focus_workspace(&workspace.data.name),
                                    None => {
                                        let new_name = format!(
                                            "{}",
                                            workspace.data.name.parse::<i32>().unwrap()
                                                + MAX_HORIZONTAL_WORKSPACES
                                        );
                                        i3.create_workspace(&new_name)
                                    }
                                },
                            }
                        }
                    }
                }
                "previous" => {
                    println!("Focusing previous...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_previous_window() {
                                Some(window) => i3.focus_window(window),
                                None => match output.get_previous_workspace() {
                                    Some(workspace) => i3.focus_workspace(&workspace.data.name),
                                    None => {
                                        let new_name = format!(
                                            "{}",
                                            workspace.data.name.parse::<i32>().unwrap() - 1
                                        );
                                        i3.create_workspace(&new_name)
                                    }
                                },
                            }
                        }
                    }
                }
                "next" => {
                    println!("Focusing next...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_next_window() {
                                Some(window) => i3.focus_window(window),
                                None => match output.get_next_workspace() {
                                    Some(workspace) => i3.focus_workspace(&workspace.data.name),
                                    None => {
                                        let new_name = format!(
                                            "{}",
                                            workspace.data.name.parse::<i32>().unwrap() + 1
                                        );
                                        i3.create_workspace(&new_name)
                                    }
                                },
                            }
                        }
                    }
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
                    println!("Moving left...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_adjacent_window(Left) {
                                Some(_) => i3.move_window(Left),
                                None => {
                                    let new_name = format!(
                                        "{}",
                                        workspace.data.name.parse::<i32>().unwrap() - 1
                                    );
                                    i3.move_window_to_workspace(
                                        match output.get_adjacent_workspace(Down) {
                                            Some(workspace) => &workspace.data.name,
                                            None => &new_name,
                                        },
                                    );
                                    i3.focus_workspace(&new_name);
                                }
                            }
                        }
                    }
                }
                "right" => {
                    println!("Moving right...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_adjacent_window(Right) {
                                Some(_) => i3.move_window(Right),
                                None => {
                                    let new_name = format!(
                                        "{}",
                                        workspace.data.name.parse::<i32>().unwrap() + 1
                                    );
                                    i3.move_window_to_workspace(
                                        match output.get_adjacent_workspace(Down) {
                                            Some(workspace) => &workspace.data.name,
                                            None => &new_name,
                                        },
                                    );
                                    i3.focus_workspace(&new_name);
                                }
                            }
                        }
                    }
                }
                "up" => {
                    println!("Moving up...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_adjacent_window(Up) {
                                Some(_) => i3.move_window(Up),
                                None => {
                                    let new_name = format!(
                                        "{}",
                                        workspace.data.name.parse::<i32>().unwrap()
                                            + MAX_HORIZONTAL_WORKSPACES
                                    );
                                    i3.move_window_to_workspace(
                                        match output.get_adjacent_workspace(Down) {
                                            Some(workspace) => &workspace.data.name,
                                            None => &new_name,
                                        },
                                    );
                                    i3.focus_workspace(&new_name);
                                }
                            }
                        }
                    }
                }
                "down" => {
                    println!("Moving down...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_adjacent_window(Down) {
                                Some(_) => i3.move_window(Down),
                                None => {
                                    let new_name = format!(
                                        "{}",
                                        workspace.data.name.parse::<i32>().unwrap()
                                            + MAX_HORIZONTAL_WORKSPACES
                                    );
                                    i3.move_window_to_workspace(
                                        match output.get_adjacent_workspace(Down) {
                                            Some(workspace) => &workspace.data.name,
                                            None => &new_name,
                                        },
                                    );
                                    i3.focus_workspace(&new_name);
                                }
                            }
                        }
                    }
                }
                "previous" => {
                    println!("Moving previous...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_adjacent_window(Left) {
                                Some(_) => i3.move_window(Left),
                                None => {
                                    let new_name = format!(
                                        "{}",
                                        workspace.data.name.parse::<i32>().unwrap() - 1
                                    );
                                    i3.move_window_to_workspace(
                                        match output.get_previous_workspace() {
                                            Some(workspace) => &workspace.data.name,
                                            None => &new_name,
                                        },
                                    );
                                    i3.focus_workspace(&new_name);
                                }
                            }
                        }
                    }
                }
                "next" => {
                    println!("Moving next...");
                    if let Some(output) = i3root.get_focused_output() {
                        if let Some(workspace) = output.get_focused_workspace() {
                            match workspace.get_adjacent_window(Left) {
                                Some(_) => i3.move_window(Left),
                                None => {
                                    let new_name = format!(
                                        "{}",
                                        workspace.data.name.parse::<i32>().unwrap() - 1
                                    );
                                    i3.move_window_to_workspace(
                                        match output.get_next_workspace() {
                                            Some(workspace) => &workspace.data.name,
                                            None => &new_name,
                                        },
                                    );
                                    i3.focus_workspace(&new_name);
                                }
                            }
                        }
                    }
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

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();
    let mut logfile: Option<String> = None;

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_usage();
        return;
    }

    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        println!("i4 version 0.1.0");
        return;
    }

    if args.contains(&"-d".to_string()) || args.contains(&"--debug".to_string()) {
        if let Some(index) = args.iter().position(|x| x == "-d" || x == "--debug") {
            args.remove(index);
            println!("Enter arguments for i4:");
            let mut input: String = "".to_string();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            args.extend(input.trim().split_whitespace().map(String::from));
            println!("Debugging with arguments: {:?}", args);
        }
    }

    if args.contains(&"-l".to_string()) || args.contains(&"--logfile".to_string()) {
        if let Some(index) = args.iter().position(|x| x == "-l" || x == "--logfile") {
            let current_executable = std::env::current_exe().unwrap();
            let executable_path = current_executable.parent().unwrap();
            let executable_path_str = executable_path.to_str().unwrap();
            logfile = Some(format!("{}/i4.log", executable_path_str));
        }
    }

    process_request(args);
}

#[cfg(test)]
mod tests {
    pub mod mock_i3ipc;

    use super::*;
    use mock_i3ipc::MockI3Connection;

    #[test]
    fn get_state_request_prints_to_stdout() {
        let i3: I4<MockI3Connection> =
            I4::connect(Some("src/tests/scenarios/basic".to_string()), 10);
        let mut output = Vec::new();
        let expected_str = "{\"output\":\"DP-1-0\", \"workspace\":{\"vertical\":\"0\", \"horizontal\":\"4\", \"number\":\"4\"}, \"window\":\"file - Project - Workspace - Code Editor\"}\n";

        let args = vec!["i4".to_string(), "get".to_string(), "state".to_string()];
        action_request(i3, args, &mut output);

        let output_str = String::from_utf8(output).expect("Failed to convert output to string");
        assert!(output_str.eq(expected_str));
    }
}
