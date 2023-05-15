use std::env;
use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use std::process::ExitStatus;
use std::thread;

//use std::thread;

use crate::cli::Commands;
use crate::workspace::get_workspace;

fn process_file(args: &Vec<String>, cmd: &Commands, f: &String, exe: &mut Command) -> ExitStatus {
    let workspace = get_workspace(f, &cmd.workspaceformat);
    println!(
        "TF_WORKSPACE={} {} {} -var-file {:?}",
        workspace,
        cmd.bin,
        args.join(" "),
        f
    );
    exe.env("TF_WORKSPACE", workspace)
        .arg("-var-file")
        .arg(f)
        .status()
        .expect(&format!("failed to execute {}", cmd.bin))
}

pub fn exec(args: &Vec<String>, cmd: &Commands) -> ExitStatus {
    let mut exe = Command::new(&cmd.bin);
    let exe = exe.args(args);
    let mut last_error = ExitStatus::from_raw(0);

    match cmd.varfiles.is_empty() {
        true => {
            println!("{} {}", cmd.bin, args.join(" "));
            exe.status()
                .expect(&format!("failed to execute {}", cmd.bin))
        }
        false => match cmd.concurrent {
            true => {
                exe.status()
                    .expect(&format!("failed to execute {}", cmd.bin))
                //process_file(args, cmd, f, exe)
                //       let handles: Vec<_> = cmd
                //           .varfiles
                //           .iter()
                //           .map(|f| {
                //               thread::spawn(move || {
                //                   process_file(args, cmd, f, &cmd.bin, exe);
                //               })
                //           })
                //           .collect();
                //       for handle in handles {
                //           let _status_result = handle.join().unwrap();
                //           //if !status_result.success() {
                //           //    last_error = status_result;
                //           // }
                //       }
                //       last_error
            }
            false => {
                for f in &cmd.varfiles {
                    let status_result = process_file(args, cmd, f, exe);
                    if !status_result.success() {
                        last_error = status_result;
                    }
                }
                last_error
            }
        },
    }
}
