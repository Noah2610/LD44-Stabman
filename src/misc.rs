use std::fs::{create_dir, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::exit;

use amethyst::utils::application_root_dir;

use crate::LOGFILE;

pub fn on_panic(info: &std::panic::PanicInfo) {
    let logfile_path = format!("{}/{}", application_root_dir(), LOGFILE);
    let logfile_path = Path::new(&logfile_path);

    // Create `LOGFILE`'s parent directory if it does not exist.
    if let Some(logdir_path) = logfile_path.parent() {
        if !logdir_path.exists() {
            if let Err(_) = create_dir(logdir_path) {
                eprintln!(
                    "Couldn't create logfile directory: {}",
                    logfile_path.display()
                );
                exit(1);
            }
        }
    }

    // Open logfile for writing.
    let mut logfile = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(logfile_path)
    {
        Err(_) => {
            eprintln!(
                "Couldn't write error to file: {}",
                logfile_path.display(),
            );
            exit(1);
        }
        Ok(f) => f,
    };

    // Gather info.
    let now = chrono::Local::now();
    let date_string = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut output = vec![
        "====================".to_string(),
        date_string,
        "====================".to_string(),
    ];
    if let Some(location) = info.location() {
        output.push(location.to_string());
    }
    if let Some(payload) = info.payload().downcast_ref::<&str>() {
        output.push(payload.to_string());
    }
    output.push(String::from("\n\n"));

    // Print panic info to file.
    if let Err(err) = logfile.write(output.join("\n").as_bytes()) {
        eprintln!("Couldn't print panic info to file: {}", err);
        exit(1);
    }
}
