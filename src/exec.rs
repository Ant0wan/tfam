use std::env;
use std::process::Command;
use std::thread;

use crate::cli::Commands;
use crate::workspace::get_workspace;

pub fn execute_varfiles(args: Vec<String>, cmd: Commands) {
    let executable = match env::var("TFAM_EXE") {
        Ok(exe) => exe,
        Err(_) => "terraform".to_string(),
    };
    if cmd.concurrent {
        multi_threads_exec(args, cmd, executable);
    } else {
        single_threaded_exec(args, cmd, executable);
    }
}

fn exec(args: Vec<String>, varfile: String, workspaceformat: String, executable: String) {
    let workspace = get_workspace(varfile.clone(), workspaceformat);
    println!(
        "TF_WORKSPACE={} {} {} -var-file={:?}",
        workspace,
        executable,
        args.join(" "),
        varfile
    );
    let _status = Command::new(executable)
        .env("TF_WORKSPACE", workspace)
        .args(args)
        .arg("-var-file")
        .arg(varfile)
        .status();
}

fn single_threaded_exec(args: Vec<String>, cmd: Commands, executable: String) {
    if cmd.varfiles.is_empty() {
        println!("{} {}", executable, args.join(" "));
        let _status = Command::new(executable).args(args).status();
        return;
    }
    for f in cmd.varfiles {
        exec(
            args.clone(),
            f,
            cmd.workspaceformat.clone(),
            executable.clone(),
        );
    }
}

fn multi_threads_exec(args: Vec<String>, cmd: Commands, executable: String) {
    let threads: Vec<_> = cmd
        .varfiles
        .into_iter()
        .map(|f| {
            let args = args.clone();
            let workspaceformat = cmd.workspaceformat.clone();
            let executable = executable.clone();
            thread::spawn(move || {
                exec(args, f, workspaceformat, executable);
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}
