
fn update_hash(current: u8, c: char) -> u8 {
    let x = current as usize;
    
    let x = x + c as usize;
    let x = x * 17;

    (x & 0xFF) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(update_hash(0, 'H'), 200);
        assert_eq!(update_hash(200, 'A'), 153);
        assert_eq!(update_hash(153, 'S'), 172);
        assert_eq!(update_hash(172, 'H'), 52);
    }
}
