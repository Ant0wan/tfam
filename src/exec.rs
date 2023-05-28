use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use std::process::ExitStatus;
use std::sync::MutexGuard;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::cli::Commands;
use crate::workspace::get_workspace;

fn process_file(cmd: &Commands, file: &String) -> ExitStatus {
    let workspace: String = get_workspace(file, &cmd.workspaceformat);
    println!(
        "TF_WORKSPACE={} {} {} -var-file {:?}",
        workspace,
        cmd.bin,
        cmd.tfargs.join(" "),
        file
    );
    Command::new(&cmd.bin)
        .args(&cmd.tfargs)
        .env("TF_WORKSPACE", workspace)
        .arg("-var-file")
        .arg(file)
        .status()
        .unwrap_or_else(|_| panic!("failed to execute {}", cmd.bin))
}

pub fn exec(cmd: Arc<Mutex<Commands>>) -> ExitStatus {
    let mut last_error: ExitStatus = ExitStatus::from_raw(0);
    let cmd_lock: MutexGuard<Commands> = cmd.lock().unwrap();

    if cmd_lock.varfiles.is_empty() {
        println!("{} {}", cmd_lock.bin, cmd_lock.tfargs.join(" "));
        Command::new(&cmd_lock.bin)
            .args(&cmd_lock.tfargs)
            .status()
            .unwrap_or_else(|_| panic!("failed to execute {}", cmd_lock.bin))
    } else if cmd_lock.concurrent {
        let handles: Vec<_> = cmd_lock
            .varfiles
            .iter()
            .map(|f: &String| {
                let cmd_clone: Arc<Mutex<Commands>> = Arc::clone(&cmd);
                let file_clone: String = f.clone();
                thread::spawn(move || {
                    let cmd_lock: MutexGuard<Commands> = cmd_clone.lock().unwrap();
                    process_file(&cmd_lock, &file_clone)
                })
            })
            .collect();
        for handle in handles {
            let status_result: ExitStatus = handle.join().unwrap();
            if !status_result.success() {
                last_error = status_result;
            }
        }
        last_error
    } else {
        for file in &cmd_lock.varfiles {
            let status_result: ExitStatus = process_file(&cmd_lock, file);
            if !status_result.success() {
                last_error = status_result;
            }
        }
        last_error
    }
}
