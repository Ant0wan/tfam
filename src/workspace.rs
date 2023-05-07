use std::env;

pub fn get_workspace(path: String) -> String {
    match env::var("TF_WORKSPACE") {
        Ok(workspace) => return workspace,
        Err(_) => return convert_path_to_workspace(path),
    }
}

fn convert_path_to_workspace(path: String) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    let joined = parts.join("_");

    let without_extension = if joined.ends_with(".tfvars.json") {
        joined.trim_end_matches(".json").to_string()
    } else if joined.ends_with(".tfvars") {
        joined.trim_end_matches(".tfvars").to_string()
    } else {
        joined
    };

    return without_extension.to_string();
}
