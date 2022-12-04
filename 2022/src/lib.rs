use std::env;
use std::fs;

pub fn read_file(directory: &str, day: u8) -> Result<String, std::io::Error> {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join(directory)
        .join(format!("day{:02}.txt", day));

    fs::read_to_string(filepath)
}
