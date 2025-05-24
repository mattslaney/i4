// use std::fs::OpenOptions;
// use std::io::Write;
// use std::sync::Mutex;

// pub struct Logger {
//     file: Mutex<std::fs::File>,
// }

// impl Logger {
//     pub fn new(file_path: &str) -> Logger {
//         let file = OpenOptions::new()
//             .create(true)
//             .append(true)
//             .open(file_path)
//             .unwrap();
//         Logger {
//             file: Mutex::new(file),
//         }
//     }

//     pub fn log(&self, msg: &str) {
//         if let Ok(mut file) = self.file.lock() {
//             writeln!(file, "{}", msg).unwrap();
//         }
//     }
// }

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

pub struct Logger {
    file: Option<Mutex<std::fs::File>>,
}

impl Logger {
    pub fn new(file_path: Option<String>) -> Logger {
        let file = file_path.map(|path| {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap();
            Mutex::new(file)
        });
        Logger { file }
    }

    pub fn log(&self, msg: String) {
        match &self.file {
            Some(file) => {
                if let Ok(mut file) = file.lock() {
                    writeln!(file, "{}", msg).unwrap();
                }
            }
            None => println!("{}", msg),
        }
    }
}
