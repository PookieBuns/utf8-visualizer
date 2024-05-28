fn main() {
    let c = '🦀';
    let c_unicode_hex = format!("{:x}", c as i32);
    let c_unicode_binary = format!("{:b}", c as i32);
    let c_unicode_binary_target_length = (c_unicode_binary.len() + 3) / 4 * 4;
    let c_unicode_binary_split = format!(
        "{:0width$b}",
        c as i32,
        width = c_unicode_binary_target_length
    )
    .as_bytes()
    .chunks(4)
    .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
    .collect::<Vec<String>>();
    let utf8_byte_count = c.len_utf8();
    let mut utf8_bytes = vec![0; utf8_byte_count];
    let _ = c.encode_utf8(&mut utf8_bytes);
    let utf8_bytes_binary = utf8_bytes
        .iter()
        .map(|byte| format!("{:08b}", byte))
        .collect::<Vec<String>>();
    let utf8_bytes_binary_visualized_from_unicode_binary = utf8_bytes_binary
        .into_iter()
        .enumerate()
        .map(|(i, mut byte)| {
            if i == 0 {
                match utf8_byte_count {
                    0 => {
                        byte.insert(1, '|');
                    }
                    x => {
                        byte.insert(x + 1, '|');
                    }
                }
            } else {
                byte.insert(2, '|');
            }
            byte
        })
        .collect::<Vec<String>>();
    let utf8_bytes_hex = utf8_bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>();
    println!(
        "\
        c: {c}\n\
        c_unicode_hex: {c_unicode_hex}\n\
        c_unicode_binary: {c_unicode_binary}\n\
        c_unicode_binary_split: {c_unicode_binary_split:?}\n\
        utf8_bytes_binary: {utf8_bytes_binary_visualized_from_unicode_binary:?}\n\
        utf8_bytes_hex: {utf8_bytes_hex:?}\n\
        utf8_bytes_decimal: {utf8_bytes:?}\
    "
    );
}
