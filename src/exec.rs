use std::thread;

use crate::workspace::convert_path_to_workspace;

pub fn execute_varfiles(args: Vec<String>, varfiles: Vec<String>, concurrent: bool) {
    // env vars:
    // - TF_WORKSPACE=<tfvars_path>
    if concurrent {
        multi_threads_exec(args, varfiles);
    } else {
        single_threaded_exec(args, varfiles);
    }
}

fn exec(args: Vec<String>, varfile: String) {
    println!(
        "TF_WORKSPACE={} {:?} -var-file={:?}",
        convert_path_to_workspace(varfile.clone()),
        args,
        varfile
    );
}

fn single_threaded_exec(args: Vec<String>, varfiles: Vec<String>) {
    for f in varfiles {
        exec(args.clone(), f);
    }
}

fn multi_threads_exec(args: Vec<String>, varfiles: Vec<String>) {
    let threads: Vec<_> = varfiles
        .into_iter()
        .map(|f| {
            let args = args.clone();
            thread::spawn(move || {
                exec(args, f);
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}
