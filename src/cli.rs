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
            "varfile" => {
                if let Some(file) = args.iter().skip_while(|x| x != &arg).nth(1) {
                    varfiles.push(file.to_string());
                }
            }
            _ => {}
        }
        match arg.starts_with() {
            "-var-file=" => {
                if let Some(suffix) = arg.strip_prefix("-var-file=") {
                    varfiles.push(suffix.to_string());
                } else {
                    println!("Error, no varfile specified"); // check terraform error msg in this case
                }
            }
            _ => {}
        }
    }

    (args, commands, varfiles)
}
