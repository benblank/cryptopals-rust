const FORWARD_S_BOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const INVERSE_S_BOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

const ROUND_CONSTANTS: [u8; 11] = [
    0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36,
];

fn add_round_key(state: &mut [u8], key_schedule: &[u8], round: usize) {
    for i in 0..16 {
        state[i] ^= key_schedule[i + round * 16];
    }
}

fn get_key_schedule(key: &[u8]) -> Result<Vec<u8>, String> {
    let key_length = key.len();
    let size = match key_length {
        16 => 176,
        24 => 208,
        32 => 240,
        _ => return Err(format!("Invalid key length: {}", key_length)),
    };
    let mut expanded_key: Vec<u8> = Vec::with_capacity(size);

    for i in (0..size).step_by(4) {
        if i < key_length {
            expanded_key.push(key[i]);
            expanded_key.push(key[i + 1]);
            expanded_key.push(key[i + 2]);
            expanded_key.push(key[i + 3]);
        } else if i % key_length == 0 {
            // Note that this branch implements the rotation operation when
            // lookup up indices in `expanded_key`: 0 => -3, 1 => -2, 2 => -1,
            // and 3 => -4.
            expanded_key.push(
                expanded_key[i - key_length]
                    ^ FORWARD_S_BOX[expanded_key[i - 3] as usize]
                    ^ ROUND_CONSTANTS[i / key_length],
            );
            expanded_key.push(
                expanded_key[i + 1 - key_length] ^ FORWARD_S_BOX[expanded_key[i - 2] as usize],
            );
            expanded_key.push(
                expanded_key[i + 2 - key_length] ^ FORWARD_S_BOX[expanded_key[i - 1] as usize],
            );
            expanded_key.push(
                expanded_key[i + 3 - key_length] ^ FORWARD_S_BOX[expanded_key[i - 4] as usize],
            );
        } else if key_length > 24 && i % key_length == 16 {
            expanded_key
                .push(expanded_key[i - key_length] ^ FORWARD_S_BOX[expanded_key[i - 4] as usize]);
            expanded_key.push(
                expanded_key[i + 1 - key_length] ^ FORWARD_S_BOX[expanded_key[i - 3] as usize],
            );
            expanded_key.push(
                expanded_key[i + 2 - key_length] ^ FORWARD_S_BOX[expanded_key[i - 2] as usize],
            );
            expanded_key.push(
                expanded_key[i + 3 - key_length] ^ FORWARD_S_BOX[expanded_key[i - 1] as usize],
            );
        } else {
            expanded_key.push(expanded_key[i - key_length] ^ expanded_key[i - 4]);
            expanded_key.push(expanded_key[i + 1 - key_length] ^ expanded_key[i - 3]);
            expanded_key.push(expanded_key[i + 2 - key_length] ^ expanded_key[i - 2]);
            expanded_key.push(expanded_key[i + 3 - key_length] ^ expanded_key[i - 1]);
        };
    }

    Ok(expanded_key)
}

fn gmul(a: u8, b: u8) -> u8 {
    let mut a = a;
    let mut b = b;
    let mut product = 0;

    for _ in 0..8 {
        if b & 0x01 == 0x01 {
            product ^= a;
        }

        let high_bit_set = a & 0x80 == 0x80;

        a <<= 1;

        if high_bit_set {
            a ^= 0x1b;
        }

        b >>= 1;
    }

    product
}

fn inverse_mix_column(column: &mut [u8]) {
    let copy = column.to_vec();

    column[0] = gmul(copy[0], 14) ^ gmul(copy[3], 9) ^ gmul(copy[2], 13) ^ gmul(copy[1], 11);
    column[1] = gmul(copy[1], 14) ^ gmul(copy[0], 9) ^ gmul(copy[3], 13) ^ gmul(copy[2], 11);
    column[2] = gmul(copy[2], 14) ^ gmul(copy[1], 9) ^ gmul(copy[0], 13) ^ gmul(copy[3], 11);
    column[3] = gmul(copy[3], 14) ^ gmul(copy[2], 9) ^ gmul(copy[1], 13) ^ gmul(copy[0], 11);
}

fn inverse_mix_columns(state: &mut [u8]) {
    for i in (0..16).step_by(4) {
        inverse_mix_column(&mut state[i..(i + 4)]);
    }
}

fn inverse_shift_rows(state: &mut [u8]) {
    let temp = state[1];
    state[1] = state[13];
    state[13] = state[9];
    state[9] = state[5];
    state[5] = temp;

    state.swap(2, 10);
    state.swap(6, 14);

    let temp = state[3];
    state[3] = state[7];
    state[7] = state[11];
    state[11] = state[15];
    state[15] = temp;
}

fn mix_column(column: &mut [u8]) {
    let copy = column.to_vec();
    let mut doubles = [0, 0, 0, 0];

    for i in 0..4 {
        let h = if column[i] & 0x80 == 0x80 { 0x1b } else { 0x00 };

        doubles[i] = (column[i] << 1) ^ h;
    }

    // This is slightly optimized from the gmul version and is equivalent to:
    // column[0] = gmul(copy[0], 2) ^ gmul(copy[3], 1) ^ gmul(copy[2], 1) ^ gmul(copy[1], 3);
    // column[1] = gmul(copy[1], 2) ^ gmul(copy[0], 1) ^ gmul(copy[3], 1) ^ gmul(copy[2], 3);
    // column[2] = gmul(copy[2], 2) ^ gmul(copy[1], 1) ^ gmul(copy[0], 1) ^ gmul(copy[3], 3);
    // column[3] = gmul(copy[3], 2) ^ gmul(copy[2], 1) ^ gmul(copy[1], 1) ^ gmul(copy[0], 3);
    column[0] = doubles[0] ^ copy[3] ^ copy[2] ^ doubles[1] ^ copy[1];
    column[1] = doubles[1] ^ copy[0] ^ copy[3] ^ doubles[2] ^ copy[2];
    column[2] = doubles[2] ^ copy[1] ^ copy[0] ^ doubles[3] ^ copy[3];
    column[3] = doubles[3] ^ copy[2] ^ copy[1] ^ doubles[0] ^ copy[0];
}

fn mix_columns(state: &mut [u8]) {
    for i in (0..16).step_by(4) {
        mix_column(&mut state[i..(i + 4)]);
    }
}

fn shift_rows(state: &mut [u8]) {
    let temp = state[1];
    state[1] = state[5];
    state[5] = state[9];
    state[9] = state[13];
    state[13] = temp;

    state.swap(2, 10);
    state.swap(6, 14);

    let temp = state[3];
    state[3] = state[15];
    state[15] = state[11];
    state[11] = state[7];
    state[7] = temp;
}

fn sub_bytes(state: &mut [u8], s_box: &[u8]) {
    for i in 0..16 {
        state[i] = s_box[state[i] as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_round_key_works() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.

        #[rustfmt::skip]
        let mut state = vec![
            0x3e, 0xc0, 0x8b, 0x72, 0xdd, 0x6a, 0xec, 0x9b, 0x52, 0xaa, 0x86, 0xac, 0xda, 0xc7, 0xeb, 0xb1,
        ];
        #[rustfmt::skip]
        let key_schedule = vec![
            0xb1, 0xf6, 0x90, 0x6f, 0x7e, 0x4a, 0xcf, 0x08, 0x6d, 0xf7, 0x51, 0x20, 0xb6, 0x90, 0xd0, 0x7e,
            0x1d, 0x12, 0x51, 0xef, 0x59, 0x76, 0x6a, 0xde, 0x84, 0x18, 0x1c, 0x3d, 0x4b, 0x7d, 0x1b, 0x13,
            0x53, 0x36, 0xf3, 0x15, 0xb4, 0xcb, 0x2a, 0x34, 0xe7, 0x3d, 0x46, 0xc1, 0x74, 0xc2, 0x37, 0x60,
            0xf8, 0x87, 0x4a, 0x83, 0x99, 0x29, 0xd2, 0x2c, 0x0a, 0x2f, 0x70, 0xc8, 0x37, 0x1d, 0x2e, 0x09,
            0x98, 0xa4, 0xf3, 0xc1, 0x95, 0x2d, 0x7c, 0xcb, 0xca, 0x8d, 0x5a, 0xef, 0xe7, 0xfd, 0xfe, 0x1d,
            0xda, 0xb1, 0x1d, 0xb7, 0xa1, 0x67, 0x27, 0xdb, 0x00, 0x82, 0xcd, 0xf2, 0x31, 0x7b, 0x8c, 0x3b,
            0x04, 0x54, 0x58, 0x3a, 0xd1, 0x58, 0xd3, 0x5c, 0x61, 0x84, 0x3f, 0x91, 0xc3, 0xe5, 0xa8, 0x8b,

            // Round key 7.
            0xf8, 0x0d, 0x37, 0xe6, 0x50, 0xe2, 0x34, 0x87, 0x44, 0xfb, 0x2d, 0x23, 0xf5, 0x69, 0x58, 0x28,

            0x83, 0xa4, 0x46, 0x38, 0x1a, 0xb0, 0x4e, 0x99, 0xb9, 0x37, 0xb5, 0x1b, 0x49, 0x59, 0x9e, 0x26,
            0xcd, 0x08, 0x00, 0xd3, 0x2d, 0x3b, 0x84, 0xa8, 0x18, 0x9e, 0x9f, 0xe5, 0x68, 0x6b, 0x68, 0xa8,
            0xcf, 0x38, 0x31, 0xbb, 0x18, 0xd8, 0xe1, 0x82, 0x3f, 0x02, 0x8a, 0xaa, 0x0c, 0xe8, 0x6e, 0xfb,
        ];

        add_round_key(&mut state, &key_schedule, 7);

        #[rustfmt::skip]
        assert_eq!(state, vec![
            0xc6, 0xcd, 0xbc, 0x94, 0x8d, 0x88, 0xd8, 0x1c, 0x16, 0x51, 0xab, 0x8f, 0x2f, 0xae, 0xb3, 0x99,
        ]);
    }

    #[test]
    fn get_key_schedule_128_all_00() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(get_key_schedule(&vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]).unwrap(), vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63,
            0x9b, 0x98, 0x98, 0xc9, 0xf9, 0xfb, 0xfb, 0xaa, 0x9b, 0x98, 0x98, 0xc9, 0xf9, 0xfb, 0xfb, 0xaa,
            0x90, 0x97, 0x34, 0x50, 0x69, 0x6c, 0xcf, 0xfa, 0xf2, 0xf4, 0x57, 0x33, 0x0b, 0x0f, 0xac, 0x99,
            0xee, 0x06, 0xda, 0x7b, 0x87, 0x6a, 0x15, 0x81, 0x75, 0x9e, 0x42, 0xb2, 0x7e, 0x91, 0xee, 0x2b,
            0x7f, 0x2e, 0x2b, 0x88, 0xf8, 0x44, 0x3e, 0x09, 0x8d, 0xda, 0x7c, 0xbb, 0xf3, 0x4b, 0x92, 0x90,
            0xec, 0x61, 0x4b, 0x85, 0x14, 0x25, 0x75, 0x8c, 0x99, 0xff, 0x09, 0x37, 0x6a, 0xb4, 0x9b, 0xa7,
            0x21, 0x75, 0x17, 0x87, 0x35, 0x50, 0x62, 0x0b, 0xac, 0xaf, 0x6b, 0x3c, 0xc6, 0x1b, 0xf0, 0x9b,
            0x0e, 0xf9, 0x03, 0x33, 0x3b, 0xa9, 0x61, 0x38, 0x97, 0x06, 0x0a, 0x04, 0x51, 0x1d, 0xfa, 0x9f,
            0xb1, 0xd4, 0xd8, 0xe2, 0x8a, 0x7d, 0xb9, 0xda, 0x1d, 0x7b, 0xb3, 0xde, 0x4c, 0x66, 0x49, 0x41,
            0xb4, 0xef, 0x5b, 0xcb, 0x3e, 0x92, 0xe2, 0x11, 0x23, 0xe9, 0x51, 0xcf, 0x6f, 0x8f, 0x18, 0x8e,
        ]);
    }

    #[test]
    fn get_key_schedule_128_all_ff() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(get_key_schedule(&vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ]).unwrap(), vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xe8, 0xe9, 0xe9, 0xe9, 0x17, 0x16, 0x16, 0x16, 0xe8, 0xe9, 0xe9, 0xe9, 0x17, 0x16, 0x16, 0x16,
            0xad, 0xae, 0xae, 0x19, 0xba, 0xb8, 0xb8, 0x0f, 0x52, 0x51, 0x51, 0xe6, 0x45, 0x47, 0x47, 0xf0,
            0x09, 0x0e, 0x22, 0x77, 0xb3, 0xb6, 0x9a, 0x78, 0xe1, 0xe7, 0xcb, 0x9e, 0xa4, 0xa0, 0x8c, 0x6e,
            0xe1, 0x6a, 0xbd, 0x3e, 0x52, 0xdc, 0x27, 0x46, 0xb3, 0x3b, 0xec, 0xd8, 0x17, 0x9b, 0x60, 0xb6,
            0xe5, 0xba, 0xf3, 0xce, 0xb7, 0x66, 0xd4, 0x88, 0x04, 0x5d, 0x38, 0x50, 0x13, 0xc6, 0x58, 0xe6,
            0x71, 0xd0, 0x7d, 0xb3, 0xc6, 0xb6, 0xa9, 0x3b, 0xc2, 0xeb, 0x91, 0x6b, 0xd1, 0x2d, 0xc9, 0x8d,
            0xe9, 0x0d, 0x20, 0x8d, 0x2f, 0xbb, 0x89, 0xb6, 0xed, 0x50, 0x18, 0xdd, 0x3c, 0x7d, 0xd1, 0x50,
            0x96, 0x33, 0x73, 0x66, 0xb9, 0x88, 0xfa, 0xd0, 0x54, 0xd8, 0xe2, 0x0d, 0x68, 0xa5, 0x33, 0x5d,
            0x8b, 0xf0, 0x3f, 0x23, 0x32, 0x78, 0xc5, 0xf3, 0x66, 0xa0, 0x27, 0xfe, 0x0e, 0x05, 0x14, 0xa3,
            0xd6, 0x0a, 0x35, 0x88, 0xe4, 0x72, 0xf0, 0x7b, 0x82, 0xd2, 0xd7, 0x85, 0x8c, 0xd7, 0xc3, 0x26,
        ]);
    }

    #[test]
    fn get_key_schedule_128_counting() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(get_key_schedule(&vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        ]).unwrap(), vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0xd6, 0xaa, 0x74, 0xfd, 0xd2, 0xaf, 0x72, 0xfa, 0xda, 0xa6, 0x78, 0xf1, 0xd6, 0xab, 0x76, 0xfe,
            0xb6, 0x92, 0xcf, 0x0b, 0x64, 0x3d, 0xbd, 0xf1, 0xbe, 0x9b, 0xc5, 0x00, 0x68, 0x30, 0xb3, 0xfe,
            0xb6, 0xff, 0x74, 0x4e, 0xd2, 0xc2, 0xc9, 0xbf, 0x6c, 0x59, 0x0c, 0xbf, 0x04, 0x69, 0xbf, 0x41,
            0x47, 0xf7, 0xf7, 0xbc, 0x95, 0x35, 0x3e, 0x03, 0xf9, 0x6c, 0x32, 0xbc, 0xfd, 0x05, 0x8d, 0xfd,
            0x3c, 0xaa, 0xa3, 0xe8, 0xa9, 0x9f, 0x9d, 0xeb, 0x50, 0xf3, 0xaf, 0x57, 0xad, 0xf6, 0x22, 0xaa,
            0x5e, 0x39, 0x0f, 0x7d, 0xf7, 0xa6, 0x92, 0x96, 0xa7, 0x55, 0x3d, 0xc1, 0x0a, 0xa3, 0x1f, 0x6b,
            0x14, 0xf9, 0x70, 0x1a, 0xe3, 0x5f, 0xe2, 0x8c, 0x44, 0x0a, 0xdf, 0x4d, 0x4e, 0xa9, 0xc0, 0x26,
            0x47, 0x43, 0x87, 0x35, 0xa4, 0x1c, 0x65, 0xb9, 0xe0, 0x16, 0xba, 0xf4, 0xae, 0xbf, 0x7a, 0xd2,
            0x54, 0x99, 0x32, 0xd1, 0xf0, 0x85, 0x57, 0x68, 0x10, 0x93, 0xed, 0x9c, 0xbe, 0x2c, 0x97, 0x4e,
            0x13, 0x11, 0x1d, 0x7f, 0xe3, 0x94, 0x4a, 0x17, 0xf3, 0x07, 0xa7, 0x8b, 0x4d, 0x2b, 0x30, 0xc5,
        ]);
    }

    #[test]
    fn get_key_schedule_128_arbitrary() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(get_key_schedule(&vec![
            0x69, 0x20, 0xe2, 0x99, 0xa5, 0x20, 0x2a, 0x6d, 0x65, 0x6e, 0x63, 0x68, 0x69, 0x74, 0x6f, 0x2a,
        ]).unwrap(), vec![
            0x69, 0x20, 0xe2, 0x99, 0xa5, 0x20, 0x2a, 0x6d, 0x65, 0x6e, 0x63, 0x68, 0x69, 0x74, 0x6f, 0x2a,
            0xfa, 0x88, 0x07, 0x60, 0x5f, 0xa8, 0x2d, 0x0d, 0x3a, 0xc6, 0x4e, 0x65, 0x53, 0xb2, 0x21, 0x4f,
            0xcf, 0x75, 0x83, 0x8d, 0x90, 0xdd, 0xae, 0x80, 0xaa, 0x1b, 0xe0, 0xe5, 0xf9, 0xa9, 0xc1, 0xaa,
            0x18, 0x0d, 0x2f, 0x14, 0x88, 0xd0, 0x81, 0x94, 0x22, 0xcb, 0x61, 0x71, 0xdb, 0x62, 0xa0, 0xdb,
            0xba, 0xed, 0x96, 0xad, 0x32, 0x3d, 0x17, 0x39, 0x10, 0xf6, 0x76, 0x48, 0xcb, 0x94, 0xd6, 0x93,
            0x88, 0x1b, 0x4a, 0xb2, 0xba, 0x26, 0x5d, 0x8b, 0xaa, 0xd0, 0x2b, 0xc3, 0x61, 0x44, 0xfd, 0x50,
            0xb3, 0x4f, 0x19, 0x5d, 0x09, 0x69, 0x44, 0xd6, 0xa3, 0xb9, 0x6f, 0x15, 0xc2, 0xfd, 0x92, 0x45,
            0xa7, 0x00, 0x77, 0x78, 0xae, 0x69, 0x33, 0xae, 0x0d, 0xd0, 0x5c, 0xbb, 0xcf, 0x2d, 0xce, 0xfe,
            0xff, 0x8b, 0xcc, 0xf2, 0x51, 0xe2, 0xff, 0x5c, 0x5c, 0x32, 0xa3, 0xe7, 0x93, 0x1f, 0x6d, 0x19,
            0x24, 0xb7, 0x18, 0x2e, 0x75, 0x55, 0xe7, 0x72, 0x29, 0x67, 0x44, 0x95, 0xba, 0x78, 0x29, 0x8c,
            0xae, 0x12, 0x7c, 0xda, 0xdb, 0x47, 0x9b, 0xa8, 0xf2, 0x20, 0xdf, 0x3d, 0x48, 0x58, 0xf6, 0xb1,
        ]);
    }

    #[test]
    fn get_key_schedule_192_all_00() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(get_key_schedule(&vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]).unwrap(), vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63,
            0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63,
            0x9b, 0x98, 0x98, 0xc9, 0xf9, 0xfb, 0xfb, 0xaa, 0x9b, 0x98, 0x98, 0xc9, 0xf9, 0xfb, 0xfb, 0xaa,
            0x9b, 0x98, 0x98, 0xc9, 0xf9, 0xfb, 0xfb, 0xaa, 0x90, 0x97, 0x34, 0x50, 0x69, 0x6c, 0xcf, 0xfa,
            0xf2, 0xf4, 0x57, 0x33, 0x0b, 0x0f, 0xac, 0x99, 0x90, 0x97, 0x34, 0x50, 0x69, 0x6c, 0xcf, 0xfa,
            0xc8, 0x1d, 0x19, 0xa9, 0xa1, 0x71, 0xd6, 0x53, 0x53, 0x85, 0x81, 0x60, 0x58, 0x8a, 0x2d, 0xf9,
            0xc8, 0x1d, 0x19, 0xa9, 0xa1, 0x71, 0xd6, 0x53, 0x7b, 0xeb, 0xf4, 0x9b, 0xda, 0x9a, 0x22, 0xc8,
            0x89, 0x1f, 0xa3, 0xa8, 0xd1, 0x95, 0x8e, 0x51, 0x19, 0x88, 0x97, 0xf8, 0xb8, 0xf9, 0x41, 0xab,
            0xc2, 0x68, 0x96, 0xf7, 0x18, 0xf2, 0xb4, 0x3f, 0x91, 0xed, 0x17, 0x97, 0x40, 0x78, 0x99, 0xc6,
            0x59, 0xf0, 0x0e, 0x3e, 0xe1, 0x09, 0x4f, 0x95, 0x83, 0xec, 0xbc, 0x0f, 0x9b, 0x1e, 0x08, 0x30,
            0x0a, 0xf3, 0x1f, 0xa7, 0x4a, 0x8b, 0x86, 0x61, 0x13, 0x7b, 0x88, 0x5f, 0xf2, 0x72, 0xc7, 0xca,
            0x43, 0x2a, 0xc8, 0x86, 0xd8, 0x34, 0xc0, 0xb6, 0xd2, 0xc7, 0xdf, 0x11, 0x98, 0x4c, 0x59, 0x70,
        ]);
    }

    #[test]
    fn get_key_schedule_192_all_ff() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(get_key_schedule(&vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ]).unwrap(), vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xe8, 0xe9, 0xe9, 0xe9, 0x17, 0x16, 0x16, 0x16,
            0xe8, 0xe9, 0xe9, 0xe9, 0x17, 0x16, 0x16, 0x16, 0xe8, 0xe9, 0xe9, 0xe9, 0x17, 0x16, 0x16, 0x16,
            0xad, 0xae, 0xae, 0x19, 0xba, 0xb8, 0xb8, 0x0f, 0x52, 0x51, 0x51, 0xe6, 0x45, 0x47, 0x47, 0xf0,
            0xad, 0xae, 0xae, 0x19, 0xba, 0xb8, 0xb8, 0x0f, 0xc5, 0xc2, 0xd8, 0xed, 0x7f, 0x7a, 0x60, 0xe2,
            0x2d, 0x2b, 0x31, 0x04, 0x68, 0x6c, 0x76, 0xf4, 0xc5, 0xc2, 0xd8, 0xed, 0x7f, 0x7a, 0x60, 0xe2,
            0x17, 0x12, 0x40, 0x3f, 0x68, 0x68, 0x20, 0xdd, 0x45, 0x43, 0x11, 0xd9, 0x2d, 0x2f, 0x67, 0x2d,
            0xe8, 0xed, 0xbf, 0xc0, 0x97, 0x97, 0xdf, 0x22, 0x8f, 0x8c, 0xd3, 0xb7, 0xe7, 0xe4, 0xf3, 0x6a,
            0xa2, 0xa7, 0xe2, 0xb3, 0x8f, 0x88, 0x85, 0x9e, 0x67, 0x65, 0x3a, 0x5e, 0xf0, 0xf2, 0xe5, 0x7c,
            0x26, 0x55, 0xc3, 0x3b, 0xc1, 0xb1, 0x30, 0x51, 0x63, 0x16, 0xd2, 0xe2, 0xec, 0x9e, 0x57, 0x7c,
            0x8b, 0xfb, 0x6d, 0x22, 0x7b, 0x09, 0x88, 0x5e, 0x67, 0x91, 0x9b, 0x1a, 0xa6, 0x20, 0xab, 0x4b,
            0xc5, 0x36, 0x79, 0xa9, 0x29, 0xa8, 0x2e, 0xd5, 0xa2, 0x53, 0x43, 0xf7, 0xd9, 0x5a, 0xcb, 0xa9,
            0x59, 0x8e, 0x48, 0x2f, 0xff, 0xae, 0xe3, 0x64, 0x3a, 0x98, 0x9a, 0xcd, 0x13, 0x30, 0xb4, 0x18,
        ]);
    }

    #[test]
    fn get_key_schedule_192_counting() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(get_key_schedule(&vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        ]).unwrap(), vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x58, 0x46, 0xf2, 0xf9, 0x5c, 0x43, 0xf4, 0xfe,
            0x54, 0x4a, 0xfe, 0xf5, 0x58, 0x47, 0xf0, 0xfa, 0x48, 0x56, 0xe2, 0xe9, 0x5c, 0x43, 0xf4, 0xfe,
            0x40, 0xf9, 0x49, 0xb3, 0x1c, 0xba, 0xbd, 0x4d, 0x48, 0xf0, 0x43, 0xb8, 0x10, 0xb7, 0xb3, 0x42,
            0x58, 0xe1, 0x51, 0xab, 0x04, 0xa2, 0xa5, 0x55, 0x7e, 0xff, 0xb5, 0x41, 0x62, 0x45, 0x08, 0x0c,
            0x2a, 0xb5, 0x4b, 0xb4, 0x3a, 0x02, 0xf8, 0xf6, 0x62, 0xe3, 0xa9, 0x5d, 0x66, 0x41, 0x0c, 0x08,
            0xf5, 0x01, 0x85, 0x72, 0x97, 0x44, 0x8d, 0x7e, 0xbd, 0xf1, 0xc6, 0xca, 0x87, 0xf3, 0x3e, 0x3c,
            0xe5, 0x10, 0x97, 0x61, 0x83, 0x51, 0x9b, 0x69, 0x34, 0x15, 0x7c, 0x9e, 0xa3, 0x51, 0xf1, 0xe0,
            0x1e, 0xa0, 0x37, 0x2a, 0x99, 0x53, 0x09, 0x16, 0x7c, 0x43, 0x9e, 0x77, 0xff, 0x12, 0x05, 0x1e,
            0xdd, 0x7e, 0x0e, 0x88, 0x7e, 0x2f, 0xff, 0x68, 0x60, 0x8f, 0xc8, 0x42, 0xf9, 0xdc, 0xc1, 0x54,
            0x85, 0x9f, 0x5f, 0x23, 0x7a, 0x8d, 0x5a, 0x3d, 0xc0, 0xc0, 0x29, 0x52, 0xbe, 0xef, 0xd6, 0x3a,
            0xde, 0x60, 0x1e, 0x78, 0x27, 0xbc, 0xdf, 0x2c, 0xa2, 0x23, 0x80, 0x0f, 0xd8, 0xae, 0xda, 0x32,
            0xa4, 0x97, 0x0a, 0x33, 0x1a, 0x78, 0xdc, 0x09, 0xc4, 0x18, 0xc2, 0x71, 0xe3, 0xa4, 0x1d, 0x5d,
        ]);
    }

    #[test]
    fn get_key_schedule_256_all_00() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(get_key_schedule(&vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]).unwrap(), vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63,
            0xaa, 0xfb, 0xfb, 0xfb, 0xaa, 0xfb, 0xfb, 0xfb, 0xaa, 0xfb, 0xfb, 0xfb, 0xaa, 0xfb, 0xfb, 0xfb,
            0x6f, 0x6c, 0x6c, 0xcf, 0x0d, 0x0f, 0x0f, 0xac, 0x6f, 0x6c, 0x6c, 0xcf, 0x0d, 0x0f, 0x0f, 0xac,
            0x7d, 0x8d, 0x8d, 0x6a, 0xd7, 0x76, 0x76, 0x91, 0x7d, 0x8d, 0x8d, 0x6a, 0xd7, 0x76, 0x76, 0x91,
            0x53, 0x54, 0xed, 0xc1, 0x5e, 0x5b, 0xe2, 0x6d, 0x31, 0x37, 0x8e, 0xa2, 0x3c, 0x38, 0x81, 0x0e,
            0x96, 0x8a, 0x81, 0xc1, 0x41, 0xfc, 0xf7, 0x50, 0x3c, 0x71, 0x7a, 0x3a, 0xeb, 0x07, 0x0c, 0xab,
            0x9e, 0xaa, 0x8f, 0x28, 0xc0, 0xf1, 0x6d, 0x45, 0xf1, 0xc6, 0xe3, 0xe7, 0xcd, 0xfe, 0x62, 0xe9,
            0x2b, 0x31, 0x2b, 0xdf, 0x6a, 0xcd, 0xdc, 0x8f, 0x56, 0xbc, 0xa6, 0xb5, 0xbd, 0xbb, 0xaa, 0x1e,
            0x64, 0x06, 0xfd, 0x52, 0xa4, 0xf7, 0x90, 0x17, 0x55, 0x31, 0x73, 0xf0, 0x98, 0xcf, 0x11, 0x19,
            0x6d, 0xbb, 0xa9, 0x0b, 0x07, 0x76, 0x75, 0x84, 0x51, 0xca, 0xd3, 0x31, 0xec, 0x71, 0x79, 0x2f,
            0xe7, 0xb0, 0xe8, 0x9c, 0x43, 0x47, 0x78, 0x8b, 0x16, 0x76, 0x0b, 0x7b, 0x8e, 0xb9, 0x1a, 0x62,
            0x74, 0xed, 0x0b, 0xa1, 0x73, 0x9b, 0x7e, 0x25, 0x22, 0x51, 0xad, 0x14, 0xce, 0x20, 0xd4, 0x3b,
            0x10, 0xf8, 0x0a, 0x17, 0x53, 0xbf, 0x72, 0x9c, 0x45, 0xc9, 0x79, 0xe7, 0xcb, 0x70, 0x63, 0x85,
        ]);
    }

    #[test]
    fn get_key_schedule_256_all_ff() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(get_key_schedule(&vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ]).unwrap(), vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xe8, 0xe9, 0xe9, 0xe9, 0x17, 0x16, 0x16, 0x16, 0xe8, 0xe9, 0xe9, 0xe9, 0x17, 0x16, 0x16, 0x16,
            0x0f, 0xb8, 0xb8, 0xb8, 0xf0, 0x47, 0x47, 0x47, 0x0f, 0xb8, 0xb8, 0xb8, 0xf0, 0x47, 0x47, 0x47,
            0x4a, 0x49, 0x49, 0x65, 0x5d, 0x5f, 0x5f, 0x73, 0xb5, 0xb6, 0xb6, 0x9a, 0xa2, 0xa0, 0xa0, 0x8c,
            0x35, 0x58, 0x58, 0xdc, 0xc5, 0x1f, 0x1f, 0x9b, 0xca, 0xa7, 0xa7, 0x23, 0x3a, 0xe0, 0xe0, 0x64,
            0xaf, 0xa8, 0x0a, 0xe5, 0xf2, 0xf7, 0x55, 0x96, 0x47, 0x41, 0xe3, 0x0c, 0xe5, 0xe1, 0x43, 0x80,
            0xec, 0xa0, 0x42, 0x11, 0x29, 0xbf, 0x5d, 0x8a, 0xe3, 0x18, 0xfa, 0xa9, 0xd9, 0xf8, 0x1a, 0xcd,
            0xe6, 0x0a, 0xb7, 0xd0, 0x14, 0xfd, 0xe2, 0x46, 0x53, 0xbc, 0x01, 0x4a, 0xb6, 0x5d, 0x42, 0xca,
            0xa2, 0xec, 0x6e, 0x65, 0x8b, 0x53, 0x33, 0xef, 0x68, 0x4b, 0xc9, 0x46, 0xb1, 0xb3, 0xd3, 0x8b,
            0x9b, 0x6c, 0x8a, 0x18, 0x8f, 0x91, 0x68, 0x5e, 0xdc, 0x2d, 0x69, 0x14, 0x6a, 0x70, 0x2b, 0xde,
            0xa0, 0xbd, 0x9f, 0x78, 0x2b, 0xee, 0xac, 0x97, 0x43, 0xa5, 0x65, 0xd1, 0xf2, 0x16, 0xb6, 0x5a,
            0xfc, 0x22, 0x34, 0x91, 0x73, 0xb3, 0x5c, 0xcf, 0xaf, 0x9e, 0x35, 0xdb, 0xc5, 0xee, 0x1e, 0x05,
            0x06, 0x95, 0xed, 0x13, 0x2d, 0x7b, 0x41, 0x84, 0x6e, 0xde, 0x24, 0x55, 0x9c, 0xc8, 0x92, 0x0f,
            0x54, 0x6d, 0x42, 0x4f, 0x27, 0xde, 0x1e, 0x80, 0x88, 0x40, 0x2b, 0x5b, 0x4d, 0xae, 0x35, 0x5e,
        ]);
    }

    #[test]
    fn get_key_schedule_256_counting() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(get_key_schedule(&vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ]).unwrap(), vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
            0xa5, 0x73, 0xc2, 0x9f, 0xa1, 0x76, 0xc4, 0x98, 0xa9, 0x7f, 0xce, 0x93, 0xa5, 0x72, 0xc0, 0x9c,
            0x16, 0x51, 0xa8, 0xcd, 0x02, 0x44, 0xbe, 0xda, 0x1a, 0x5d, 0xa4, 0xc1, 0x06, 0x40, 0xba, 0xde,
            0xae, 0x87, 0xdf, 0xf0, 0x0f, 0xf1, 0x1b, 0x68, 0xa6, 0x8e, 0xd5, 0xfb, 0x03, 0xfc, 0x15, 0x67,
            0x6d, 0xe1, 0xf1, 0x48, 0x6f, 0xa5, 0x4f, 0x92, 0x75, 0xf8, 0xeb, 0x53, 0x73, 0xb8, 0x51, 0x8d,
            0xc6, 0x56, 0x82, 0x7f, 0xc9, 0xa7, 0x99, 0x17, 0x6f, 0x29, 0x4c, 0xec, 0x6c, 0xd5, 0x59, 0x8b,
            0x3d, 0xe2, 0x3a, 0x75, 0x52, 0x47, 0x75, 0xe7, 0x27, 0xbf, 0x9e, 0xb4, 0x54, 0x07, 0xcf, 0x39,
            0x0b, 0xdc, 0x90, 0x5f, 0xc2, 0x7b, 0x09, 0x48, 0xad, 0x52, 0x45, 0xa4, 0xc1, 0x87, 0x1c, 0x2f,
            0x45, 0xf5, 0xa6, 0x60, 0x17, 0xb2, 0xd3, 0x87, 0x30, 0x0d, 0x4d, 0x33, 0x64, 0x0a, 0x82, 0x0a,
            0x7c, 0xcf, 0xf7, 0x1c, 0xbe, 0xb4, 0xfe, 0x54, 0x13, 0xe6, 0xbb, 0xf0, 0xd2, 0x61, 0xa7, 0xdf,
            0xf0, 0x1a, 0xfa, 0xfe, 0xe7, 0xa8, 0x29, 0x79, 0xd7, 0xa5, 0x64, 0x4a, 0xb3, 0xaf, 0xe6, 0x40,
            0x25, 0x41, 0xfe, 0x71, 0x9b, 0xf5, 0x00, 0x25, 0x88, 0x13, 0xbb, 0xd5, 0x5a, 0x72, 0x1c, 0x0a,
            0x4e, 0x5a, 0x66, 0x99, 0xa9, 0xf2, 0x4f, 0xe0, 0x7e, 0x57, 0x2b, 0xaa, 0xcd, 0xf8, 0xcd, 0xea,
            0x24, 0xfc, 0x79, 0xcc, 0xbf, 0x09, 0x79, 0xe9, 0x37, 0x1a, 0xc2, 0x3c, 0x6d, 0x68, 0xde, 0x36,
        ]);
    }

    #[test]
    fn gmul_1() {
        assert_eq!(gmul(0x12, 0x34), 0x05);
    }

    #[test]
    fn gmul_2() {
        assert_eq!(gmul(0x43, 0x21), 0xfb);
    }

    #[test]
    fn gmul_3() {
        assert_eq!(gmul(0xde, 0xad), 0xf1);
    }

    #[test]
    fn inverse_mix_column_1() {
        let mut column = [0x8e, 0x4d, 0xa1, 0xbc];

        inverse_mix_column(&mut column);

        assert_eq!(column, [0xdb, 0x13, 0x53, 0x45]);
    }

    #[test]
    fn inverse_mix_column_2() {
        let mut column = [0x9f, 0xdc, 0x58, 0x9d];

        inverse_mix_column(&mut column);

        assert_eq!(column, [0xf2, 0x0a, 0x22, 0x5c]);
    }

    #[test]
    fn inverse_mix_column_3() {
        let mut column = [0x01, 0x01, 0x01, 0x01];

        inverse_mix_column(&mut column);

        assert_eq!(column, [0x01, 0x01, 0x01, 0x01]);
    }

    #[test]
    fn inverse_mix_column_4() {
        let mut column = [0xc6, 0xc6, 0xc6, 0xc6];

        inverse_mix_column(&mut column);

        assert_eq!(column, [0xc6, 0xc6, 0xc6, 0xc6]);
    }

    #[test]
    fn inverse_mix_column_5() {
        let mut column = [0xd5, 0xd5, 0xd7, 0xd6];

        inverse_mix_column(&mut column);

        assert_eq!(column, [0xd4, 0xd4, 0xd4, 0xd5]);
    }

    #[test]
    fn inverse_mix_column_6() {
        let mut column = [0x4d, 0x7e, 0xbd, 0xf8];

        inverse_mix_column(&mut column);

        assert_eq!(column, [0x2d, 0x26, 0x31, 0x4c]);
    }

    #[test]
    fn inverse_shift_rows_works() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.

        #[rustfmt::skip]
        let mut state = vec![
            0x00, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b,
            0x0c, 0x0d, 0x0e, 0x0f,
        ];

        inverse_shift_rows(&mut state);

        #[rustfmt::skip]
        assert_eq!(state, vec![
            0x00, 0x0d, 0x0a, 0x07,
            0x04, 0x01, 0x0e, 0x0b,
            0x08, 0x05, 0x02, 0x0f,
            0x0c, 0x09, 0x06, 0x03,
        ]);
    }

    #[test]
    fn mix_column_1() {
        let mut column = [0xdb, 0x13, 0x53, 0x45];

        mix_column(&mut column);

        assert_eq!(column, [0x8e, 0x4d, 0xa1, 0xbc]);
    }

    #[test]
    fn mix_column_2() {
        let mut column = [0xf2, 0x0a, 0x22, 0x5c];

        mix_column(&mut column);

        assert_eq!(column, [0x9f, 0xdc, 0x58, 0x9d]);
    }

    #[test]
    fn mix_column_3() {
        let mut column = [0x01, 0x01, 0x01, 0x01];

        mix_column(&mut column);

        assert_eq!(column, [0x01, 0x01, 0x01, 0x01]);
    }

    #[test]
    fn mix_column_4() {
        let mut column = [0xc6, 0xc6, 0xc6, 0xc6];

        mix_column(&mut column);

        assert_eq!(column, [0xc6, 0xc6, 0xc6, 0xc6]);
    }

    #[test]
    fn mix_column_5() {
        let mut column = [0xd4, 0xd4, 0xd4, 0xd5];

        mix_column(&mut column);

        assert_eq!(column, [0xd5, 0xd5, 0xd7, 0xd6]);
    }

    #[test]
    fn mix_column_6() {
        let mut column = [0x2d, 0x26, 0x31, 0x4c];

        mix_column(&mut column);

        assert_eq!(column, [0x4d, 0x7e, 0xbd, 0xf8]);
    }

    #[test]
    fn shift_rows_works() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.

        #[rustfmt::skip]
        let mut state = vec![
            0x00, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b,
            0x0c, 0x0d, 0x0e, 0x0f,
        ];

        shift_rows(&mut state);

        #[rustfmt::skip]
        assert_eq!(state, vec![
            0x00, 0x05, 0x0a, 0x0f,
            0x04, 0x09, 0x0e, 0x03,
            0x08, 0x0d, 0x02, 0x07,
            0x0c, 0x01, 0x06, 0x0b,
        ]);
    }

    #[test]
    fn sub_bytes_works() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.

        #[rustfmt::skip]
        let mut state = vec![
            0x98, 0x59, 0x86, 0xc8, 0x0c, 0x12, 0x10, 0xac, 0xba, 0x8f, 0xb8, 0x0b, 0x5e, 0x69, 0x20, 0x75,
        ];

        sub_bytes(&mut state, &INVERSE_S_BOX);

        #[rustfmt::skip]
        assert_eq!(state, vec![
            0xe2, 0x15, 0xdc, 0xb1, 0x81, 0x39, 0x7c, 0xaa, 0xc0, 0x73, 0x9a, 0x9e, 0x9d, 0xe4, 0x54, 0x3f,
        ]);
    }
}
