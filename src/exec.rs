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

fn exec(args: Vec<String>, varfile: String) {
    println!(
        "TF_WORKSPACE={} {:?} -var-file={:?}",
        get_workspace(varfile.clone()),
        args,
        varfile
    );
}

fn single_threaded_exec(args: Vec<String>, cmd: Commands) {
    for f in cmd.varfiles {
        exec(args.clone(), f);
    }
}

fn multi_threads_exec(args: Vec<String>, cmd: Commands) {
    let threads: Vec<_> = cmd
        .varfiles
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
//            for element in ans.unwrap() {
//                println!("terraform {:?} -var-file {}", args.clone(), element);
//                let status = Command::new("terraform")
//                    .args(args.clone())
//                    .arg("-var-file")
//                    .arg(element)
//                    .status();
//    }
