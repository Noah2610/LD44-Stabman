use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use amethyst::utils::application_root_dir;

const RESOURCES_DIRNAME: &str = "resources";

pub fn resources_dir() -> String {
    format!("{}/{}", application_root_dir(), RESOURCES_DIRNAME)
}

pub fn resource<T: ToString>(path: T) -> String {
    format!("{}/{}", resources_dir(), path.to_string())
}

pub fn read_file<P>(path: P) -> Result<String, io::Error>
where
    P: AsRef<Path> + Display,
{
    let mut file = File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
