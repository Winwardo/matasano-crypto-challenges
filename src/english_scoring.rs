pub fn score_on_word_length(string: &Vec<u8>) -> u16 {
    // Ensure a minimum of one space
    let mut string_with_space = string.clone();
    string_with_space.push(' ' as u8);

    // We'll simply score on how much whitespace there is
    let mut whitespace_count = 0;
    for c in &string_with_space {
        match *c as char {
            ' ' | '\t' | '\n' => whitespace_count += 1,
            _ => {}
        }
    }

    assert!(whitespace_count > 0);
    let average_word_length = string_with_space.len() / whitespace_count;

    let mut closeness_to_avg: f32 = average_word_length as f32 / 5.0;
    closeness_to_avg -= 1.0;
    if closeness_to_avg < 0.0 {
        closeness_to_avg = -closeness_to_avg;
    }

    let score_modifier: i32 = (closeness_to_avg * 40.0) as i32;
    let mut score: i32 = 255i32 - score_modifier;

    if score < 0 {
        score = 0;
    } else if score > 255 {
        score = 255;
    }

    score as u16
}

pub fn score_on_letter_frequency(string: &Vec<u8>) -> u16 {
    let mut frequency_table: Vec<u32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0];
    let example_frequency_table: Vec<f32> = vec![8.167, 1.492, 2.782, 4.253, 12.702, 2.228, 2.015,
                                                 6.094, 6.966, 0.153, 0.772, 4.025, 2.406, 6.749,
                                                 7.507, 1.929, 0.095, 5.987, 6.327, 9.056, 2.758,
                                                 0.978, 2.360, 0.150, 1.974, 0.074, 0.0];
    assert_eq!(frequency_table.len(), example_frequency_table.len());

    for byte in string {
        match *byte as char {
            'A'...'Z' => frequency_table[(byte - 65) as usize] += 1,
            'a'...'z' => frequency_table[(byte - 97) as usize] += 1,
            '\n' => {}
            e @ _ => {
                let y = e as u8;
                if y < 128 && y > 31 {
                    // Must be a valid character
                    frequency_table[26] += 1
                } else {
                    return 0;
                }
            }
        }
    }

    let max_value: u32 = *frequency_table.iter().max().unwrap();

    // Score tables out of 0 - 1
    let normalised_frequency_table: Vec<f32> = frequency_table.iter()
                                                              .map(|x| {
                                                                  (*x) as f32 / max_value as f32
                                                              })
                                                              .collect();
    let normalised_example_frequency_table: Vec<f32> = example_frequency_table.iter()
                                                                              .map(|x| {
                                                                                  (*x) as f32 /
                                                                                  12.702
                                                                              })
                                                                              .collect();

    // Create a similarity vector
    let mut similarity_vector: Vec<f32> = vec![];
    for pair in normalised_frequency_table.iter().zip(normalised_example_frequency_table.iter()) {
        let (a, b) = pair;
        similarity_vector.push(a - b);
    }

    let difference: f32 = similarity_vector.iter().fold(0.0, |acc, &item| acc + item).abs();
    (255.0 - difference * 5.0) as u16
}

pub fn score_combined(string: &Vec<u8>) -> u16 {
    let length_score = score_on_word_length(&string);
    let freq_score = score_on_letter_frequency(&string);

    if length_score == 0 || freq_score == 0 {
        return 0;
    }

    length_score as u16 + freq_score as u16
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score_on_word_length_i_longer() {
        let score_real = score_on_word_length(&"This is a realistic looking piece of text."
                                                   .bytes()
                                                   .collect());
        let score_fake = score_on_word_length(&"ejGD%5545 j48494$%i43i3fdg3AE22".bytes().collect());

        println!("{} {} ", score_real, score_fake);

        assert!(score_real > score_fake);
    }

    #[test]
    fn score_on_word_length_i_really_long() {
        let score_real = score_on_word_length(&"This is a realistic looking piece of text."
                                                   .bytes()
                                                   .collect());
        let score_fake = score_on_word_length(&"ejGD%5545j48494$%i43i3ffg5dg3AE22"
                                                   .bytes()
                                                   .collect());

        println!("{} {} ", score_real, score_fake);

        assert!(score_real > score_fake);
    }

    #[test]
    fn score_on_word_length_i_shorter() {
        let score_real = score_on_word_length(&"This is a realistic looking piece of text."
                                                   .bytes()
                                                   .collect());
        let score_fake = score_on_word_length(&"ej n 45 j 48 494 $% i4 3AE 22".bytes().collect());

        assert!(score_real > score_fake);
    }

    #[test]
    fn score_on_letter_frequency_i_longer() {
        let score_real = score_on_letter_frequency(&"This is a realistic looking piece of text."
                                                        .bytes()
                                                        .collect());
        let score_fake = score_on_letter_frequency(&"ejGD*%%%%5545 j48494$%i43i3fdg3AE22"
                                                        .bytes()
                                                        .collect());

        println!("{} {} ", score_real, score_fake);

        assert!(score_real > score_fake);
    }


    #[test]
    fn score_on_letter_frequency_i_weird_character() {
        let score_real = score_on_letter_frequency(&"This is a realistic looking piece of text."
                                                        .bytes()
                                                        .collect());
        let score_fake = score_on_letter_frequency(&"This is a realistic looking piece of text with a weird character in it: ☃".bytes().collect());

        println!("{} {} ", score_real, score_fake);

        assert!(score_real > score_fake);
    }
}
