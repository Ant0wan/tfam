use std::env;
use std::slice::Iter;

#[derive(Debug)]
pub struct Commands {
    pub automation: bool,
    pub bin: String,
    pub commands: Vec<String>,
    pub concurrent: bool,
    pub help: bool,
    pub interactive: bool,
    pub tfargs: Vec<String>,
    pub varfiles: Vec<String>,
    pub workspaceformat: String,
}

impl Commands {
    fn new() -> Self {
        Commands {
            automation: false,
            bin: String::new(),
            commands: Vec::new(),
            concurrent: false,
            help: false,
            interactive: false,
            tfargs: Vec::new(),
            varfiles: Vec::new(),
            workspaceformat: String::new(),
        }
    }
}

pub fn print_usage() {
    println!("Usage: tfam [global options] <subcommand> [args]"); // To complete
}

impl Commands {
    pub fn parse_commands(mut args: Vec<String>) -> Self {
        let mut cmd: Commands = Commands::new();
        let mut allformats: Vec<String> = Vec::new();

        cmd.bin = env::var("TFAM_EXE").unwrap_or_else(|_| "terraform".to_string());

        let mut args_iter: Iter<String> = args.iter();
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
                        println!(
                            "Error, no format specified. `-workspace-format=` cannot be empty."
                        );
                    }
                }
                _ => {}
            }
        }
        if cmd.interactive || cmd.automation {
            args.retain(|x: &String| x != "-interactive");
        }
        if cmd.concurrent {
            args.retain(|x: &String| x != "-concurrent");
        }
        for value in &cmd.varfiles {
            args.retain(|x: &String| x != value);
        }
        args.retain(|e: &String| !e.starts_with("-var-file"));
        for fmt in &allformats {
            args.retain(|x: &String| x != fmt);
        }
        args.retain(|e: &String| !e.starts_with("-workspace-format"));
        cmd.tfargs = args;
        cmd
    }
}
