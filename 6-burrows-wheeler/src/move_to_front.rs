use itertools::*;
use std::io::prelude::*;

/// Reads from stdin and performs move to front encoding, writing the result to stdout
pub fn encode<R: Read, W: Write>(input: R, output: &mut W) {

    let mut alphabet = (0..u8::max_value()).collect::<Vec<_>>();

    for in_byte in input.bytes().map(|r| r.unwrap()) {
        let byte_pos = alphabet.iter().find_position(|&&a| a == in_byte)
            .expect("alphabet covers all byte values")
            .0; // discard the found letter
        let out_byte = byte_pos as u8;
        output.write(&[out_byte]).unwrap();
        move_byte_to_front(alphabet.as_mut(), byte_pos);
    }
}

pub fn decode<R: Read, W: Write>(input: R, output: &mut W) {
    let mut alphabet = (0..u8::max_value()).collect::<Vec<_>>();

    for in_byte in input.bytes().map(|r| r.unwrap()) {
        let byte_pos = in_byte as usize;
        let out_byte = alphabet[byte_pos];
        output.write(&[out_byte]).unwrap();
        move_byte_to_front(alphabet.as_mut(), byte_pos);
    }
}

fn move_byte_to_front(slice: &mut [u8], pos: usize) {
    if pos != 0 {
        for i in (1..(pos + 1)).rev() { // iterate from pos to 1
            slice.swap(i, i - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{encode, decode, move_byte_to_front};
    use std::io::Cursor;
    use quickcheck::quickcheck;

    #[test]
    fn alphabet_is_updated_properly() {
        println!("breaking loose");
        let mut vec = vec![0, 1, 2, 3, 4, 5];
        move_byte_to_front(vec.as_mut(), 3);
        assert_eq!(vec, vec![3, 0, 1, 2, 4, 5]);
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
    fn can_encode_and_decode_a_string_with_no_repeating_chars() {
        assert!(try_encode_and_decode_input((0..10).collect()));
    }

    #[test]
    fn can_encode_and_decode_arbitrary_inputs() {
        quickcheck(try_encode_and_decode_input as fn(Vec<u8>) -> bool);
    }

}
