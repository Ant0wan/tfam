use std::process::Command;
use std::thread;

use crate::cli::Commands;
use crate::workspace::get_workspace;

pub fn execute_varfiles(args: Vec<String>, cmd: Commands) {
    if cmd.concurrent {
        multi_threads_exec(args, cmd);
    } else {
        single_threaded_exec(args, cmd);
    }
}

fn exec(args: Vec<String>, varfile: String, workspaceformat: String) {
    let workspace = get_workspace(varfile.clone(), workspaceformat);
    println!(
        "TF_WORKSPACE={} terraform {} -var-file={:?}",
        workspace,
        args.join(" "),
        varfile
    );
    let _status = Command::new("terraform")
        .env("TF_WORKSPACE", workspace)
        .args(args)
        .arg("-var-file")
        .arg(varfile)
        .status();
}

fn single_threaded_exec(args: Vec<String>, cmd: Commands) {
    if cmd.varfiles.is_empty() {
        println!("terraform {}", args.join(" "));
        let _status = Command::new("terraform").args(args).status();
        return;
    }
    for f in cmd.varfiles {
        exec(args.clone(), f, cmd.workspaceformat.clone());
    }
}

fn multi_threads_exec(args: Vec<String>, cmd: Commands) {
    let threads: Vec<_> = cmd
        .varfiles
        .into_iter()
        .map(|f| {
            let args = args.clone();
            let workspaceformat = cmd.workspaceformat.clone();
            thread::spawn(move || {
                exec(args, f, workspaceformat);
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}
