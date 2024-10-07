#[allow(dead_code)]
pub fn dump_slice_to_binary(bytes: &[u8]) -> String {
    let mut string = String::with_capacity(bytes.len() * (8 + 2)); // a character for each bit and the newline

    for (i, byte) in bytes.iter().enumerate() {
        string += &format!("{:08b}", byte);

        if (i + 1) % 2 == 0 {
            string += "\n";
        }
    }

    string
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dump_slice() {
        let data = [0xDE, 0xAD, 0xBE, 0xEF];

        let expected = "1101111010101101\n1011111011101111\n";
        let dumped = super::dump_slice_to_binary(&data[..]);

        assert_eq!(expected, dumped);
    }
}
