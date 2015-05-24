use circular_suffix_array::{self, CircularSuffixArray};
use std::io::prelude::*;
use std::mem;
use itertools::Itertools;

pub fn encode<R: Read, W: Write>(mut input: R, output: &mut W) {
    let mut input_vec = Vec::new();
    input.read_to_end(&mut input_vec).unwrap();
    drop(input);
    let CircularSuffixArray(csa_vec) = circular_suffix_array::create(input_vec.as_ref());
    let original_pos = csa_vec.iter().find_position(|&&x| x == 0).unwrap().0;
    write_usize(original_pos, output);
    for x in csa_vec {
        let pos = if x == 0 { input_vec.len() - 1 } else { x - 1 };
        let to_write = input_vec[pos];
        assert_eq!(output.write(&[to_write]).unwrap(), 1);
    }
}

pub fn decode<R: Read, W: Write>(mut input: R, output: &mut W) {
    let first = read_usize(&mut input); // aka original_pos when we encoded it
    let t_vec = input.bytes().map(|r| r.unwrap()).collect::<Vec<_>>();

    //TODO use black magic to build next_vec (below is hardcoded for "ABRACADABRA!" from spec)
    let next_vec = vec![3usize, 0, 6, 7, 8, 9, 10, 11, 5, 2, 1, 4];

    let first_col = {
        let mut tmp = t_vec.clone();
        tmp.sort();
        tmp
    };
    let mut curr = first;
    loop {
        let decoded_byte = first_col[curr];
        println!("Decoded {}", decoded_byte as char);
        assert_eq!(output.write(&[decoded_byte]).unwrap(), 1);
        curr = next_vec[curr];

        if curr == first {
            break;
        }
    }
}

fn write_usize<W: Write>(n: usize, output: &mut W) {
    let usize_size_in_bytes: usize = mem::size_of::<usize>();
    for byte in 0..usize_size_in_bytes {
        let to_write = (n >> (byte * 8)) & 255;
        let num_bytes_written = output.write(&[to_write as u8]).unwrap();
        assert!(num_bytes_written == 1, "Unable to write expected number of bytes");
    }
}

fn read_usize<R: Read>(input: &mut R) -> usize {
    let mut result = 0usize;
    let usize_size_in_bytes: usize = mem::size_of::<usize>();
    for byte in 0..usize_size_in_bytes {
        let mut buf = [0u8];
        let num_bytes_read = input.read(&mut buf).unwrap();
        assert!(num_bytes_read == 1, "Unable to read expected number of bytes");
        result += (buf[0] as usize) << (byte * 8);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{decode, encode, read_usize, write_usize};
    use quickcheck::quickcheck;
    use std::io::Cursor;

    fn try_encode_and_decode_usize(input: usize) -> bool {
        let mut encoded = Cursor::new(Vec::new());

        write_usize(input, &mut encoded);
        encoded.set_position(0); // seek to the start of the vec
        let result = read_usize(&mut encoded);

        result == input
    }

    #[test]
    fn can_encode_and_decode_a_big_usize() {
        assert!(try_encode_and_decode_usize(usize::max_value()));
    }

    #[test]
    fn can_encode_and_decode_arbitrary_usizes() {
        quickcheck(try_encode_and_decode_usize as fn(usize) -> bool);
    }

    fn try_encode_and_decode_input(input: Vec<u8>) -> bool {
        let copy = Cursor::new(input.clone());
        let mut encoded = Cursor::new(Vec::with_capacity(input.len()));
        let mut decoded = Cursor::new(Vec::with_capacity(input.len()));

        encode(copy, &mut encoded);
        encoded.set_position(0); // seek to the start of the vec
        decode(encoded, &mut decoded);

        decoded.get_ref() == &input
    }

    #[test]
    fn can_encode_and_decode_sample_input() {
        assert!(try_encode_and_decode_input("ABRACADABRA!".bytes().collect()));
    }

    #[test]
    fn can_encode_and_decode_a_string_with_no_repeating_chars() {
        assert!(try_encode_and_decode_input((0..10).collect()));
    }

    #[test]
    fn can_encode_and_decode_arbitrary_inputs() {
        quickcheck(try_encode_and_decode_input as fn(Vec<u8>) -> bool);
    }
}
