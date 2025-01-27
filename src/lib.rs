use std::{collections::VecDeque, fs, path::Path};

use error::BinaryFileReaderError;

pub mod error;

pub struct BinaryFileReader {
    current_offset: usize,
    buffer: VecDeque<u8>,
}

impl BinaryFileReader {
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, BinaryFileReaderError> {
        let buffer = fs::read(path)?.into_iter().collect();
        let current_offset = 0;
        Ok(Self {
            buffer,
            current_offset,
        })
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// let mut reader = BinaryFileReader::from(vec![0x01,0x02,0x03,0x04]);
    /// assert_eq!(reader.current_offset(),0);
    /// assert_eq!(reader.shift()?,0x01);
    /// assert_eq!(reader.current_offset(),1);
    ///
    /// let mut reader = BinaryFileReader::from(vec![0x01,0x02,0x03,0x04]);
    /// let mut splited = reader.split(2)?;
    /// assert_eq!(splited.current_offset(),0);
    /// assert_eq!(reader.current_offset(),2);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    #[inline]
    pub fn current_offset(&self) -> usize {
        self.current_offset
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// let mut reader = BinaryFileReader::from(vec![0x01,0x02,0x03,0x04]);
    /// assert_eq!(reader.available_bytes(),4);
    /// assert_eq!(reader.shift()?,0x01);
    /// assert_eq!(reader.available_bytes(),3);
    ///
    /// let mut reader = BinaryFileReader::from(vec![0x01,0x02,0x03,0x04]);
    /// let mut splited = reader.split(2)?;
    /// assert_eq!(splited.available_bytes(),2);
    /// assert_eq!(reader.available_bytes(),2);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    #[inline]
    pub fn available_bytes(&self) -> usize {
        self.buffer.len()
    }

    /// # Examples
    /// ```
    /// # // hidden lines start with `#` symbol, but they're still compilable!
    /// # use binary_file_reader::BinaryFileReader;
    /// # use binary_file_reader::error::BinaryFileReaderError;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// let mut reader = BinaryFileReader::from(vec![0x01,0x02,0x03,0x04]);
    /// let mut splited = reader.split(2)?;
    ///
    /// assert_eq!(splited.current_offset(),0);
    /// assert_eq!(splited.shift()?,0x01);
    ///
    /// assert_eq!(reader.current_offset(),2);
    /// assert_eq!(reader.shift()?,0x03);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn split(&mut self, n: usize) -> Result<Self, BinaryFileReaderError> {
        if n > self.buffer.len() {
            return Err(BinaryFileReaderError::BufferUnderflow {
                current_offset: self.current_offset,
                available_bytes: self.buffer.len(),
                requested_bytes: n,
            });
        }

        let taked_block_offset = self.current_offset;
        let splited = self.buffer.split_off(n);
        self.current_offset += n;
        let splited = std::mem::replace(&mut self.buffer, splited);

        Ok(Self {
            buffer: splited,
            current_offset: taked_block_offset,
        })
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// let mut reader = BinaryFileReader::from(vec![0x01,0x02]);
    /// assert_eq!(reader.shift()?,0x01);
    /// assert_eq!(reader.shift()?,0x02);
    /// assert!(reader.shift().is_err());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn shift(&mut self) -> Result<u8, BinaryFileReaderError> {
        match self.buffer.pop_front() {
            Some(shifted) => {
                self.current_offset += 1;
                Ok(shifted)
            }
            None => Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: 1,
                current_offset: self.current_offset,
                available_bytes: 0,
            }),
        }
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// let mut reader = BinaryFileReader::from(vec![0x01,0x02,0x03,0x04,0x05]);
    /// assert_eq!(reader.shift_u16()?,0x0102);
    /// assert_eq!(reader.shift_u16()?,0x0304);
    /// assert!(reader.shift_u16().is_err());
    /// assert_eq!(reader.shift()?,0x05);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn shift_u16(&mut self) -> Result<u16, BinaryFileReaderError> {
        if self.buffer.len() < 2 {
            return Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: 2,
                current_offset: self.current_offset,
                available_bytes: self.buffer.len(),
            });
        };

        let upper = self.buffer.pop_front().unwrap() as u16;
        let lower = self.buffer.pop_front().unwrap() as u16;
        self.current_offset += 2;

        Ok((upper << 8) | (lower))
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// let mut reader = BinaryFileReader::from(vec![0x12,0x34]);
    /// let (upper,lower) = reader.shift_u4()?;
    /// assert_eq!(upper,0x01);
    /// assert_eq!(lower,0x02);
    /// assert_eq!(reader.shift_u4()?,(0x03,0x04));
    /// assert!(reader.shift_u4().is_err());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn shift_u4(&mut self) -> Result<(u8, u8), BinaryFileReaderError> {
        match self.buffer.pop_front() {
            Some(shifted) => {
                self.current_offset += 1;
                let upper = shifted >> 4;
                let lower = shifted & 0x0f;
                Ok((upper, lower))
            }
            None => Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: 1,
                current_offset: self.current_offset,
                available_bytes: 0,
            }),
        }
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// let mut reader = BinaryFileReader::from(vec![0x01,0x02,0x03,0x04,0x05]);
    /// assert!(reader.expect(&[0x01]).is_ok());
    /// assert!(reader.expect(&[0x02,0x03]).is_ok());
    ///
    /// assert!(reader.expect(&[0xff,0xff]).is_err());
    /// assert!(reader.expect(&[0x04,0x05,0xff]).is_err());
    ///
    /// assert!(reader.expect(&[0x04,0x05]).is_ok());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn expect(&mut self, expect_bytes: &[u8]) -> Result<(), BinaryFileReaderError> {
        self.expect_peek(expect_bytes)?;
        self.buffer.drain(0..expect_bytes.len());
        self.current_offset += expect_bytes.len();

        Ok(())
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// let reader = BinaryFileReader::from(vec![0x01,0x02,0x03,0x04,0x05]);
    /// assert!(reader.expect_peek(&[0x01]).is_ok());
    /// assert!(reader.expect_peek(&[0x02,0x03]).is_err());
    /// assert!(reader.expect_peek(&[0x01,0x02,0x03]).is_ok());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn expect_peek(&self, expect_bytes: &[u8]) -> Result<(), BinaryFileReaderError> {
        if expect_bytes.len() > self.buffer.len() {
            return Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: expect_bytes.len(),
                current_offset: self.current_offset,
                available_bytes: self.buffer.len(),
            });
        }

        for (i, expect) in expect_bytes.iter().enumerate() {
            if self.buffer[i] == *expect {
                continue;
            }
            return Err(BinaryFileReaderError::Expect {});
        }

        Ok(())
    }

    /// # Examples
    /// ```
    /// # use std::collections::VecDeque;
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// #
    /// let reader = BinaryFileReader::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
    /// let taked = reader.take();
    /// assert_eq!(taked, VecDeque::from([0x01, 0x02, 0x03, 0x04, 0x05]));
    /// // reader; // borrow of moved value: `reader`
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn take(self) -> VecDeque<u8> {
        self.buffer
    }

    /// # Examples
    /// ```
    /// # use std::collections::VecDeque;
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// #
    /// let mut reader = BinaryFileReader::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
    /// let taked = reader.take_bytes(2)?;
    ///
    /// assert_eq!(taked, VecDeque::from([0x01, 0x02]));
    /// assert!(reader.expect(&[0x03,0x04,0x05]).is_ok());
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn take_bytes(&mut self, n: usize) -> Result<VecDeque<u8>, BinaryFileReaderError> {
        if n > self.buffer.len() {
            return Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: n,
                current_offset: self.current_offset,
                available_bytes: self.buffer.len(),
            });
        };
        let splited = self.buffer.split_off(n);
        let taked = std::mem::replace(&mut self.buffer, splited);
        self.current_offset += n;
        Ok(taked)
    }
}

impl From<Vec<u8>> for BinaryFileReader {
    fn from(value: Vec<u8>) -> Self {
        let buffer = value.into_iter().collect();
        let current_offset = 0;
        Self {
            buffer,
            current_offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{error::BinaryFileReaderError, BinaryFileReader};

    #[test]
    fn test_take_bytes() -> Result<(), BinaryFileReaderError> {
        let mut reader = BinaryFileReader::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        let taked = reader.take_bytes(2)?;

        assert_eq!(reader.available_bytes(), 3);
        assert_eq!(reader.current_offset(), 2);

        assert_eq!(taked, VecDeque::from([0x01, 0x02]));
        assert!(reader.expect(&[0x03, 0x04, 0x05]).is_ok());

        Ok(())
    }

    #[test]
    fn test_take() -> Result<(), BinaryFileReaderError> {
        let reader = BinaryFileReader::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        let taked = reader.take();
        assert_eq!(taked, VecDeque::from([0x01, 0x02, 0x03, 0x04, 0x05]));

        // borrow of moved value: `reader`
        // reader;

        Ok(())
    }
}
