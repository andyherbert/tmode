use crate::ascii;
use std::io::{stdin, Read};
use std::fs::File;
use std::error::Error;

pub fn read_file_to_bytes(file: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes = Vec::new();
    let mut file = File::open(file)?;
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub fn read_stdin_to_bytes() -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes = Vec::new();
    stdin().read_to_end(&mut bytes)?;
    Ok(bytes)
}

#[derive(Debug)]
pub enum BytesError {
    TooLarge,
}

impl std::fmt::Display for BytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BytesError::TooLarge => writeln!(f, "Bytes too large"),
        }
    }
}

impl Error for BytesError {}

pub trait VecOps<T> {
    fn strip_trailing_value(&mut self, value: T) -> &Vec<T>;
    fn strip_trailing_spaces(&mut self) -> &Vec<T>;
    fn strip_trailing_null(&mut self) -> &Vec<T>;
    fn pad_with_value(&mut self, length: usize, value: T) -> &Self;
    fn pad_with_spaces(&mut self, length: usize) -> &Self;
    fn pad_with_null(&mut self, length: usize) -> &Self;
    fn write_to_slice(&self, slice: &mut [u8]) -> Result<(), Box<dyn Error>>;
}

pub trait AsUSize {
    fn as_usize(&self) -> usize;
}

pub trait PackToBytes {
    fn pack_to_bytes(&self, bytes: &mut[u8]);
}

impl VecOps<u8> for Vec<u8> {
    fn strip_trailing_value(&mut self, value: u8) -> &Vec<u8> {
        while let Some(last) = self.last() {
            if *last == value {
                self.pop();
            } else {
                break;
            }
        }
        self
    }

    fn strip_trailing_spaces(&mut self) -> &Vec<u8> {
        self.strip_trailing_value(ascii::SPACE)
    }

    fn strip_trailing_null(&mut self) -> &Vec<u8> {
        self.strip_trailing_value(ascii::NULL)

    }

    fn pad_with_value(&mut self, length: usize, value: u8) -> &Self {
        while self.len() < length {
            &self.push(value);
        }
        self
    }

    fn pad_with_spaces(&mut self, length: usize) -> &Self {
        self.pad_with_value(length, ascii::SPACE)
    }

    fn pad_with_null(&mut self, length: usize) -> &Self {
        self.pad_with_value(length, ascii::NULL)
    }

    fn write_to_slice(&self, bytes: &mut [u8]) -> Result<(), Box<dyn Error>> {
        if self.len() > bytes.len() {
            return Err(Box::new(BytesError::TooLarge));
        }
        for (i, c) in self.iter().enumerate() {
            bytes[i] = *c;
        }
        Ok(())
    }
}

impl PackToBytes for usize {
    fn pack_to_bytes(&self, bytes: &mut[u8]) {
        let mut value = *self;
        for i in 0..bytes.len() {
            bytes[i] = (value & 255) as u8;
            value >>= 8;
        }
    }
}

impl AsUSize for [u8] {
    fn as_usize(&self) -> usize {
        let mut value: usize = 0;
        for &byte in self.iter().rev() {
            value <<= 8;
            value += byte as usize;
        }
        value
    }
}
