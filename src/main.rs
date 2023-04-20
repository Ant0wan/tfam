extern crate termion;
extern crate walkdir;

use std::env;
use std::fs;
use std::io::{self, stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let mut results: Vec<String> = Vec::new();
    let current_dir = env::current_dir()?;
    for entry in WalkDir::new(current_dir.clone()) {
        let entry = entry?;
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .map(|e| e == "tfvars")
                .unwrap_or(false)
        {
            let entry_path = fs::canonicalize(entry.path().display().to_string()).unwrap();
            let relative_path = entry_path.strip_prefix(&current_dir).unwrap();
            results.push(relative_path.to_string_lossy().to_string());
        }
    }

    results.sort();
    println!("{:?}", results); //DEBUG

    //    let stdout = stdout();
    //    let stdout = stdout.lock().into_raw_mode()?;
    //    let mut stdout = io::BufWriter::new(stdout);
    //    write!(stdout, "Select a file (use arrow keys, press Enter to confirm):\n")?;
    //    stdout.flush()?;
    //
    //    let mut selected_index = 0;
    //    let mut stdin = stdin();
    //    for (index, result) in results.iter().enumerate() {
    //        if index == selected_index {
    //            write!(stdout, "\x1B[1m>\x1B[0m {}\n", result)?;
    //        } else {
    //            write!(stdout, "  {}\n", result)?;
    //        }
    //        stdout.flush()?;
    //    }
    //
    //    loop {
    //        match stdin.next()? {
    //            Some(Key::Up) => {
    //                if selected_index > 0 {
    //                    selected_index -= 1;
    //                }
    //            }
    //            Some(Key::Down) => {
    //                if selected_index < results.len() - 1 {
    //                    selected_index += 1;
    //                }
    //            }
    //            Some(Key::Char('\n')) => {
    //                // User pressed Enter, confirm the selection
    //                break;
    //            }
    //            _ => {}
    //        }
    //
    //        // Clear the screen and redraw the prompt
    //        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;
    //        write!(stdout, "Select a file (use arrow keys, press Enter to confirm):\n")?;
    //
    //        for (index, result) in results.iter().enumerate() {
    //            if index == selected_index {
    //                write!(stdout, "\x1B[1m>\x1B[0m {}\n", result)?;
    //            } else {
    //                write!(stdout, "  {}\n", result)?;
    //            }
    //            stdout.flush()?;
    //        }
    //    }
    //
    //    // Print the selected file path
    //    write!(stdout, "{}\n", results[selected_index])?;
    //    stdout.flush()?;

    Ok(())
}
