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
    let first_col = key_indexed_sort(&t_vec);

    let next_vec = {
        let mut v = Vec::with_capacity(t_vec.len());
        let mut last_byte_and_pos = None;
        for &first_col_byte in first_col.iter() {
            let start_pos = match last_byte_and_pos {//TODO not using pattern matching is probably nicer
                None => {
                    0
                },
                Some((last_byte, last_pos)) => if last_byte == first_col_byte {
                    (last_pos + 1) % t_vec.len()
                } else {
                    0
                }
            };
            //FIXME this will run in O(N*N), which is too slow
            for t_col_idx in start_pos..t_vec.len() {
                let t_col_byte = t_vec[t_col_idx];
                if first_col_byte == t_col_byte {
                    last_byte_and_pos = Some((t_col_byte, t_col_idx));
                    v.push(t_col_idx);
                    break;
                }
            }
        }
        v
    };

    let mut curr = first;
    loop {
        let decoded_byte = first_col[curr];
        assert_eq!(try!(output.write(&[decoded_byte])), 1);
        curr = next_vec[curr];

        if curr == first {
            break;
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

fn key_indexed_sort(input: &Vec<u8>) -> Vec<u8> {
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
        aux[count[input[i] as usize]] = input[i];
        count[input[i] as usize] += 1;
    }
    aux
}

#[cfg(test)]
mod tests {
    use super::{decode, encode, key_indexed_sort, read_usize, write_usize};
    use quickcheck::quickcheck;
    use std::io::Cursor;

    fn try_sort(mut input: Vec<u8>) -> bool {
        let result = key_indexed_sort(&input);

        input.sort();

        input == result // because result is now sorted
    }

    #[test]
    fn can_sort_sample_input() {
        assert!(try_sort("ABRACADABRA!".bytes().collect()));
    }

    #[test]
    fn can_sort_arbitrary_inputs() {
        quickcheck(try_sort as fn(Vec<u8>) -> bool);
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
