pub use std::env;

pub fn get_workspace(path: &String, format: &String) -> String {
    match env::var("TF_WORKSPACE") {
        Ok(workspace) => return workspace,
        Err(_) => return convert_path_to_workspace(path, format),
    }
}

fn convert_path_to_workspace(path: &String, format: &String) -> String {
    let parts: Vec<&str>;

    let without_extension: &str = path.splitn(2, '.').next().unwrap_or(&path);
    parts = without_extension.split('/').collect();
    if format.is_empty() {
        match env::var("TF_WORKSPACE_FORMAT") {
            Ok(format) => return replace_placeholders(parts, &format),
            Err(_) => return parts.join("_"),
        }
    } else {
        return replace_placeholders(parts, format);
    }
}

fn replace_placeholders(vector: Vec<&str>, str_expr: &String) -> String {
    let mut result: String = String::from(str_expr.clone());

    for i in 0..vector.len() {
        let placeholder: String = format!("${}", i + 1);
        let negative_placeholder: String = format!("$-{}", i + 1);
        let value: Option<&&str> = if str_expr.contains(&negative_placeholder) {
            vector.get(vector.len() - i - 1)
        } else {
            vector.get(i)
        };

        if let Some(s) = value {
            result = result.replace(&placeholder, s);
            result = result.replace(&negative_placeholder, s);
        }
    }

    result
}
