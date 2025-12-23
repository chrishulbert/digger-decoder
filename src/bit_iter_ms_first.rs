// Iterates through bits, most significant first.
pub struct BitIterMsFirst {
    bit: i8,
    byte: u8,
}
impl BitIterMsFirst {
    pub fn new(byte: &u8) -> BitIterMsFirst {
        BitIterMsFirst { bit: 7, byte: *byte }
    }
}
impl Iterator for BitIterMsFirst {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        let this_bit = self.bit;
        if this_bit >= 0 {
            self.bit -= 1;
            Some((self.byte >> this_bit) & 1)
        } else {
            None
        }
    }
}

// Creates a bit iterator from [u8].
pub fn iterate(arr: &[u8]) -> impl Iterator<Item = u8> {
    arr.iter().flat_map(BitIterMsFirst::new)
}
