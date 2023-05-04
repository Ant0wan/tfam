extern crate getopts;

use getopts::Options;
use std::env;

#[derive(Debug)]
pub enum Command {
    Workspace,
}

#[derive(Debug)]
pub enum WorkspaceSubcommand {
    Purge { all: bool },
}

#[derive(Debug)]
pub struct Args {
    pub interactive: bool,
    pub concurrent: bool,
    pub varfiles: Vec<String>,
    pub command: Option<Command>,
    pub workspace_subcommand: Option<WorkspaceSubcommand>,
}

fn print_usage(opts: &Options) {
    let brief = "Usage: tfam [options] <command> [<args>]";
    print!("{}", opts.usage(&brief));
}

pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag(
        "i",
        "interactive",
        "execute with interactive prompt to select tfvars before executing terraform",
    );
    opts.optflag("c", "concurrent", "enable concurrent operations");
    opts.optopt("", "var-file", "use a specific variable file", "<filename>");
    opts.optflag("h", "help", "print usage message and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}", f.to_string());
            print_usage(&opts);
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(&opts);
        std::process::exit(0);
    }

    let mut varfiles = matches.opt_strs("var-file");

    let command = match matches.free.len() {
        0 => None,
        1 => match matches.free[0].as_str() {
            "workspace" => Some(Command::Workspace),
            cmd => {
                eprintln!("Unknown command: {}", cmd);
                print_usage(&opts);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Multiple commands provided. Only one command can be used at a time.");
            print_usage(&opts);
            std::process::exit(1);
        }
    };

    let workspace_subcommand = match command {
        Some(Command::Workspace) => {
            let mut sub_opts = Options::new();
            sub_opts.optflag("a", "", "delete all workspaces");

            let matches = match sub_opts.parse(&matches.free[1..]) {
                Ok(m) => m,
                Err(f) => {
                    eprintln!("{}", f.to_string());
                    print_usage(&opts);
                    std::process::exit(1);
                }
            };

            if matches.free.len() > 0 {
                eprintln!("Unexpected argument provided: {}", matches.free[0]);
                print_usage(&opts);
                std::process::exit(1);
            }

            if matches.opt_present("a") {
                Some(WorkspaceSubcommand::Purge { all: true })
            } else {
                None
            }
        }
        _ => None,
    };

    Args {
        interactive: matches.opt_present("interactive"),
        concurrent: matches.opt_present("concurrent"),
        varfiles,
        command,
        workspace_subcommand,
    }
}
