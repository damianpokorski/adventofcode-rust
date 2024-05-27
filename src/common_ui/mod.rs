use dialoguer::Select;
use std::ops::Range;

pub fn cli_range_picker(range: Range<i32>, label: &str) -> Option<i32> {
    // Convert range to list of strings
    let values: Vec<String> = range.into_iter().map(|x| x.to_string()).collect();

    // Display selection
    let selected_index = Select::new()
        .items(&values)
        .max_length(5)
        .with_prompt(label)
        .default(0)
        .interact()
        .unwrap();

    // Get selected index, parse value, return it
    return values
        .get(selected_index)
        .and_then(|selected_value| selected_value.parse::<i32>().ok());
}
