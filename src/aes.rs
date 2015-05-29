use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

pub fn decrypt_aes_128_ecb_no_padding(encrypted_data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    // https://github.com/DaGenix/rust-crypto/blob/master/examples/symmetriccipher.rs
    let mut decryptor = aes::ecb_decryptor(
        aes::KeySize::KeySize128,
        &key[..],
        blockmodes::NoPadding
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(&encrypted_data[..]);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}