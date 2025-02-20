use std::collections::HashMap;

// list of numbers:
pub static FIBBONACI_NUMBERS: [i32; 20] = [
    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
];

pub fn fibbil_hash(string: &str) -> String {
    let mut hash = String::new();
    for char in string.chars() {
        let shifted = ((char as u32) + (FIBBONACI_NUMBERS[char as usize] as u32)) as u8;
        hash.push(shifted as char);
    }
    hash
}

pub fn codesmith28(string: &str) -> String {
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

    // Fill x with characters and y with indices
    for &(c, i) in &idx {
        x.push(c);
        y.push_str(&i.to_string());
    }

    // Concatenate x and y
    x.push_str(&y);

    x
}
