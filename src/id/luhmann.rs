pub fn generate_luhmann_id(parent_id: Option<&str>) -> String {
    match parent_id {
        Some(id) => format!("{}.1", id),
        None => "1".to_string(),
    }
}
