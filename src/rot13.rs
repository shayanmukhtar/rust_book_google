use std::io::Read;

struct RotDecoder<R> 
where
    R: Read
{
    input: R,
    rot: u8,
}

const SIZE_OF_ALPHABET: u8 = 26;
// Implement the `Read` trait for `RotDecoder`.
impl <R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // remember, self is of type R which already implements the Read trait
        let size: usize = self.input.read(buf)?;
        for b in &mut buf[..] {
            if b.is_ascii_alphabetic() {
                let base = if b.is_ascii_uppercase() {'A'} else {'a'} as u8;
                // now rotate the letter, keeping in mind the rollover past 26
                *b = (*b - base + self.rot) % 26 + base;
            }
        }

        Ok(size)
    }
}

pub fn test_rot13() {
    let mut rot =
        RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}