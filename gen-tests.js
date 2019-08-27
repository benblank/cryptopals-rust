'use strict';

const crypto = require('crypto');
const fs = require('fs');

function formatHex(raw) {
    const cooked = [];

    for (let i = 0; i < raw.length; i += 2) {
        cooked.push(`0x${raw.substr(i, 2)},`);
    }

    return cooked.join(' ');
}

const tests = [];

for (let i = 0; i < 4; i++) {
    const key = crypto.randomBytes(32);
    const plaintext = crypto.randomBytes(16);
    let cipher, ciphertext;

    cipher = crypto.createCipheriv('aes-128-ecb', key.slice(0, 16), null);
    cipher.setAutoPadding(false);
    ciphertext = cipher.update(plaintext, null, 'hex');
    ciphertext += cipher.final('hex');

    tests.push(`
    #[test]
    fn decrypt_block_128_${i + 1}() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(decrypt_block(&vec![
            ${formatHex(ciphertext)}
        ], &vec![
            ${formatHex(key.slice(0, 16).toString('hex'))}
        ]), Ok(vec![
            ${formatHex(plaintext.toString('hex'))}
        ]));
    }
`);

    tests.push(`
    #[test]
    fn encrypt_block_128_${i + 1}() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(encrypt_block(&vec![
            ${formatHex(plaintext.toString('hex'))}
        ], &vec![
            ${formatHex(key.slice(0, 16).toString('hex'))}
        ]), Ok(vec![
            ${formatHex(ciphertext)}
        ]));
    }
`);

    cipher = crypto.createCipheriv('aes-192-ecb', key.slice(0, 24), null);
    cipher.setAutoPadding(false);
    ciphertext = cipher.update(plaintext, null, 'hex');
    ciphertext += cipher.final('hex');

    tests.push(`
    #[test]
    fn decrypt_block_192_${i + 1}() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(decrypt_block(&vec![
            ${formatHex(ciphertext)}
        ], &vec![
            ${formatHex(key.slice(0, 24).toString('hex'))}
        ]), Ok(vec![
            ${formatHex(plaintext.toString('hex'))}
        ]));
    }
`);

    tests.push(`
    #[test]
    fn encrypt_block_192_${i + 1}() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(encrypt_block(&vec![
            ${formatHex(plaintext.toString('hex'))}
        ], &vec![
            ${formatHex(key.slice(0, 24).toString('hex'))}
        ]), Ok(vec![
            ${formatHex(ciphertext)}
        ]));
    }
`);

    cipher = crypto.createCipheriv('aes-256-ecb', key, null);
    cipher.setAutoPadding(false);
    ciphertext = cipher.update(plaintext, null, 'hex');
    ciphertext += cipher.final('hex');

    tests.push(`
    #[test]
    fn decrypt_block_256_${i + 1}() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(decrypt_block(&vec![
            ${formatHex(ciphertext)}
        ], &vec![
            ${formatHex(key.toString('hex'))}
        ]), Ok(vec![
            ${formatHex(plaintext.toString('hex'))}
        ]));
    }
`);

    tests.push(`
    #[test]
    fn encrypt_block_256_${i + 1}() {
        // rustfmt will wrap at 14 bytes, which makes these tables harder to read.
        #[rustfmt::skip]
        assert_eq!(encrypt_block(&vec![
            ${formatHex(plaintext.toString('hex'))}
        ], &vec![
            ${formatHex(key.toString('hex'))}
        ]), Ok(vec![
            ${formatHex(ciphertext)}
        ]));
    }
`);
}

tests.sort();

fs.writeFileSync('tests.rs', tests.join(''));
