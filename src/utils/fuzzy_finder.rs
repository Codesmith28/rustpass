pub fn fuzzy_match(query: &str, target: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let mut query_chars = query.chars();
    let mut current = query_chars.next().unwrap();
    for c in target.chars() {
        if c == current {
            if let Some(next) = query_chars.next() {
                current = next;
            } else {
                return true;
            }
        }
    }
    false
}
