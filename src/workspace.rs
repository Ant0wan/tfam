pub use std::env;

#[must_use]
pub fn get(path: &String, format: &String) -> String {
    match env::var("TF_WORKSPACE") {
        Ok(workspace) => workspace,
        Err(_) => convert_path_to_workspace(path, format),
    }
}

fn convert_path_to_workspace(path: &String, format: &String) -> String {
    let without_extension: &str = path.split('.').next().unwrap_or(path);
    let parts: Vec<&str> = without_extension.split('/').collect();
    if format.is_empty() {
        match env::var("TF_WORKSPACE_FORMAT") {
            Ok(format) => replace_placeholders(parts, &format),
            Err(_) => parts.join("_"),
        }
    } else {
        replace_placeholders(parts, format)
    }
}

#[allow(clippy::needless_pass_by_value)]
fn replace_placeholders(vector: Vec<&str>, str_expr: &str) -> String {
    let mut result: String = String::from(str_expr);

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
