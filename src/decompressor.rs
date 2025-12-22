// This is for decompressing lemmings DAT files:
// https://www.camanis.net/lemmings/files/docs/lemmings_dat_file_format.txt

use crate::reverse_bit_iterator;

// Decompresses a dat file into sections.
pub fn decompress(dat: &[u8]) -> Vec<Vec<u8>> {
    decompose_into_compressed_sections(dat)
        .iter()
        .map(|s| s.decompress())
        .collect()
}

// Parses a DAT file into it's sections with their headers and compressed data.
fn decompose_into_compressed_sections(dat: &[u8]) -> Vec<CompressedSection<'_>> {
    let mut sections: Vec<CompressedSection> = Vec::new();
    let mut offset: usize = 0;
    loop {
        let section = CompressedSection::new(&dat[offset..]);
        offset += section.compressed_data_size;
        sections.push(section);
        if offset >= dat.len() {
            break
        }
    }
    sections
}

struct CompressedSection<'a> {
    num_bits_in_first_byte: u8,
    _checksum: u8,
    decompressed_data_size: usize,
    compressed_data_size: usize, // Includes the 10byte header.
    data: &'a [u8],
}

impl<'a> CompressedSection<'a> {
    fn new(data: &'a [u8]) -> CompressedSection<'a> {
        let num_bits_in_first_byte = data[0];
        let _checksum = data[1];
        let decompressed_data_size: usize = ((data[4] as usize) << 8) + (data[5] as usize);
        let compressed_data_size: usize = ((data[8] as usize) << 8) + (data[9] as usize);
        let data = &data[10..compressed_data_size];
        CompressedSection {
            num_bits_in_first_byte,
            _checksum,
            decompressed_data_size,
            compressed_data_size,
            data,
        }
    }

    fn decompress(&self) -> Vec<u8> {
        // if offset=0, returns the end byte.
        // if offset=1, returns the one just before the end, and so on.
        fn from_end(vec: &[u8], offset: usize) -> u8 {
            vec[vec.len() - 1 - offset]
        }

        let mut bits = reverse_bit_iterator::ReverseBitIterator::new(self.data, self.num_bits_in_first_byte);
        let mut decompressed: Vec<u8> = Vec::new();
        while decompressed.len() < self.decompressed_data_size as usize {
            match bits.next().unwrap() {
                false => {
                    match bits.next().unwrap() {
                        false => { // 1: some raw bytes
                            let n1 = bits.next_bit().unwrap();
                            let n2 = bits.next_bit().unwrap();
                            let n3 = bits.next_bit().unwrap();
                            let n = (n1 << 2) + (n2 << 1) + n3 + 1;
                            for _ in 0..n {
                                decompressed.push(bits.next_byte().unwrap());
                            }
                        },
                        true => { // 2: Reuse 2 bytes.
                            let offset = bits.next_byte().unwrap() as usize;
                            for _ in 0..2 {
                                let b = from_end(&decompressed, offset);
                                decompressed.push(b);
                            }
                        }
                    }
                },
                true => {
                    let b = bits.next().unwrap();
                    let c = bits.next().unwrap();
                    match (b, c) {
                        (false, false) => { // 3: reuse 3 bytes.
                            let m1 = bits.next_bit().unwrap();
                            let m2 = bits.next_byte().unwrap();
                            let offset: usize = ((m1 as usize) << 8) + (m2 as usize);
                            for _ in 0..3 {
                                let b = from_end(&decompressed, offset);
                                decompressed.push(b);
                            }
                        },
                        (false, true) => { // 4: reuse 4 bytes.
                            let m1 = bits.next_bit().unwrap();
                            let m2 = bits.next_bit().unwrap();
                            let m3 = bits.next_byte().unwrap();
                            let offset: usize = ((m1 as usize) << 9) + ((m2 as usize) << 8) + (m3 as usize);
                            for _ in 0..4 {
                                let b = from_end(&decompressed, offset);
                                decompressed.push(b);
                            }
                        },
                        (true, false) => { // 5: reuse N bytes.
                            let n = bits.next_byte().unwrap();
                            let length: u16 = n as u16 + 1;
                            let m1 = bits.next_bit().unwrap();
                            let m2 = bits.next_bit().unwrap();
                            let m3 = bits.next_bit().unwrap();
                            let m4 = bits.next_bit().unwrap();
                            let m5 = bits.next_byte().unwrap();
                            let offset: usize = ((m1 as usize) << 11) + ((m2 as usize) << 10) + ((m3 as usize) << 9) + ((m4 as usize) << 8) + (m5 as usize);
                            for _ in 0..length {
                                let b = from_end(&decompressed, offset);
                                decompressed.push(b);
                            }
                        },
                        (true, true) => { // 6: many raw bytes.
                            let n = bits.next_byte().unwrap();
                            let length: u16 = n as u16 + 9;
                            for _ in 0..length {
                                decompressed.push(bits.next_byte().unwrap());
                            }
                        }
                    }
                }
            }
        }
        decompressed.reverse();
        decompressed
    }
}
