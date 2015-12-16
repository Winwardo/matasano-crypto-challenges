use aes::*;
use block_cipher_operation::*;
use byte_conversion::*;
use byte_manipulation::*;
use ciphertext::*;
use key::*;

pub struct CBC {
    pub initialisation_vector: Vec<u8>,
    pub input: PaddedBytes,
    pub key: RepeatingKey,
}

impl BlockCipherOperation for CBC {
    fn encrypt(&mut self) -> Vec<u8> {
        let xored = xor_byte_streams(&self.initialisation_vector, self.input.vec());
        let key = self.key.of_length(16);

        encrypt_aes_128_ecb_no_padding(&xored[..], &key).unwrap()
    }

    fn decrypt(&mut self) -> Vec<u8> {
        let key = self.key.of_length(16);
        let decipher = decrypt_aes_128_ecb_no_padding(self.input.bytes(), &key).unwrap();

        xor_byte_streams(&self.initialisation_vector, &decipher)
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    use block_cipher_operation::*;
    use byte_conversion::*;
    use ciphertext::*;
    use key::*;

    #[test]
    fn RepeatingKey_sanity() {
        let mut cipher_block = CBC {
            input: PaddedBytes::from_text("Some text", 16).unwrap(),
            initialisation_vector: RepeatingKey::new(&"\x00").of_length(16),
            key: RepeatingKey::new(&"YELLOW SUBMARINE"),
        };

        assert_eq!(16, cipher_block.encrypt().len());
    }

    #[test]
    fn RepeatingKey_encrypt_decrypt_equal() {
        fn plaintext() -> PaddedBytes {
            PaddedBytes::from_text("Some text", 16).unwrap()
        };
        fn IV() -> Vec<u8> {
            RepeatingKey::new(&"\x00").of_length(16)
        };
        fn key() -> RepeatingKey {
            RepeatingKey::new(&"YELLOW SUBMARINE")
        };

        let mut cipher_block = CBC {
            input: plaintext(),
            initialisation_vector: IV(),
            key: key(),
        };

        let ciphertext = cipher_block.encrypt();

        let mut decrypt_cipher = CBC {
            input: PaddedBytes::from_bytes(&ciphertext[..], 16).unwrap(),
            initialisation_vector: IV(),
            key: key(),
        };

        assert_eq!(plaintext().vec(), &decrypt_cipher.decrypt());
    }
}
