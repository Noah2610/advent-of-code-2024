use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn read_input() -> io::Result<String> {
    let is_dev = env::var("DEV").is_ok();
    let file_name = if is_dev { "dev-input.txt" } else { "input.txt" };

    let project_dir = get_project_dir()?;
    let input_file = project_dir.join(file_name);

    fs::read_to_string(input_file)
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
        dbg!(parent);
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
