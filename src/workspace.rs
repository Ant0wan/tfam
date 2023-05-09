use std::env;

pub fn get_workspace(path: String, format: String) -> String {
    match env::var("TF_WORKSPACE") {
        Ok(workspace) => return workspace,
        Err(_) => return convert_path_to_workspace(path, format),
    }
}

fn convert_path_to_workspace(path: String, format: String) -> String {
    let parts: Vec<&str>;

    let without_extension = path.splitn(2, '.').next().unwrap_or(&path);
    parts = without_extension.split('/').collect();
    println!("{:?}", parts);
    if format.is_empty() {
        println!("Non Custom format");
        return parts.join("_");
    } else {
        println!("Custom format");
        return replace_placeholders(parts, format);
    }
}

fn replace_placeholders(vector: Vec<&str>, str_expr: String) -> String {
    let mut result = String::from(str_expr);

    for (i, s) in vector.iter().enumerate() {
        let placeholder = format!("${}", i + 1);
        result = result.replace(&placeholder, s);
    }

    result
}
