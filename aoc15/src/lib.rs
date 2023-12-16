use tracing::trace;

fn update_hash(current: u8, c: char) -> u8 {
    let x = current as usize;

    trace!("Hashing {} with '{}'", current, c);
    let x = x + c as usize;
    trace!("    With ASCII: {}", x);
    let x = x * 17;
    trace!("    Times 17: {}", x);

    (x & 0xFF) as u8
}

fn hash_string(s: &str) -> u8 {
    s.chars().fold(0, |acc, c| update_hash(acc, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_hash() {
        assert_eq!(update_hash(0, 'H'), 200);
        assert_eq!(update_hash(200, 'A'), 153);
        assert_eq!(update_hash(153, 'S'), 172);
        assert_eq!(update_hash(172, 'H'), 52);
    }

    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string("HASH"), 52);
        assert_eq!(hash_string("rn=1"), 30);
        assert_eq!(hash_string("cm-"), 253);
    }
}

