use std::env;
use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use std::process::ExitStatus;
use std::thread;

//use std::thread;

use crate::cli::Commands;
use crate::workspace::get_workspace;

fn process_file(
    args: &Vec<String>,
    cmd: &Commands,
    f: &String,
    executable: &String,
    exe: &mut Command,
) -> ExitStatus {
    let workspace = get_workspace(f, &cmd.workspaceformat);
    println!(
        "TF_WORKSPACE={} {} {} -var-file {:?}",
        workspace,
        executable,
        args.join(" "),
        f
    );
    exe.env("TF_WORKSPACE", workspace)
        .arg("-var-file")
        .arg(f)
        .status()
        .expect(&format!("failed to execute {}", executable))
}

pub fn exec(args: &Vec<String>, cmd: &Commands) -> ExitStatus {
    let executable = env::var("TFAM_EXE").unwrap_or_else(|_| "terraform".to_string());
    let mut exe = Command::new(&executable);
    let mut exe = exe.args(args);
    let mut last_error = ExitStatus::from_raw(0);

    match cmd.varfiles.is_empty() {
        true => {
            println!("{} {}", executable, args.join(" "));
            exe.status()
                .expect(&format!("failed to execute {}", executable))
        }
        false => match cmd.concurrent {
            true => {
                //        let handle = thread::spawn(|| {
                //        });
                //        handle.join().unwrap();
                exe.status().expect("TEST")
            }
            false => {
                for f in &cmd.varfiles {
                    let status_result = process_file(args, cmd, f, &executable, exe);
                    if !status_result.success() {
                        last_error = status_result;
                    }
                }
                last_error
            }
        },
    }
}
