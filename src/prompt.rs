use inquire::error::InquireResult;
use inquire::formatter::MultiOptionFormatter;
use inquire::list_option::ListOption;
use inquire::MultiSelect;

/// Selects `TFVars` files.
///
/// # Errors
///
/// Returns an error if there is an issue with selecting the `TFVars` files.
///
/// # Arguments
///
/// * `results` - A vector of strings representing the `TFVars` files.
///
/// # Returns
///
/// A `Result` containing a vector of selected `TFVars` files or an error.
pub fn select_tfvars_files(results: Vec<String>) -> InquireResult<Vec<String>> {
    let mut results: Vec<String> = results;
    results.sort();

    let formatter: MultiOptionFormatter<String> =
        &|a: &[ListOption<&String>]| format!("{} tfvars files", a.len());

    MultiSelect::new("Select tfvars:", results)
        .with_formatter(formatter)
        .prompt()
}
