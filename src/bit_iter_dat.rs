// This iterates bits in the order required when parsing lemmings DAT files.
// Order is reversed, and the first-returned byte has limited bits.
pub struct BitIterDat<'a> {
    data: &'a [u8],
    byte_index: isize,
    bit: u8, // 0=lsb.
    is_starting_byte: bool,
    starting_byte_bits: u8,
}

impl<'a> BitIterDat<'a> {
    pub fn new(data: &'a [u8], starting_byte_bits: u8) -> Self {
        if starting_byte_bits == 0 { // Skip initial (at end) byte for weirdly-compressed sections.
            return BitIterDat { data, byte_index: (data.len() - 2) as isize, bit: 0, is_starting_byte: true, starting_byte_bits: 8 }
        }
        BitIterDat { data, byte_index: (data.len() - 1) as isize, bit: 0, is_starting_byte: true, starting_byte_bits }
    }

    pub fn next_byte(&mut self) -> Option<u8> {
        let b1 = if self.next()? { 1 } else { 0 };
        let b2 = if self.next()? { 1 } else { 0 };
        let b3 = if self.next()? { 1 } else { 0 };
        let b4 = if self.next()? { 1 } else { 0 };
        let b5 = if self.next()? { 1 } else { 0 };
        let b6 = if self.next()? { 1 } else { 0 };
        let b7 = if self.next()? { 1 } else { 0 };
        let b8 = if self.next()? { 1 } else { 0 };
        Some((b1<<7) + (b2<<6) + (b3<<5) + (b4<<4) + (b5<<3) + (b6<<2) + (b7<<1) + b8)
    }

    pub fn next_bit(&mut self) -> Option<u8> {
        self.next().map(|b| if b { 1 } else { 0 })
    }
}

impl<'a> Iterator for BitIterDat<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.data.get(self.byte_index as usize)?;
        let is_set = (byte & (1 << self.bit)) != 0;
        self.bit += 1;
        let max_bits: u8 = if self.is_starting_byte { self.starting_byte_bits } else { 8 };
        if self.bit >= max_bits {
            self.byte_index -= 1;
            self.bit = 0;
            self.is_starting_byte = false;
        }
        Some(is_set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bitBitIterDat() {
        let input: Vec<u8> = vec![0b00001111, 0];

        let output1: Vec<bool> = BitIterDat::new(&input, 1).collect();
        assert_eq!(output1, vec![
            false,
            true, true, true, true, false, false, false, false,
        ]);

        let output2: Vec<bool> = BitIterDat::new(&input, 2).collect();
        assert_eq!(output2, vec![
            false, false,
            true, true, true, true, false, false, false, false,
        ]);

        let output8: Vec<bool> = BitIterDat::new(&input, 8).collect();
        assert_eq!(output8, vec![
            false, false, false, false, false, false, false, false,
            true, true, true, true, false, false, false, false,
        ]);
    }

    #[test]
    fn text_next_byte() {
        let input: Vec<u8> = vec![0b00001111, 0];
        assert_eq!(BitIterDat::new(&input, 1).next_byte().unwrap(), 0b01111000);
        assert_eq!(BitIterDat::new(&input, 2).next_byte().unwrap(), 0b00111100);
        assert_eq!(BitIterDat::new(&input, 3).next_byte().unwrap(), 0b00011110);
        assert_eq!(BitIterDat::new(&input, 4).next_byte().unwrap(), 0b00001111);
        assert_eq!(BitIterDat::new(&input, 5).next_byte().unwrap(), 0b00000111);
        assert_eq!(BitIterDat::new(&input, 6).next_byte().unwrap(), 0b00000011);
        assert_eq!(BitIterDat::new(&input, 7).next_byte().unwrap(), 0b00000001);
        assert_eq!(BitIterDat::new(&input, 8).next_byte().unwrap(), 0b00000000);
    }
}
