extern crate walkdir;

use inquire::formatter::MultiOptionFormatter;
use inquire::MultiSelect;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::process::Command;
use walkdir::WalkDir;

{
    results.sort();
    let formatter: MultiOptionFormatter<String> = &|a| format!("{} tfvars files", a.len());
    let ans = MultiSelect::new("Select tfvars:", results)
        .with_formatter(formatter)
        .prompt();

}
