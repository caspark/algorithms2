use circular_suffix_array::{self, CircularSuffixArray};
use std::io;
use std::io::prelude::*;
use std::mem;
use itertools::Itertools;

pub fn encode<R: Read, W: Write>(mut input: R, output: &mut W) -> io::Result<()> {
    let mut input_vec = Vec::new();
    try!(input.read_to_end(&mut input_vec));
    drop(input);
    if input_vec.len() == 0 {
        return Ok(());
    }
    let CircularSuffixArray(csa_vec) = circular_suffix_array::create(input_vec.as_ref());
    let original_pos = csa_vec.iter().find_position(|&&x| x == 0).unwrap().0;
    try!(write_usize(original_pos, output));
    for x in csa_vec {
        let pos = if x == 0 { input_vec.len() - 1 } else { x - 1 };
        let to_write = input_vec[pos];
        assert_eq!(output.write(&[to_write]).unwrap(), 1);
    }
    Ok(())
}

pub fn decode<R: Read, W: Write>(mut input: R, output: &mut W) -> io::Result<()> {
    let first = match read_usize(&mut input) { // aka original_pos when we encoded it
        Ok(f) => f,
        Err(err) => if err.kind() == io::ErrorKind::NotFound {
            return Ok(()); // seems like there's no data coming down the pipe
        } else {
            return Err(err)
        }
    };
    let t_vec = input.bytes().map(|r| r.unwrap()).collect::<Vec<_>>();
    let next_vec = key_indexed_count(&t_vec);
    {
        let mut curr = next_vec[first];
        for _ in 0..t_vec.len() {
            let decoded_byte = t_vec[curr as usize];
            assert_eq!(try!(output.write(&[decoded_byte])), 1);
            curr = next_vec[curr as usize];
        }
    }

    Ok(())
}

fn write_usize<W: Write>(n: usize, output: &mut W) -> io::Result<()> {
    let usize_size_in_bytes: usize = mem::size_of::<usize>();
    for byte in 0..usize_size_in_bytes {
        let to_write = (n >> (byte * 8)) & 255;
        let num_bytes_written = try!(output.write(&[to_write as u8]));
        if num_bytes_written != 1 {
            return Err(io::Error::new(io::ErrorKind::Other, "Unable to write expected number of bytes to encode a usize"));
        }
    }
    Ok(())
}

fn read_usize<R: Read>(input: &mut R) -> io::Result<usize> {
    let mut result = 0usize;
    let usize_size_in_bytes: usize = mem::size_of::<usize>();
    for byte in 0..usize_size_in_bytes {
        let mut buf = [0u8];
        let num_bytes_read = try!(input.read(&mut buf));
        if num_bytes_read != 1 {
            if byte == 0 {
                return Err(io::Error::new(io::ErrorKind::NotFound, "Cannot decode a usize; Read appears to be empty"));
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, "Unable to read expected number of bytes to decode a usize"));
            }
        }
        assert!(num_bytes_read == 1, "Unable to read expected number of bytes");
        result += (buf[0] as usize) << (byte * 8);
    }
    Ok(result)
}

// Similar to key indexed sorting.
fn key_indexed_count(input: &Vec<u8>) -> Vec<usize> {
    const R: usize = 256; // size of the alphabet (aka u8::max_value() + 1)
    let mut aux = vec![0; input.len()];
    let mut count = [0usize; R + 1];
    for i in 0..input.len() {
        count[input[i] as usize + 1] += 1;
    }
    for r in 0..R {
        count[r + 1] += count[r];
    }
    for i in 0..input.len() {
        aux[count[input[i] as usize]] = i;
        count[input[i] as usize] += 1;
    }
    aux
}

#[cfg(test)]
mod tests {
    use super::{decode, encode, key_indexed_count, read_usize, write_usize};
    use quickcheck::quickcheck;
    use std::io::Cursor;

    #[test]
    fn can_build_next_vec_for_sample_input() {
        let sample_t_vec = "ARD!RCAAAABB"; // from example in spec (encoded form of "ABRACADABRA!")
        let result = key_indexed_count(&sample_t_vec.bytes().collect());
        assert_eq!(result, vec![3, 0, 6, 7, 8, 9, 10, 11, 5, 2, 1, 4]);
    }

    fn try_encode_and_decode_usize(input: usize) -> bool {
        let mut encoded = Cursor::new(Vec::new());

        write_usize(input, &mut encoded).unwrap();
        encoded.set_position(0); // seek to the start of the vec
        let result = read_usize(&mut encoded).unwrap();

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

        encode(copy, &mut encoded).unwrap();
        encoded.set_position(0); // seek to the start of the vec
        decode(encoded, &mut decoded).unwrap();

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
