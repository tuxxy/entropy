use std::io::Read;
use std::fs::File;

/// Contains metadata about the file that's being used in the Entropy calculation.
///
/// `byte_freqs` is a lookup table that contains the number of occurances of
/// a byte specified by the index, e.g. 0x00 is `byte_freqs[0]`.
///
/// `length` is the number of bytes in the file.
pub struct Entropy {
    pub byte_freqs: [u64; 255],
    pub length: u64,
}

impl Entropy {
    /// Gets metadata for the Entropy calculation from a File reference
    pub fn new(file: &File) -> Entropy {
        let mut byte_freqs = [0u64; 255];
        for byte in file.bytes() {
            byte_freqs[byte.unwrap() as usize] += 1
        }

        Entropy {
            byte_freqs: byte_freqs,
            length: file.metadata().unwrap().len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::tempfile;

    #[test]
    fn test_new() {
        // Create a temporary file and write five bytes to it
        let mut test_file = tempfile().unwrap();
        test_file.write(&[0x00, 0x00, 0x01, 0x01, 0x02]).unwrap();

        test_file.seek(SeekFrom::Start(0));
        let test_entropy = Entropy::new(&test_file);

        // Test that the frequency table was populated correctly
        assert_eq!(test_entropy.byte_freqs[0], 2);
        assert_eq!(test_entropy.byte_freqs[1], 2);
        assert_eq!(test_entropy.byte_freqs[2], 1);

        // Test the length
        assert_eq!(test_entropy.length, 5);
    }
}
