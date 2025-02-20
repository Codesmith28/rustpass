use super::encrypt::FIBBONACI_NUMBERS;

pub fn fibbil_unhash(hash: &str) -> String {
    let mut un_hash = String::new();
    for char in hash.chars() {
        let shifted = ((char as u32) - (FIBBONACI_NUMBERS[char as usize] as u32)) as u8;
        un_hash.push(shifted as char);
    }
    un_hash
}
