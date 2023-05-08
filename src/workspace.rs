use std::env;

pub fn get_workspace(path: String) -> String {
    match env::var("TF_WORKSPACE") {
        Ok(workspace) => return workspace,
        Err(_) => return convert_path_to_workspace(path),
    }
}

fn convert_path_to_workspace(path: String) -> String {
    let parts: Vec<&str>;

    let without_extension = if path.ends_with(".tfvars.json") {
        path.trim_end_matches(".tfvars.json").to_string()
    } else if path.ends_with(".tfvars") {
        path.trim_end_matches(".tfvars").to_string()
    } else {
        path
    };

    parts = without_extension.split('/').collect();
    println!("{:?}", parts);
    //    if format.len()
    //    let mut fields:
    //    let mut joined = parts.join("_");
    //    //let mut fields: Vec<&str> = path.trim().split('/').map(|field| field.trim()).collect();
    //    fields.reverse();
    //    println!("{:?}", fields);

    return parts.join("_");
}

//fn custom_workspace_format(elements: Vec<String>)
