// list of numbers:
pub static FIBBONACI_NUMBERS: [i32; 20] = [
    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
];

// static numeric key:
pub static NUMERIC_KEY: [i32; 20] = [
    14, 7, 3, 16, 11, 19, 1, 13, 6, 9, 17, 5, 12, 8, 4, 20, 15, 10, 18, 2,
];

// fibil encryption
pub fn fibbil_hash(string: &str) -> String {
    let mut hash = String::new();
    for (i, char) in string.chars().enumerate() {
        let key_index = i % NUMERIC_KEY.len();
        let fib_index = (NUMERIC_KEY[key_index] - 1) as usize;
        // Use wrapping_add to handle overflow safely and ensure valid char range
        let shifted =
            (((char as u32).wrapping_add(FIBBONACI_NUMBERS[fib_index] as u32)) % 128) as u8;
        hash.push(shifted as char);
    }
    hash
}

pub fn codesmith28(string: &str) -> String {
    if string.is_empty() {
        return String::new();
    }

    // Create a vector of tuples (character, index+1)
    let mut idx: Vec<(char, usize)> = string
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();

    // Sort the vector by character
    idx.sort_by(|a, b| a.0.cmp(&b.0));

    // Create strings x and y
    let mut x = String::new();
    let mut y = String::new();

    // Fill x with characters and y with indices, using a separator
    for &(c, i) in &idx {
        x.push(c);
        y.push_str(&format!("{:03}", i)); // Use fixed-width format for indices
    }

    // Concatenate x and y with a separator
    x.push('|'); // Add separator between characters and indices
    x.push_str(&y);

    x
}
