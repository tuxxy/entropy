use std::io::Read;
use std::fs::File;

/// Contains metadata about the file that's being used in the calculation
pub struct Entropy {
    pub byte_freqs: [u64; 255],
    pub length: u64,
}

impl Entropy {
    pub fn new(filename: &String) -> Entropy {
        let f = &File::open(filename).expect("Couldn't open file.");

        let mut byte_freqs = [0u64; 255];
        for byte in f.bytes() {
            byte_freqs[byte.unwrap() as usize] += 1;
        }
        let length = f.metadata().unwrap().len().clone();

        Entropy {
            byte_freqs: byte_freqs,
            length: length,
        }
    }
}
