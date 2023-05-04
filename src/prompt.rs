use inquire::error::InquireResult;
use inquire::formatter::MultiOptionFormatter;
use inquire::MultiSelect;

pub fn select_tfvars_files(results: Vec<String>) -> InquireResult<Vec<String>> {
    let mut results = results;
    results.sort();

    let formatter: MultiOptionFormatter<String> = &|a| format!("{} tfvars files", a.len());

    MultiSelect::new("Select tfvars:", results)
        .with_formatter(formatter)
        .prompt()
}
