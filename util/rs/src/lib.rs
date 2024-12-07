use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

pub type AppResult<T = ()> = Result<T, AppError>;

#[derive(Debug)]
pub struct AppError {
    message: String,
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            message: format!("[IO Error] {error}"),
        }
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError {
            message: format!("[Error] {error}"),
        }
    }
}

impl<'a> From<&'a str> for AppError {
    fn from(error: &'a str) -> Self {
        AppError {
            message: format!("[Error] {error}"),
        }
    }
}

pub fn read_input() -> io::Result<String> {
    let is_dev = env::var("DEV").is_ok();
    let file_name = if is_dev { "dev-input.txt" } else { "input.txt" };

    let project_dir = get_project_dir()?;
    let input_file = project_dir.join(file_name);

    fs::read_to_string(input_file).map_err(|e| {
        io::Error::new(io::ErrorKind::NotFound, format!("{e}, {file_name}"))
    })
}

// https://github.com/neilwashere/rust-project-root/blob/main/src/lib.rs
fn get_project_dir() -> io::Result<PathBuf> {
    let current_exe = env::current_exe()?;
    let path = current_exe.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to find parent directory of current executable",
        )
    })?;
    let mut path_ancestors = path.ancestors();

    while let Some(parent) = path_ancestors.next() {
        let has_cargo = fs::read_dir(parent)?.into_iter().any(|entry_res| {
            entry_res
                .map(|entry| {
                    entry.file_name() == std::ffi::OsString::from("Cargo.toml")
                })
                .unwrap_or(false)
        });
        if has_cargo {
            return Ok(PathBuf::from(parent));
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Failed to find Cargo.toml",
    ))
}
