pub fn puzzle_data(module_mod_rs_path: &str) -> String {
    let new_path = (std::path::Path::new(module_mod_rs_path))
        .parent()
        .unwrap()
        .join("data.raw");

    return std::fs::read_to_string(new_path.to_str().unwrap())
        .expect("Should be able to read the file");
}
