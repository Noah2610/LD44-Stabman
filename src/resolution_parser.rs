use std::io;

use amethyst::utils::application_root_dir;
use regex::Regex;

use crate::resource_helpers::*;

type Resolution = (u32, u32);

pub fn get_resolution() -> Result<Option<Resolution>, String> {
    const RESOLUTION_FILE_NAME: &str = "resolution.txt";
    let parsing_error = |msg| {
        format!(
            "Error parsing resolution file `{}`: {}",
            RESOLUTION_FILE_NAME, msg
        )
    };

    let file_path =
        format!("{}/{}", application_root_dir(), RESOLUTION_FILE_NAME);
    match read_file(file_path) {
        Ok(file_content) => match parse_resolution(file_content) {
            Err(err) => Err(parsing_error(err)),
            ret => ret,
        },
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(parsing_error(err.to_string())),
    }
}

pub fn parse_resolution<S>(text: S) -> Result<Option<Resolution>, String>
where
    S: ToString,
{
    // Ignore blank lines, or lines starting with a `#`.
    const IGNORE_LINES_PATTERN: &str = r"\A\s*(#.*)?\z";
    // Resolution string in the format `1280x720` (ignoring whitespace).
    const RESOLUTION_STRING_PATTERN: &str =
        r"\A\s*(?P<w>\d+)\s*x\s*(?P<h>\d+)\s*(#.*)?\z";

    let ignore_lines_re = Regex::new(IGNORE_LINES_PATTERN).unwrap();
    let resolution_string_re = Regex::new(RESOLUTION_STRING_PATTERN).unwrap();

    let text = text.to_string();
    if let Some(usable_line) = text.split("\n").find_map(|line| {
        if ignore_lines_re.is_match(line) {
            None
        } else {
            Some(line)
        }
    }) {
        if let Some(captures) = resolution_string_re.captures(usable_line) {
            match (captures.name("w"), captures.name("h")) {
                (Some(w_str), Some(h_str)) => {
                    let w_str = w_str.as_str();
                    let h_str = h_str.as_str();
                    match (w_str.parse::<u32>(), h_str.parse::<u32>()) {
                        (Ok(w), Ok(h)) => Ok(Some((w, h))),
                        (Err(err), _) | (_, Err(err)) => Err(format!(
                            "Error parsing text to integer (u32): {}",
                            err
                        )),
                    }
                }
                _ => Err(format!(
                    "Error matching width and height on '{}'",
                    usable_line
                )),
            }
        } else {
            Err(format!("Could not parse line:\n'{}'", usable_line))
        }
    } else {
        Ok(None)
    }
}
