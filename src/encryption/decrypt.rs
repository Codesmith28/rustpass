use super::encrypt::FIBBONACI_NUMBERS;

pub fn fibbil_unhash(hash: &str) -> String {
    let mut un_hash = String::new();
    for char in hash.chars() {
        let shifted = ((char as u32) - (FIBBONACI_NUMBERS[char as usize] as u32)) as u8;
        un_hash.push(shifted as char);
    }
    un_hash
}
pub fn codesmith82(encoded: &str) -> Option<String> {
    // Find the split point
    let char_count = encoded.chars().take_while(|c| !c.is_ascii_digit()).count();

    if char_count == 0 || char_count == encoded.len() {
        return None; // Invalid format
    }

    let chars: Vec<char> = encoded[..char_count].chars().collect();
    let indices_str = &encoded[char_count..];

    // Extract numeric indices correctly
    let indices: Vec<usize> = indices_str
        .split_inclusive(|c: char| c.is_ascii_digit())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    // Ensure the number of characters matches the indices
    if chars.len() != indices.len() {
        return None;
    }

    // Pair and sort by original position
    let mut pairs: Vec<(char, usize)> = chars.into_iter().zip(indices).collect();
    pairs.sort_by_key(|&(_, pos)| pos);

    // Reconstruct the original string
    Some(pairs.into_iter().map(|(c, _)| c).collect())
}
