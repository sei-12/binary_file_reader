use error::BinaryFileReaderError;

pub mod error;

#[derive(Debug, Clone)]
pub struct BinaryFileReader<'a> {
    current_offset: usize,
    own_left: usize,
    buf: &'a [u8],
}

impl BinaryFileReader<'_> {
    fn peek(&self, buffer: &mut [u8]) -> Result<(), BinaryFileReaderError> {
        if buffer.len() > self.available_bytes() {
            return Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: buffer.len(),
                current_offset: self.current_offset,
                available_bytes: self.available_bytes(),
            });
        }

        buffer.copy_from_slice(&self.buf[self.current_offset..self.current_offset + buffer.len()]);

        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<(), BinaryFileReaderError> {
        self.peek(buffer)?;
        self.current_offset += buffer.len();
        Ok(())
    }
}

impl<'a> BinaryFileReader<'a> {
    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # use std::fs;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = fs::read("./sample-files/1.png")?;
    /// let mut reader = BinaryFileReader::new(&buffer);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn new(buffer: &'a [u8]) -> Self {
        let current_offset = 0;
        let own_left = buffer.len();
        Self {
            own_left,
            current_offset,
            buf: buffer,
        }
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # use std::fs;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0,1,2,3,4,5];
    /// let mut reader = BinaryFileReader::new(&buffer);
    /// assert_eq!(reader.current_offset(),0);
    /// reader.read_u8()?;
    /// assert_eq!(reader.current_offset(),1);
    /// reader.read_u16()?;
    /// assert_eq!(reader.current_offset(),3);
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
    /// # use std::fs;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0,1,2,3,4,5];
    /// let mut reader = BinaryFileReader::new(&buffer);
    /// assert_eq!(reader.available_bytes(),6);
    /// reader.read_u8()?;
    /// assert_eq!(reader.available_bytes(),5);
    /// reader.read_u16()?;
    /// assert_eq!(reader.available_bytes(),3);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    #[inline]
    pub fn available_bytes(&self) -> usize {
        self.own_left - self.current_offset
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # use std::fs;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0xab,0xcd,0xef];
    /// let mut reader = BinaryFileReader::new(&buffer);
    /// let (upper,lower) = reader.read_u4()?;
    /// assert_eq!(upper,0x0a);
    /// assert_eq!(lower,0x0b);
    /// assert_eq!(reader.read_u4()?,(0x0c,0x0d));
    /// assert_eq!(reader.read_u4()?,(0x0e,0x0f));
    /// assert!(reader.read_u4().is_err());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn read_u4(&mut self) -> Result<(u8, u8), BinaryFileReaderError> {
        let mut buffer = [0; 1];
        self.read(&mut buffer)?;
        let upper = buffer[0] >> 4;
        let lower = buffer[0] & 0x0f;
        Ok((upper, lower))
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![1,2,3];
    /// let mut reader = BinaryFileReader::new(&buffer);
    /// assert_eq!(reader.read_u8()?,1);
    /// assert_eq!(reader.read_u8()?,2);
    /// assert_eq!(reader.read_u8()?,3);
    /// assert!(reader.read_u8().is_err());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn read_u8(&mut self) -> Result<u8, BinaryFileReaderError> {
        let mut buffer = [0; 1];
        self.read(&mut buffer)?;
        Ok(buffer[0])
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0x12,0x34,0x56,0x78,0x90];
    /// let mut reader = BinaryFileReader::new(&buffer);
    /// assert_eq!(reader.read_u16()?,0x1234);
    /// assert_eq!(reader.read_u16()?,0x5678);
    /// assert!(reader.read_u16().is_err());
    /// assert_eq!(reader.read_u8()?,0x90);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn read_u16(&mut self) -> Result<u16, BinaryFileReaderError> {
        let mut buffer = [0; 2];
        self.read(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0x12,0x34,0x56,0x78,0x90];
    /// let mut reader = BinaryFileReader::new(&buffer);
    /// assert_eq!(reader.read_u32()?,0x12345678);
    /// assert!(reader.read_u32().is_err());
    /// assert_eq!(reader.read_u8()?,0x90);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn read_u32(&mut self) -> Result<u32, BinaryFileReaderError> {
        let mut buffer = [0; 4];
        self.read(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0x12,0x34,0x56,0x78,0x90,0x12,0x34,0x56,0x78,0x90];
    /// let mut reader = BinaryFileReader::new(&buffer);
    /// assert_eq!(reader.read_u64()?,0x1234567890123456);
    /// assert!(reader.read_u64().is_err());
    /// assert_eq!(reader.read_u8()?,0x78);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn read_u64(&mut self) -> Result<u64, BinaryFileReaderError> {
        let mut buffer = [0; 8];
        self.read(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    pub fn read_u128(&mut self) -> Result<u128, BinaryFileReaderError> {
        let mut buffer = [0; 16];
        self.read(&mut buffer)?;
        Ok(u128::from_be_bytes(buffer))
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0,1,2,3,4,5,6,7,8,9];
    /// let mut reader = BinaryFileReader::new(&buffer);
    /// let mut buf = vec![0;5];
    /// reader.read_bytes(&mut buf);
    /// assert_eq!(buf,vec![0,1,2,3,4]);
    /// reader.read_bytes(&mut buf);
    /// assert_eq!(buf,vec![5,6,7,8,9]);
    /// assert!(reader.read_u8().is_err());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<(), BinaryFileReaderError> {
        self.read(buffer)?;
        Ok(())
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let text = "Hello, world!";
    /// let binary_data: Vec<u8> = text.as_bytes().to_vec();
    /// let mut reader = BinaryFileReader::new(&binary_data);
    /// assert_eq!(reader.read_utf8(13)?,"Hello, world!");
    /// assert!(reader.read_utf8(10).is_err());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn read_utf8(&mut self, bytes: usize) -> Result<&'a str, BinaryFileReaderError> {
        if bytes > self.available_bytes() {
            return Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: bytes,
                current_offset: self.current_offset,
                available_bytes: self.available_bytes(),
            });
        }

        let slice = &self.buf[self.current_offset..self.current_offset + bytes];
        let result = std::str::from_utf8(slice)?;
        self.current_offset += bytes;
        Ok(result)
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0xab,0xcd];
    /// let reader = BinaryFileReader::new(&buffer);
    /// assert_eq!(reader.peek_u4()?,(0x0a,0x0b));
    /// assert_eq!(reader.peek_u4()?,(0x0a,0x0b));
    /// assert_eq!(reader.peek_u4()?,(0x0a,0x0b));
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn peek_u4(&self) -> Result<(u8, u8), BinaryFileReaderError> {
        let mut buffer = [0; 1];
        self.peek(&mut buffer)?;
        let upper = buffer[0] >> 4;
        let lower = buffer[0] & 0x0f;
        Ok((upper, lower))
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0x01,0x02,0x03];
    /// let reader = BinaryFileReader::new(&buffer);
    /// assert_eq!(reader.peek_u8()?,0x01);
    /// assert_eq!(reader.peek_u8()?,0x01);
    /// assert_eq!(reader.peek_u8()?,0x01);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn peek_u8(&self) -> Result<u8, BinaryFileReaderError> {
        let mut buffer = [0; 1];
        self.peek(&mut buffer)?;
        Ok(u8::from_be_bytes(buffer))
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0x01,0x02,0x03];
    /// let reader = BinaryFileReader::new(&buffer);
    /// assert_eq!(reader.peek_u16()?,0x0102);
    /// assert_eq!(reader.peek_u16()?,0x0102);
    /// assert_eq!(reader.peek_u16()?,0x0102);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn peek_u16(&self) -> Result<u16, BinaryFileReaderError> {
        let mut buffer = [0; 2];
        self.peek(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0x01,0x02,0x03,0x04,0x05];
    /// let reader = BinaryFileReader::new(&buffer);
    /// assert_eq!(reader.peek_u32()?,0x01020304);
    /// assert_eq!(reader.peek_u32()?,0x01020304);
    /// assert_eq!(reader.peek_u32()?,0x01020304);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn peek_u32(&self) -> Result<u32, BinaryFileReaderError> {
        let mut buffer = [0; 4];
        self.peek(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![1,2,3,4,5,6,7,8,9];
    /// let reader = BinaryFileReader::new(&buffer);
    /// assert_eq!(reader.peek_u64()?,0x0102030405060708);
    /// assert_eq!(reader.peek_u64()?,0x0102030405060708);
    /// assert_eq!(reader.peek_u64()?,0x0102030405060708);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn peek_u64(&self) -> Result<u64, BinaryFileReaderError> {
        let mut buffer = [0; 8];
        self.peek(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    pub fn peek_u128(&self) -> Result<u128, BinaryFileReaderError> {
        let mut buffer = [0; 16];
        self.peek(&mut buffer)?;
        Ok(u128::from_be_bytes(buffer))
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0,1,2,3,4,5,6,7,8,9];
    /// let reader = BinaryFileReader::new(&buffer);
    ///
    /// let mut buf = vec![0;5];
    /// reader.peek_bytes(&mut buf);
    /// assert_eq!(buf,vec![0,1,2,3,4]);
    ///
    /// let mut buf = vec![0;5];
    /// reader.peek_bytes(&mut buf);
    /// assert_eq!(buf,vec![0,1,2,3,4]);
    ///
    /// let mut buf = vec![0; 11];
    /// assert!(reader.peek_bytes(&mut buf).is_err());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn peek_bytes(&self, buffer: &mut [u8]) -> Result<(), BinaryFileReaderError> {
        self.peek(buffer)?;
        Ok(())
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let text = "Hello, world!";
    /// let binary_data: Vec<u8> = text.as_bytes().to_vec();
    /// let mut reader = BinaryFileReader::new(&binary_data);
    /// assert_eq!(reader.peek_utf8(13)?,"Hello, world!");
    /// assert_eq!(reader.peek_utf8(13)?,"Hello, world!");
    /// assert!(reader.peek_utf8(14).is_err());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn peek_utf8(&self, bytes: usize) -> Result<&'a str, BinaryFileReaderError> {
        if bytes > self.available_bytes() {
            return Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: bytes,
                current_offset: self.current_offset,
                available_bytes: self.available_bytes(),
            });
        }

        let slice = &self.buf[self.current_offset..self.current_offset + bytes];
        let result = std::str::from_utf8(slice)?;
        Ok(result)
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// let mut reader = BinaryFileReader::new(&buffer);
    /// reader.expect(&[0, 1, 2, 3])?;
    /// reader.expect(&[4, 5, 6, 7])?;
    /// assert!(reader.expect(&[0, 0, 0]).is_err());
    /// assert!(reader.expect(&[8, 9, 0, 0, 0]).is_err());
    /// reader.expect(&[8, 9])?;
    ///
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn expect(&mut self, expect_bytes: &[u8]) -> Result<(), BinaryFileReaderError> {
        self.expect_peek(expect_bytes)?;
        self.current_offset += expect_bytes.len();
        Ok(())
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    /// let text = "Hello, world!";
    /// let binary_data: Vec<u8> = text.as_bytes().to_vec();
    /// let mut reader = BinaryFileReader::new(&binary_data);
    ///
    /// reader.expect_utf8("Hello")?;
    /// assert!(reader.expect_utf8("Hello").is_err());
    /// reader.expect_utf8(", world!")?;
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn expect_utf8(&mut self, expect_str: &str) -> Result<(), BinaryFileReaderError> {
        self.expect(expect_str.as_bytes())?;
        Ok(())
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let buffer = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let reader = BinaryFileReader::new(&buffer);
    /// reader.expect_peek(&[0, 1, 2, 3])?;
    /// reader.expect_peek(&[0, 1, 2, 3])?;
    ///
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn expect_peek(&self, expect_bytes: &[u8]) -> Result<(), BinaryFileReaderError> {
        if self.available_bytes() < expect_bytes.len() {
            let require = Vec::from(expect_bytes);
            return Err(BinaryFileReaderError::ExpectInsufficientBytes {
                require,
                available_bytes: self.available_bytes(),
                current_offset: self.current_offset(),
            });
        }

        let slice = &self.buf[self.current_offset..self.current_offset + expect_bytes.len()];

        for (req, got) in expect_bytes.iter().zip(slice) {
            if *req == *got {
                continue;
            }

            let require = Vec::from(expect_bytes);
            let got = Vec::from(slice);
            return Err(BinaryFileReaderError::Expect {
                require,
                got,
                current_offset: self.current_offset(),
                available_bytes: self.available_bytes(),
            });
        }

        Ok(())
    }

    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let buffer = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let mut reader = BinaryFileReader::new(&buffer);
    /// let mut splited = reader.split_off_front(5)?;
    ///
    /// splited.expect_peek(&[0,1,2,3,4])?;
    /// assert_eq!(splited.current_offset(),0);
    ///
    /// reader.expect_peek(&[5,6,7,8,9])?;
    /// assert_eq!(reader.current_offset(),5);
    ///
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn split_off_front(&mut self, size: usize) -> Result<Self, BinaryFileReaderError> {
        if size > self.available_bytes() {
            return Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: size,
                current_offset: self.current_offset,
                available_bytes: self.available_bytes(),
            });
        }

        let splited_offset = self.current_offset;
        let new_offset = self.current_offset + size;

        self.current_offset = new_offset;

        Ok(Self {
            current_offset: splited_offset,
            own_left: new_offset,
            buf: self.buf,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::BinaryFileReaderError;

    use super::BinaryFileReader;

    #[test]
    fn test_read() -> Result<(), BinaryFileReaderError> {
        let mut reader = BinaryFileReader::new(&[0, 1, 2, 3, 4, 5]);
        assert_eq!(reader.read_u8()?, 0);
        assert_eq!(reader.read_u8()?, 1);
        assert_eq!(reader.read_u8()?, 2);
        assert_eq!(reader.read_u16()?, 0x0304);
        assert_eq!(reader.read_u8()?, 5);

        let mut reader = BinaryFileReader::new(&[15, 1, 2, 3, 4, 5]);
        assert_eq!(reader.read_u4()?, (0x00, 0x0f));
        assert_eq!(reader.read_u32()?, 0x01020304);

        let mut reader = BinaryFileReader::new(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3,
        ]);
        assert_eq!(reader.read_u64()?, 0x0001020304050607);
        assert_eq!(reader.read_u128()?, 0x08090001020304050607080900010203);

        Ok(())
    }

    #[test]
    fn test_peek() -> Result<(), BinaryFileReaderError> {
        let reader = BinaryFileReader::new(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3,
        ]);
        assert_eq!(reader.peek_u8()?, 0);
        assert_eq!(reader.peek_u8()?, 0);
        assert_eq!(reader.peek_u4()?, (0, 0));
        assert_eq!(reader.peek_u4()?, (0, 0));
        assert_eq!(reader.peek_u16()?, 1);
        assert_eq!(reader.peek_u16()?, 1);
        assert_eq!(reader.peek_u32()?, 0x00010203);
        assert_eq!(reader.peek_u32()?, 0x00010203);
        assert_eq!(reader.peek_u64()?, 0x0001020304050607);
        assert_eq!(reader.peek_u64()?, 0x0001020304050607);
        assert_eq!(reader.peek_u128()?, 0x00010203040506070809000102030405);
        assert_eq!(reader.peek_u128()?, 0x00010203040506070809000102030405);

        let reader = BinaryFileReader::new(&[]);
        assert!(reader.peek_u4().is_err());
        assert!(reader.peek_u8().is_err());
        assert!(reader.peek_u16().is_err());
        assert!(reader.peek_u32().is_err());
        assert!(reader.peek_u64().is_err());
        assert!(reader.peek_u128().is_err());

        Ok(())
    }

    #[test]
    fn test_split_off_front() -> Result<(), BinaryFileReaderError> {
        let buffer = (0..=255).collect::<Vec<u8>>();
        let mut a = BinaryFileReader::new(&buffer);
        let mut b = a.split_off_front(128)?;
        assert_eq!(b.current_offset(), 0);
        assert_eq!(b.read_u8()?, 0);
        assert_eq!(a.current_offset(), 128);
        assert_eq!(a.read_u8()?, 128);

        let buffer = (0..=255).collect::<Vec<u8>>();
        let mut a = BinaryFileReader::new(&buffer);
        let mut b = a.split_off_front(128)?;
        assert_eq!(b.current_offset(), 0);
        assert_eq!(a.current_offset(), 128);
        assert!(b.split_off_front(129).is_err());
        assert!(a.split_off_front(129).is_err());
        let mut c = a.split_off_front(64)?;
        let mut d = b.split_off_front(64)?;
        assert_eq!(a.current_offset(), 128 + 64);
        assert_eq!(b.current_offset(), 64);
        assert_eq!(c.current_offset(), 128);
        assert_eq!(d.current_offset(), 0);
        assert_eq!(a.available_bytes(), 64);
        assert_eq!(b.available_bytes(), 64);
        assert_eq!(c.available_bytes(), 64);
        assert_eq!(d.available_bytes(), 64);
        let mut c = c.split_off_front(64)?;
        assert_eq!(c.available_bytes(), 64);

        assert_eq!(a.read_u8()?, 128 + 64);
        assert_eq!(b.read_u8()?, 64);
        assert_eq!(c.read_u8()?, 128);
        assert_eq!(d.read_u8()?, 0);

        Ok(())
    }

    #[test]
    fn test_expect() -> Result<(), BinaryFileReaderError> {
        let buffer = (0..=255).collect::<Vec<u8>>();
        let mut reader = BinaryFileReader::new(&buffer);
        reader.expect(&[0, 1, 2, 3, 4])?;

        let buffer = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut reader = BinaryFileReader::new(&buffer);
        reader.expect(&[0, 1, 2, 3])?;
        assert_eq!(reader.current_offset(), 4);
        assert_eq!(reader.available_bytes(), 6);
        reader.expect(&[4, 5, 6, 7])?;
        assert_eq!(reader.current_offset(), 8);
        assert_eq!(reader.available_bytes(), 2);

        let buffer = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let reader = BinaryFileReader::new(&buffer);
        reader.expect_peek(&[0, 1, 2, 3])?;
        reader.expect_peek(&[0, 1, 2, 3])?;

        Ok(())
    }

    #[test]
    fn test_utf8() -> Result<(), BinaryFileReaderError> {
        let text = "Hello, world!";
        let binary_data: Vec<u8> = text.as_bytes().to_vec();
        let mut reader = BinaryFileReader::new(&binary_data);
        assert_eq!(reader.peek_utf8(13)?, "Hello, world!");
        assert_eq!(reader.read_utf8(13)?, "Hello, world!");
        assert!(reader.read_utf8(10).is_err());

        let text = "こんにちは";
        let binary_data: Vec<u8> = text.as_bytes().to_vec();
        let mut reader = BinaryFileReader::new(&binary_data);
        assert_eq!(reader.peek_utf8(15)?, "こんにちは");
        assert!(reader.read_utf8(10).is_err());

        Ok(())
    }

    #[test]
    fn test1() -> Result<(), BinaryFileReaderError> {
        let buffer = (0..=255).collect::<Vec<u8>>();
        let mut reader = BinaryFileReader::new(&buffer);

        reader.read_u8()?;
        assert_eq!(reader.available_bytes(), 255);
        reader.read_u8()?;
        assert_eq!(reader.available_bytes(), 254);
        reader.read_u4()?;
        assert_eq!(reader.available_bytes(), 253);
        reader.read_u16()?;
        assert_eq!(reader.available_bytes(), 251);
        reader.read_u16()?;
        assert_eq!(reader.available_bytes(), 249);

        let mut b = vec![0; 200];
        reader.read_bytes(&mut b)?;
        assert_eq!(reader.available_bytes(), 49);

        let mut splited = reader.split_off_front(20)?;

        assert_eq!(reader.available_bytes(), 29);
        assert_eq!(splited.available_bytes(), 20);

        let a = splited.split_off_front(20)?;
        assert_eq!(splited.available_bytes(), 0);
        assert!(splited.read_u8().is_err());
        assert!(splited.read_u4().is_err());
        assert!(splited.read_u128().is_err());
        assert!(splited.read_u32().is_err());
        assert!(splited.read_u64().is_err());
        assert!(splited.read_bytes(&mut b).is_err());
        assert!(splited.read_u8().is_err());

        assert_eq!(a.available_bytes(), 20);

        Ok(())
    }
}
