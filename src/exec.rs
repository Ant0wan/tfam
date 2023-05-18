use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use std::process::ExitStatus;
use std::thread;

use crate::cli::Commands;
use crate::workspace::get_workspace;

fn process_file(args: &Vec<String>, cmd: &Commands, file: &String) -> ExitStatus {
    let workspace = get_workspace(file, &cmd.workspaceformat);
    println!(
        "TF_WORKSPACE={} {} {} -var-file {:?}",
        workspace,
        cmd.bin,
        args.join(" "),
        file
    );
    Command::new(&cmd.bin)
        .args(args)
        .env("TF_WORKSPACE", workspace)
        .arg("-var-file")
        .arg(file)
        .status()
        .expect(&format!("failed to execute {}", cmd.bin))
}

pub fn exec(args: &Vec<String>, cmd: &Commands) -> ExitStatus {
    let mut last_error = ExitStatus::from_raw(0);

    match cmd.varfiles.is_empty() {
        true => {
            println!("{} {}", cmd.bin, args.join(" "));
            Command::new(&cmd.bin)
                .args(args)
                .status()
                .expect(&format!("failed to execute {}", cmd.bin))
        }
        false => match cmd.concurrent {
            true => {
                let handles: Vec<_> = cmd
                    .varfiles
                    .iter()
                    .map(|f| thread::spawn(move || process_file(args, cmd, f)))
                    .collect();
                for handle in handles {
                    let status_result = handle.join().unwrap();
                    if !status_result.success() {
                        last_error = status_result;
                    }
                }
                last_error
            }
            false => {
                for file in &cmd.varfiles {
                    let status_result = process_file(args, cmd, file);
                    if !status_result.success() {
                        last_error = status_result;
                    }
                }
                last_error
            }
        },
    }
}
