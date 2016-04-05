use std::io::{Read, Result};

pub trait Wichard: Sized {
    fn wichard(self) -> WichardReader<Self>;
}

impl<T: Read> Wichard for T {
    fn wichard(self) -> WichardReader<Self> {
        WichardReader {
            inner: self
        }
    }
}

pub struct WichardReader<T> {
    inner: T
}

impl<T: Read> Read for WichardReader<T> {    
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.inner.read(buf) {
            Err(e) => Err(e),
            Ok(bytes) => {
                for byte in buf.iter_mut() {
                    match byte {
                        &mut b'r' => *byte = b'w',
                        &mut b'R' => *byte = b'W',
                        _ => (),
                    }
                }
                
                Ok(bytes)
            }
        }
    }
}