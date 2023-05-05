use std::env;

pub fn parse_commands() -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut args: Vec<String> = env::args().collect();
    let mut commands: Vec<String> = Vec::new();
    let mut varfiles: Vec<String> = Vec::new();

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "interactive" => commands.push(String::from("interactive")),
            "concurrent" => commands.push(String::from("concurrent")),
            "varfile" => {
                if let Some(file) = args.iter().skip_while(|x| x != &arg).nth(1) {
                    varfiles.push(file.to_string());
                }
            }
            _ => {}
        }
    }

    (args, commands, varfiles)
}
