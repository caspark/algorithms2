use itertools::*;
use std::io::prelude::*;
use std::io;

macro_rules! printerrln(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

/// Reads from stdin and performs move to front encoding, writing the result to stdout
pub fn encode() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut alphabet = (0..u8::max_value()).collect::<Vec<_>>();

    for in_byte in stdin_lock.bytes().map(|r| r.unwrap()) {
        let byte_pos = alphabet.iter().find_position(|&&a| a == in_byte)
            .expect("alphabet covers all byte values")
            .0; // discard the found letter
        let out_byte = byte_pos as u8;
        stdout_lock.write(&[out_byte]).unwrap();
        move_byte_to_front(alphabet.as_mut(), byte_pos);
    }
}

pub fn decode() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut alphabet = (0..u8::max_value()).collect::<Vec<_>>();

    for in_byte in stdin_lock.bytes().map(|r| r.unwrap()) {
        let byte_pos = in_byte as usize;
        let out_byte = alphabet[byte_pos];
        stdout_lock.write(&[out_byte]).unwrap();
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
    use super::move_byte_to_front;

    #[test]
    fn alphabet_is_updated_properly() {
        println!("breaking loose");
        let mut vec = vec![0, 1, 2, 3, 4, 5];
        move_byte_to_front(vec.as_mut(), 3);
        assert_eq!(vec, vec![3, 0, 1, 2, 4, 5]);
    }
}
