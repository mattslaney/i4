/**
 * i4 - a grid-like navigator for i3wm
 */
mod i3wm;
mod logger;
mod macros;

extern crate i3ipc;

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
    println!("  list             List nodes");
    println!("                   [all, focused, visible]");
    println!("  focus            Focus a window");
    println!("                   [left, right, up, down, previous, next]");
    println!("  move             Move a window");
    println!("                   [left, right, up, down, previous, next]");
}

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();
    // let mut debug_mode = false;
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
        // debug_mode = true;
        // let current_executable = std::env::current_exe().unwrap();
        // let executable_path = current_executable.parent().unwrap();
        // let executable_path_str = executable_path.to_str().unwrap();
        // if executable_path_str == "/usr/local/bin" {
        //     logfile = Some("/var/log/i4.log".to_string());
        // } else {
        //     logfile = Some(format!("{}/i4.log", executable_path_str));
        // }
        logfile = Some("/var/log/i4.log".to_string());
    }

    let logger = Logger::new(logfile);

    match args[1].as_str() {
        "list" => {
            if args.len() < 3 {
                println!("Error: Missing argument for list command");
                return;
            }
            match args[2].as_str() {
                "all" => {}
                "focused" => {
                    println!("Listing focused node...");
                }
                "visible" => println!("Listing visible nodes..."),
                "windows" => {
                    println!("Listing windows...");
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
                "previous" => {
                    println!("Focusing previous window...");
                }
                "next" => {
                    println!("Focusing next window...");
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
