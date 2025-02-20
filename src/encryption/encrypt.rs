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
