pub fn guess_single_xor_char_decode(bytes: &Vec<u8>) -> (u16, Vec<u8>, u8) {
    use english_scoring::*;
    use key::*;

    let mut top_score = 0;
    let mut top_decode = vec![];
    let mut top_x = 2;
    for x in 0..255 {
        let mut rk = RepeatingKey::new_bytes(&vec![x]);
        let decoded = rk.xor_with(&bytes);

        let score = score_combined(&decoded);
        if score > top_score {
            top_score = score;
            top_decode = decoded;
            top_x = x;
        }
    }

    (top_score, top_decode, top_x)
}

pub fn guess_repeating_xor_key(data_bytes: &Vec<u8>,
                               max_keysize: usize)
                               -> (u16, Vec<u8>, Vec<u8>) {
    // As described at http://cryptopals.com/sets/1/challenges/6/
    use byte_manipulation::*;
    use byte_utilities::*;
    use english_scoring::*;
    use general_utilities::*;
    use key::*;

    // Shortcut single character key guesses
    if max_keysize == 1 {
        let (a, b, c) = guess_single_xor_char_decode(&data_bytes);
        return (a, b, vec![c]);
    }

    // For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes,
    // and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
    // The KEYSIZE with the smallest normalized edit distance is probably the key.
    //  You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances.
    let mut keysize_points = Vec::new();
    for keysize in 2..max_keysize + 1 {
        if data_bytes.len() < keysize * 4 {
            continue;
        }

        let chunks: Vec<Vec<u8>> = data_bytes.chunks(keysize).map(|x| slice_to_vec(x)).collect();
        assert!(chunks.len() >= 4);

        let distance1 = hamming_distance(&chunks[0], &chunks[1]);
        let distance2 = hamming_distance(&chunks[1], &chunks[2]);
        let distance3 = hamming_distance(&chunks[2], &chunks[3]);
        let distance4 = hamming_distance(&chunks[3], &chunks[4]);
        let avg_distance: f32 = (distance1 + distance2 + distance3 + distance4) as f32 / 4.0;
        let normalized_distance = avg_distance / keysize as f32;

        let pair = (keysize, normalized_distance);
        keysize_points.push(pair);
    }

    // Sort into the best keysize values
    keysize_points.sort_by(|a, b| {
        use std::cmp::Ordering::*;

        let (_, x): (_, f32) = *a;
        let (_, y): (_, f32) = *b;

        // f32s require partial cmp
        x.partial_cmp(&y).unwrap_or(Equal)
    });

    let mut top_score = 0;
    let mut top_decode: Vec<u8> = Vec::new();
    let mut top_key: Vec<u8> = Vec::new();

    // Check the best 10, any more than that starts using up a lot of time.
    for y in 0..10 {
        if y >= keysize_points.len() {
            break;
        }
        let (best_keysize, _) = keysize_points[y];

        // Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
        let blocks: Vec<Vec<u8>> = data_bytes.chunks(best_keysize)
                                             .map(|x| slice_to_vec(x))
                                             .collect();

        // Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second byte of every block, and so on.
        let transposed_blocks: Vec<Vec<u8>> = transpose_chunks(&blocks);

        // Solve each block as if it was single-character XOR. You already have code to do this.
        // : Vec<(u16, String, u8)>
        let solved_key: Vec<_> = transposed_blocks.iter()
                                                  .map(|x| {
                                                      let (_, _, guess) =
                                                          guess_single_xor_char_decode(x);
                                                      guess
                                                  })
                                                  .collect();

        // For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block.
        // Put them together and you have the key.
        let mut repeating_key = RepeatingKey::new_bytes(&solved_key);
        let decoded = repeating_key.xor_with(&data_bytes);

        let score = score_combined(&decoded);
        if score > top_score {
            top_score = score;
            top_decode = decoded;
            top_key = solved_key;
        }
    }

    (top_score, top_decode, top_key)
}

pub fn detect_ecb(ciphertext: &[u8], key_size: usize, required_chunk_repeats: usize) -> bool {
    // ECB is easy to detect.
    // Simply cut your bytes up into key_size chunks, and see if any appear more than once.
    // https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#Electronic_Codebook_.28ECB.29
    use std::collections::HashMap;

    let mut chunk_counter: HashMap<&[u8], u8> = HashMap::new();

    for chunk in ciphertext.chunks(key_size) {
        let count = chunk_counter.entry(&chunk).or_insert(0);
        *count += 1;
    }

    for (_, v) in chunk_counter {
        if v >= required_chunk_repeats as u8 {
            return true;
        }
    }

    false
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn guess_single_xor_char_decode_i_set_1_problem_3() {
        // http://cryptopals.com/sets/1/challenges/3/
        use byte_conversion::*;
        let encoded = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

        let (top_score, guessed_decode, guessed_character) = guess_single_xor_char_decode(&encoded);

        let expected = readable_text_to_bytes(&"Cooking MC's like a pound of bacon");

        assert_eq!(expected, guessed_decode);
        assert_eq!(88, guessed_character);
        assert!(top_score > 0);
    }

    #[test]
    fn guess_single_xor_char_decode_i_example() {
        use byte_conversion::*;
        use key::*;

        let mut rk = RepeatingKey::new("Q");
        let example = readable_text_to_bytes(&"Some super hard to decrypt (but still English!) text.");

        let encrypted = rk.xor_with(&example);
        let (top_score, guessed_decode, guessed_character) =
            guess_single_xor_char_decode(&encrypted);

        assert_eq!(example, guessed_decode);
        assert_eq!('Q' as u8, guessed_character);
        assert!(top_score > 0);
    }

    #[test]
    fn guess_repeating_xor_key_decode_i_single_character() {
        use byte_conversion::*;
        use key::*;

        let example_key: Vec<u8> = readable_text_to_bytes(&"Q");
        let mut rk = RepeatingKey::new_bytes(&example_key);
        let example = readable_text_to_bytes(&"Some super hard to decrypt (but still English!) text.");

        let encrypted = rk.xor_with(&example);
        let (top_score, guessed_decode, guessed_key) = guess_repeating_xor_key(&encrypted, 1);

        assert_eq!(example, guessed_decode);
        assert_eq!(example_key, guessed_key);
        assert!(top_score > 0);
    }

    #[test]
    fn guess_repeating_xor_key_decode_i_two_characters() {
        use byte_conversion::*;
        use key::*;

        let example_key: Vec<u8> = readable_text_to_bytes(&"Qr");
        let mut rk = RepeatingKey::new_bytes(&example_key);
        let example = readable_text_to_bytes(&"Some super hard to decrypt (but still English!) text.");

        let encrypted = rk.xor_with(&example);
        let (top_score, guessed_decode, guessed_key) = guess_repeating_xor_key(&encrypted, 2);

        assert_eq!(example, guessed_decode);
        assert_eq!(example_key, guessed_key);
        assert!(top_score > 0);
    }

    #[test]
    fn guess_repeating_xor_key_decode_i_non_trivial_key() {
        // http://cryptopals.com/sets/1/challenges/6/
        use byte_conversion::*;
        use key::*;

        let example_key: Vec<u8> = readable_text_to_bytes(&"Terminator X: Bring the noise");
        let mut rk = RepeatingKey::new_bytes(&example_key);
        let example = readable_text_to_bytes(&"I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that\'s my DJ Deshay cuttin\' all them Z\'s \nHittin\' hard and the girlies goin\' crazy \nVanilla\'s on the mike, man I\'m not lazy. \n\nI\'m lettin\' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse\'s to the side yellin\', Go Vanilla Go! \n\nSmooth \'cause that\'s the way I will be \nAnd if youdon\'t give a damn, then \nWhy you starin\' at me \nSo get off \'cause I control the stage \nThere\'s no dissin\' allowed \nI\'m in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n\' play \n\nStage 2 -- Yea the one ya\' wanna listen to \nIt\'s off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI\'m an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI\'m like Samson -- Samson to Delilah \nThere\'s no denyin\', You can try to hang \nBut you\'ll keep tryin\' to get my style \nOver and over, practice makes perfect \nBut not if you\'re a loafer. \n\nYou\'ll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I\'m comin\' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin\' \nVanilla Ice is sellin\' and you people are buyin\' \n\'Cause why the freaks are jockin\' like Crazy Glue \nMovin\' and groovin\' trying to sing along \nAll through the ghetto groovin\' this here song \nNow you\'re amazed by the VIP posse. \n\nSteppin\' so hard like a German Nazi \nStartled by the bases hittin\' ground \nThere\'s no trippin\' on mine, I\'m just gettin\' down \nSparkamatic, I\'m hangin\' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n\'89 in my time! You, \'90 is my year. \n\nYou\'re weakenin\' fast, YO! and I can tell it \nYour body\'s gettin\' hot, so, so I can smell it \nSo don\'t be mad and don\'t be sad \n\'Cause the lyrics belong to ICE, You can call me Dad \nYou\'re pitchin\' a fit, so step back and endure \nLet the witch doctor, Ice, do thedance to cure \nSo come up close and don\'t be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that Iwas weak, Boy, you\'re dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you sayit, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n");

        let encrypted = rk.xor_with(&example);
        let (top_score, guessed_decode, guessed_key) = guess_repeating_xor_key(&encrypted, 29);

        assert_eq!(bytes_to_readable_text(&example),
                   bytes_to_readable_text(&guessed_decode));
        assert_eq!(example_key, guessed_key);
        assert!(top_score > 0);
    }

    #[test]
    fn detect_ecb_i_problem_7() {
        // http://cryptopals.com/sets/1/challenges/6/
        use byte_conversion::*;

        let example_hex = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a";
        let example_bytes = hex_to_bytes(example_hex);

        assert!(detect_ecb(&example_bytes[..], 16, 2));
    }

    #[test]
    fn detect_ecb_i_problem_7_non() {
        // http://cryptopals.com/sets/1/challenges/6/
        use byte_conversion::*;

        let example_hex = "5c4ca78f8de3527788e7d1efcd6aad0adc3878ea70993ae20937ef0a601730494946f078de2099c62de9af1c47ee4f18216ed5a7268464f210374dbf421d55449c8f399d8824c5a0ff8526a940223ee999a6f945f0ba3eaa672c434ad867ac7adaa46bd3289729c6c7d920dd0d8237bf678d88bde91e0683e72e88fef50bdb23cceb6270acba5aeebd0a834ccf99cd3e6bad8c158f5819f1f1c785fdaa3df505";
        let example_bytes = hex_to_bytes(example_hex);

        assert!(!detect_ecb(&example_bytes[..], 16, 2));
    }
}
