fn check_bit(left: &u8, right: &u8, bit: u8) -> u16 {
    let left_bit = left & ((1 << bit) as u8);
    let right_bit = right & ((1 << bit) as u8);

    if (left_bit ^ right_bit) > 0 {
        1
    } else {
        0
    }
}

pub fn hamming_distance(a: &Vec<u8>, b: &Vec<u8>) -> u16 {
    let mut count: u16 = 0;

    for (left, right) in a.iter().zip(b.iter()) {
        for bit in 0..8 {
            count += check_bit(&left, &right, bit);
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hamming_distance_no_distance() {
        assert_eq!(0, hamming_distance(&vec![5], &vec![5]));
    }

    #[test]
    fn hamming_distance_one_distance_1() {
        assert_eq!(1, hamming_distance(&vec![0], &vec![1]));
    }

    #[test]
    fn hamming_distance_one_distance_2() {
        assert_eq!(1, hamming_distance(&vec![4], &vec![5]));
    }

    #[test]
    fn hamming_distance_one_distance_3() {
        assert_eq!(1, hamming_distance(&vec![4], &vec![6]));
    }

    #[test]
    fn hamming_distance_two_distance_1() {
        assert_eq!(2, hamming_distance(&vec![4], &vec![7]));
    }

    #[test]
    fn hamming_distance_example() {
        use byte_conversion::*;
        let left = readable_text_to_bytes("this is a test");
        let right = readable_text_to_bytes("wokka wokka!!!");

        assert_eq!(37, hamming_distance(&left, &right));
    }
}
