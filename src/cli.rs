use std::env;

#[derive(Debug)]
pub struct Commands {
    pub interactive: bool,
    pub concurrent: bool,
    pub help: bool,
    pub commands: Vec<String>,
    pub varfiles: Vec<String>,
}

fn print_usage() {
    let brief = "Usage: tfam [global options] <subcommand> [args]";
    print!("{}", brief);
}

pub fn parse_commands() -> (Vec<String>, Commands) {
    let mut args: Vec<String> = env::args().collect();
    let mut cmd = Commands {
        interactive: false,
        concurrent: false,
        help: false,
        commands: Vec::new(),
        varfiles: Vec::new(),
    };

    let mut args_iter = args.iter().skip(1);
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-interactive" => cmd.interactive = true,
            "-concurrent" => cmd.concurrent = true,
            "-help" => cmd.help = true,
            "-var-file" => {
                if let Some(file) = args.iter().skip_while(|x| x != &arg).nth(1) {
                    cmd.varfiles.push(file.to_string());
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
    }

    (args, cmd)
}
