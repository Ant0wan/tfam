use std::thread;

pub fn execute_varfiles(args: Vec<String>, varfiles: Vec<String>) {
    let threads: Vec<_> = varfiles
        .into_iter()
        .map(|f| {
            let args = args.clone();
            thread::spawn(move || {
                println!("{:?} -var-file={:?}", args, f);
                // add any other code you want to execute for each varfile here
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}
