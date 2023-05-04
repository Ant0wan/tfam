use inquire::formatter::MultiOptionFormatter;
use inquire::MultiSelect;

fn select_tfvars_files(results: Vec<String>) -> Option<Vec<usize>> {
    let mut results = results;
    results.sort();

    let formatter: MultiOptionFormatter<String> = &|a| format!("{} tfvars files", a.len());

    MultiSelect::new("Select tfvars:", results)
        .with_formatter(formatter)
        .prompt()
}

