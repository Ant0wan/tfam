use std::env;

pub fn parse_commands() -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut args: Vec<String> = env::args().collect();
    let mut commands: Vec<String> = Vec::new();
    let mut varfiles: Vec<String> = Vec::new();

    let mut args_iter = args.iter().skip(1);
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-interactive" => commands.push(String::from("interactive")),
            "-concurrent" => commands.push(String::from("concurrent")),
            "-var-file" => {
                if let Some(file) = args.iter().skip_while(|x| x != &arg).nth(1) {
                    varfiles.push(file.to_string());
                }
            }
            _ => {}
        }
        match arg.starts_with("-var-file=") {
            true => {
                if let Some(suffix) = arg.strip_prefix("-var-file=") {
                    varfiles.push(suffix.to_string());
                } else {
                    println!("Error, no varfile specified. `-var-file=` cannot be empty.");
                }
            }
            _ => {}
        }
    }

    (args, commands, varfiles)
}
