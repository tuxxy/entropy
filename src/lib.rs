use std::io::Read;
use std::fs::File;

/// Contains metadata about the file that's being used in the Entropy calculation.
///
/// `byte_count` is a lookup table that contains the number of occurances of
/// a byte specified by the index, e.g. 0x00 is `byte_count[0]`.
///
/// `length` is the number of bytes in the file.
pub struct Entropy {
    pub byte_count: [u64; 256],
    pub length: u64,
}

impl Entropy {
    /// Gets metadata for the Entropy calculation from a File reference
    pub fn new(file: &File) -> Entropy {
        let mut byte_count = [0u64; 256];
        for byte in file.bytes() {
            byte_count[byte.unwrap() as usize] += 1
        }

        Entropy {
            byte_count: byte_count,
            length: file.metadata().unwrap().len(),
        }
    }

    /// Measures the Shannon entropy based on the frequency table and returns
    /// it as a float.
    ///
    /// The equation is defined as: H(X) = - \sum_{i=0}^{n} P(x_i) log_2 P(x_i)
    /// It can be described as the minimum number of bits (per symbol) to encode
    /// the input. Thus the output will be between 0 and 8.
    /// See https://en.wikipedia.org/wiki/Entropy_(information_theory) for
    /// more information.
    pub fn shannon_entropy(&self) -> f32 {
        let mut entropy = 0.0f32;
        for count in self.byte_count.iter() {
            if count == &0u64 {
                continue;
            } else {
                let symbol_probability = *count as f32 / self.length as f32;
                entropy += symbol_probability * symbol_probability.log2();
            }
        }
        -entropy
    }

    /// Measures the metric entropy based on the Shannon entropy of the
    /// generated frequency table and returns it as a float between 0 and 1.
    ///
    /// Metric entropy is derived by dividing the Shannon entropy by the length
    /// of the string being measured.
    /// It can be described as the uncertainty or randomness of a string, where
    /// 1 means information is uniformly distributed across the string.
    pub fn metric_entropy(&self) -> f32 {
        self.shannon_entropy() / 8f32
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
        assert_eq!(test_entropy.byte_count[0], 2);
        assert_eq!(test_entropy.byte_count[1], 2);
        assert_eq!(test_entropy.byte_count[2], 1);

        // Test the length
        assert_eq!(test_entropy.length, 5);
    }

    #[test]
    fn test_shannon_entropy() {
        // Create a temporary file and write five bytes to it
        let mut test_file = tempfile().unwrap();
        test_file.write(&[0x00, 0x00, 0x01, 0x01, 0x02]).unwrap();

        test_file.seek(SeekFrom::Start(0));
        let test_entropy = Entropy::new(&test_file);

        let shannon_entropy = test_entropy.shannon_entropy();
        assert_eq!(shannon_entropy, 1.5219281);
    }

    #[test]
    fn test_metric_entropy() {
        // Create a temporary file and write five bytes to it
        let mut test_file = tempfile().unwrap();
        test_file.write(&[0x00, 0x00, 0x01, 0x01, 0x02]).unwrap();

        test_file.seek(SeekFrom::Start(0));
        let test_entropy = Entropy::new(&test_file);

        let metric_entropy = test_entropy.metric_entropy();
        assert_eq!(metric_entropy, 0.19024101);
    }
}
