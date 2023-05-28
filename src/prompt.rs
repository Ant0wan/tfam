use inquire::error::InquireResult;
use inquire::formatter::MultiOptionFormatter;
use inquire::list_option::ListOption;
use inquire::MultiSelect;

pub fn select_tfvars_files(results: Vec<String>) -> InquireResult<Vec<String>> {
    let mut results: Vec<String> = results;
    results.sort();

    let formatter: MultiOptionFormatter<String> =
        &|a: &[ListOption<&String>]| format!("{} tfvars files", a.len());

    MultiSelect::new("Select tfvars:", results)
        .with_formatter(formatter)
        .prompt()
}
