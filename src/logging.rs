use std::{
    fs::{File, OpenOptions},
    io::Write,
};

pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

pub struct Logger<'a> {
    pub path: &'a str,
    pub level: LogLevel,
}

impl Logger<'_> {
    pub fn debug<'a>(msg: &'a str, path: &'a str) -> Result<(), &'a str> {
        let mut log_file: File =
            if let Ok(f) = OpenOptions::new().append(true).create(true).open(path) {
                f
            } else {
                return Err("could not open or create the file");
            };
        let (line, col) = (line!(), column!());
        let _ = writeln!(log_file, "line {line}, col {col}\n||| {msg}\n");
        return Ok(());
    }
}
