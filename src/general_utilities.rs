pub fn read_file(location: &str) -> String {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    // Create a path to the desired file
    let path = Path::new(location);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_data = String::new();
    match file.read_to_string(&mut file_data) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {}
    }

    file_data
}

pub fn slice_to_vec<T: Copy>(slice: &[T]) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    for x in slice {
        result.push(x.clone());
    }

    result
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn slice_to_vec_i_empty() {
        let x: Vec<u8> = vec![];
        let a = slice_to_vec(&[]);
        assert_eq!(x, a);
    }

    #[test]
    fn slice_to_vec_i_u8() {
        let x: Vec<u8> = vec![5, 7];
        let a = slice_to_vec(&[5, 7]);
        assert_eq!(x, a);
    }

    #[test]
    fn slice_to_vec_i_u16() {
        let x: Vec<u16> = vec![7, 8];
        let a = slice_to_vec(&[7, 8]);
        assert_eq!(x, a);
    }
}
