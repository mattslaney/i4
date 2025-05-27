/**
 * i4 - a grid-like navigator for i3wm
 */
mod i3wm;
mod logger;
mod macros;

extern crate i3ipc;

use std::char::MAX;

use i3ipc::reply::Output;
use i3wm::i3wm::Direction::{Down, Left, Right, Up};
use i3wm::i3wm::{Root, Util, Window};
use logger::Logger;

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

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();
    let mut debug_mode = false;
    let mut logfile: Option<String> = None;

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
        println!("Enter arguments for i4:");
        let mut input: String = "".to_string();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        args.extend(input.trim().split_whitespace().map(String::from));
        println!("Debugging with arguments: {:?}", args);
    }

    // let current_executable = std::env::current_exe().unwrap();
    // let executable_path = current_executable.parent().unwrap();
    // let executable_path_str = executable_path.to_str().unwrap();
    // if executable_path_str == "/usr/local/bin" {
    //     logfile = Some("/var/log/i4.log".to_string());
    // } else {
    //     logfile = Some(format!("{}/i4.log", executable_path_str));
    // }
    // let logger = Logger::new(logfile);
    // logfile = Some("/var/log/i4.log".to_string());

    let MAX_HORIZONTAL_WORKSPACES = 10;
    let mut i3 = Util::connect();
    let i3root = i3.get_root(MAX_HORIZONTAL_WORKSPACES); //Root::new(MAX_HORIZONTAL_WORKSPACES);

    match args[1].as_str() {
        "get" => {
            if args.len() < 3 {
                println!("Error: Missing argument for get command");
                return;
            }
            match args[2].as_str() {
                "state" => {
                    let output = i3root.get_focused_output().unwrap();
                    let workspace = output.get_focused_workspace().unwrap();
                    let workspace_number = workspace.data.name.parse::<i32>().unwrap();
                    let vertical_space = workspace_number / MAX_HORIZONTAL_WORKSPACES;
                    let horizontal_space = workspace_number % MAX_HORIZONTAL_WORKSPACES;
                    match workspace.get_focused_window() {
                        Some(window) => {
                            println!(
                                "{{\"output\":\"{}\", \"workspace\":{{\"vertical\":\"{}\", \"horizontal\":\"{}\", \"number\":\"{}\"}}, \"window\":\"{}\"}}",
                                output.data.name,
                                vertical_space,
                                horizontal_space,
                                workspace.data.name,
                                window.node.name.unwrap_or("".to_string())
                            );
                        }
                        None => {
                            println!(
                                "{{\"output\":\"{}\", \"workspace\":{{\"vertical\":\"{}\", \"horizontal\":\"{}\", \"number\":\"{}\"}}, \"window\":\"\"}}",
                                output.data.name,
                                vertical_space,
                                horizontal_space,
                                workspace.data.name,
                            );
                        }
                    }
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
