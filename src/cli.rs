use std::env;

#[derive(Debug)]
pub struct Commands {
    pub interactive: bool,
    pub concurrent: bool,
    pub automation: bool,
    pub help: bool,
    pub workspaceformat: String,
    pub commands: Vec<String>,
    pub varfiles: Vec<String>,
}

pub fn print_usage() {
    println!("Usage: tfam [global options] <subcommand> [args]"); // To complete
}

pub fn parse_commands() -> (Vec<String>, Commands) {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut cmd = Commands {
        interactive: false,
        concurrent: false,
        automation: false,
        help: false,
        workspaceformat: String::new(),
        commands: Vec::new(),
        varfiles: Vec::new(),
    };
    let mut allformats: Vec<String> = Vec::new();

    let mut args_iter = args.iter();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-interactive" => match env::var("TF_IN_AUTOMATION") {
                Ok(_) => {
                    cmd.interactive = false;
                    cmd.automation = true;
                }
                Err(_) => cmd.interactive = true,
            },
            "-concurrent" => cmd.concurrent = true,
            "-help" => cmd.help = true,
            "-var-file" => {
                if let Some(file) = args_iter.next() {
                    cmd.varfiles.push(file.to_string());
                }
            }
            "-workspace-format" => {
                if let Some(file) = args_iter.next() {
                    cmd.workspaceformat = file.to_string();
                    allformats.push(file.to_string());
                }
            }
            _ => {}
        }
        match arg.starts_with("-var-file=") {
            true => {
                if let Some(suffix) = arg.strip_prefix("-var-file=") {
                    cmd.varfiles.push(suffix.to_string());
                } else {
                    println!("Error, no varfile specified. `-var-file=` cannot be empty.");
                }
            }
            _ => {}
        }
        match arg.starts_with("-workspace-format=") {
            true => {
                if let Some(suffix) = arg.strip_prefix("-workspace-format=") {
                    cmd.workspaceformat = suffix.to_string();
                    allformats.push(suffix.to_string());
                } else {
                    println!("Error, no format specified. `-workspace-format=` cannot be empty.");
                }
            }
            _ => {}
        }
    }
    if cmd.interactive || cmd.automation {
        args.retain(|x| x != "-interactive");
    }
    if cmd.concurrent {
        args.retain(|x| x != "-concurrent");
    }
    for value in &cmd.varfiles {
        args.retain(|x| x != value);
    }
    args.retain(|e| !e.starts_with("-var-file"));
    for fmt in &allformats {
        args.retain(|x| x != fmt);
    }
    args.retain(|e| !e.starts_with("-workspace-format"));

    (args, cmd)
}
