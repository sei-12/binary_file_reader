use std::{collections::VecDeque, fs, path::Path};

use error::BinaryFileReaderError;

pub mod error;

#[derive(Debug)]
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
    /// assert_eq!(reader.read()?,0x01);
    /// assert_eq!(reader.current_offset(),1);
    ///
    /// let mut reader = BinaryFileReader::from(vec![0x01,0x02,0x03,0x04]);
    /// let mut splited = reader.split_off_front(2)?;
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
    /// assert_eq!(reader.read()?,0x01);
    /// assert_eq!(reader.available_bytes(),3);
    ///
    /// let mut reader = BinaryFileReader::from(vec![0x01,0x02,0x03,0x04]);
    /// let mut splited = reader.split_off_front(2)?;
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
    /// let mut splited = reader.split_off_front(2)?;
    ///
    /// assert_eq!(splited.current_offset(),0);
    /// assert_eq!(splited.read()?,0x01);
    ///
    /// assert_eq!(reader.current_offset(),2);
    /// assert_eq!(reader.read()?,0x03);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn split_off_front(&mut self, n: usize) -> Result<Self, BinaryFileReaderError> {
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

    //----------------------------------------------------------------------------------------------------//
    //                                                                                                    //
    //                                                READ                                                //
    //                                                                                                    //
    //----------------------------------------------------------------------------------------------------//
    /// # Examples
    /// ```
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// let mut reader = BinaryFileReader::from(vec![0x01,0x02]);
    /// assert_eq!(reader.read()?,0x01);
    /// assert_eq!(reader.read()?,0x02);
    /// assert!(reader.read().is_err());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn read(&mut self) -> Result<u8, BinaryFileReaderError> {
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
    /// assert_eq!(reader.read_u16()?,0x0102);
    /// assert_eq!(reader.read_u16()?,0x0304);
    /// assert!(reader.read_u16().is_err());
    /// assert_eq!(reader.read()?,0x05);
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn read_u16(&mut self) -> Result<u16, BinaryFileReaderError> {
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
    /// let (upper,lower) = reader.read_u4()?;
    /// assert_eq!(upper,0x01);
    /// assert_eq!(lower,0x02);
    /// assert_eq!(reader.read_u4()?,(0x03,0x04));
    /// assert!(reader.read_u4().is_err());
    /// #
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn read_u4(&mut self) -> Result<(u8, u8), BinaryFileReaderError> {
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
    /// # use std::collections::VecDeque;
    /// # use binary_file_reader::BinaryFileReader;
    /// # fn try_main() -> Result<(), Box<dyn std::error::Error>> { // line that wraps the body shown in doc
    /// #
    /// let mut reader = BinaryFileReader::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
    /// let taked = reader.read_bytes(2)?;
    ///
    /// assert_eq!(taked, VecDeque::from([0x01, 0x02]));
    /// assert!(reader.expect(&[0x03,0x04,0x05]).is_ok());
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn read_bytes(&mut self, n: usize) -> Result<VecDeque<u8>, BinaryFileReaderError> {
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
    
    
    

    //----------------------------------------------------------------------------------------------------//
    //                                                                                                    //
    //                                               EXPECT                                               //
    //                                                                                                    //
    //----------------------------------------------------------------------------------------------------//   

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
    #[inline]
    pub fn take(self) -> VecDeque<u8> {
        self.buffer
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



//----------------------------------------------------------------------------------------------------//
//                                                                                                    //
//                                               TESTS                                                //
//                                                                                                    //
//----------------------------------------------------------------------------------------------------//
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{error::BinaryFileReaderError, BinaryFileReader};

    #[test]
    fn test_read_bytes() -> Result<(), BinaryFileReaderError> {
        let mut reader = BinaryFileReader::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        let taked = reader.read_bytes(2)?;

        assert_eq!(reader.available_bytes(), 3);
        assert_eq!(reader.current_offset(), 2);

        assert_eq!(taked, VecDeque::from([0x01, 0x02]));
        assert!(reader.expect(&[0x03, 0x04, 0x05]).is_ok());
        

        let v: Vec<u8> = (0..=255).collect();
        let mut reader = BinaryFileReader::from(v);
        let taked = reader.read_bytes(128)?;
        assert_eq!(taked, (0..128).collect::<Vec<u8>>());
        reader.expect_peek(&(128..=255).collect::<Vec<u8>>())?;
        assert_eq!(reader.available_bytes(),128);
        assert_eq!(reader.current_offset(),128);


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

    #[test]
    fn test_current_offset() -> Result<(), BinaryFileReaderError> {
        let mut reader = BinaryFileReader::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(reader.current_offset(), 0);
        reader.read()?;
        assert_eq!(reader.current_offset(), 1);
        reader.read()?;
        assert_eq!(reader.current_offset(), 2);
        reader.read_u16()?;
        assert_eq!(reader.current_offset(), 4);

        let v: Vec<u8> = (0..=255).collect();
        let mut reader = BinaryFileReader::from(v);
        let mut a = reader.split_off_front(128)?;
        assert_eq!(reader.current_offset(), 128);
        assert_eq!(a.current_offset(), 0);
        a.read_bytes(64)?;
        assert_eq!(a.current_offset(), 64);
        let b = reader.read_bytes(64)?;
        assert_eq!(b.len(), 64);
        assert_eq!(reader.current_offset(), 128 + 64);

        Ok(())
    }

    #[test]
    fn test_available_bytes() -> Result<(), BinaryFileReaderError> {
        let mut reader = BinaryFileReader::from(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(reader.available_bytes(), 5);
        reader.read()?;
        assert_eq!(reader.available_bytes(), 4);
        reader.read()?;
        assert_eq!(reader.available_bytes(), 3);
        reader.read_u16()?;
        assert_eq!(reader.available_bytes(), 1);
        reader.read()?;
        assert_eq!(reader.available_bytes(), 0);

        let v: Vec<u8> = (0..=255).collect();
        let mut reader1 = BinaryFileReader::from(v);
        let mut reader2 = reader1.split_off_front(128)?;
        assert_eq!(reader1.available_bytes(), 128);
        assert_eq!(reader2.available_bytes(), 128);
        let mut reader3 = reader2.split_off_front(64)?;
        assert_eq!(reader1.available_bytes(), 128);
        assert_eq!(reader3.available_bytes(), 64);
        assert_eq!(reader2.available_bytes(), 64);

        assert_eq!(reader3.read()?, 0);
        assert_eq!(reader3.available_bytes(), 63);

        reader3.read_u4()?;
        assert_eq!(reader3.available_bytes(), 62);

        reader3.read_u16()?;
        assert_eq!(reader3.available_bytes(), 60);

        reader3.expect(&[4, 5, 6, 7, 8])?;
        assert_eq!(reader3.available_bytes(), 55);

        reader3.expect_peek(&[9, 10, 11, 12, 13])?;
        assert_eq!(reader3.available_bytes(), 55);

        reader3.read_bytes(5)?;
        assert_eq!(reader3.available_bytes(), 50);

        Ok(())
    }

    #[test]
    fn test_split_off_front() -> Result<(), BinaryFileReaderError> {
        let v: Vec<u8> = (0..=255).collect();
        let mut reader1 = BinaryFileReader::from(v);

        reader1.expect_peek(&(0..=255).collect::<Vec<u8>>())?;

        let mut reader2 = reader1.split_off_front(128)?;
        reader2.expect_peek(&(0..128).collect::<Vec<u8>>())?;
        reader1.expect_peek(&(128..=255).collect::<Vec<u8>>())?;

        assert_eq!(reader1.current_offset(), 128);
        assert_eq!(reader2.current_offset(), 0);

        let err = reader2.split_off_front(129).unwrap_err();
        assert!(matches!(
            err,
            BinaryFileReaderError::BufferUnderflow {
                requested_bytes: 129,
                current_offset: 0,
                available_bytes: 128
            }
        ));

        let err = reader2.split_off_front(132).unwrap_err();
        assert!(matches!(
            err,
            BinaryFileReaderError::BufferUnderflow {
                requested_bytes: 132,
                current_offset: 0,
                available_bytes: 128
            }
        ));

        let err = reader1.split_off_front(132).unwrap_err();
        assert!(matches!(
            err,
            BinaryFileReaderError::BufferUnderflow {
                requested_bytes: 132,
                current_offset: 128,
                available_bytes: 128
            }
        ));

        let mut reader3 = reader2.split_off_front(128)?;
        assert_eq!(reader2.available_bytes(), 0);
        reader3.expect_peek(&(0..128).collect::<Vec<u8>>())?;

        let mut reader4 = reader3.split_off_front(64)?;
        reader3.expect_peek(&(64..128).collect::<Vec<u8>>())?;
        reader4.expect_peek(&(0..64).collect::<Vec<u8>>())?;

        assert_eq!(reader1.read()?, 128);
        assert!(reader2.read().is_err());
        assert_eq!(reader3.read()?, 64);
        assert_eq!(reader4.read()?, 0);

        Ok(())
    }

    #[test]
    fn test_read() -> Result<(), BinaryFileReaderError> {
        let v: Vec<u8> = (0..=255).collect();
        let mut reader1 = BinaryFileReader::from(v);
        assert_eq!(reader1.read()?, 0);
        assert_eq!(reader1.read_u16()?, 0x0102);

        let mut reader = BinaryFileReader::from(vec![0x01, 0x02, 0x03]);
        assert_eq!(reader.read()?, 0x01);
        assert_eq!(reader.read()?, 0x02);
        assert_eq!(reader.read()?, 0x03);
        assert!(matches!(
            reader.read(),
            Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: 1,
                current_offset: 3,
                available_bytes: 0,
            }),
        ));

        let mut reader = BinaryFileReader::from(vec![0xab, 0xcd, 0xef]);
        assert_eq!(reader.read_u4()?, (0x0a, 0x0b));
        assert_eq!(reader.read_u4()?, (0x0c, 0x0d));
        assert_eq!(reader.read_u4()?, (0x0e, 0x0f));
        assert!(matches!(
            reader.read_u4(),
            Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: 1,
                current_offset: 3,
                available_bytes: 0,
            }),
        ));

        let mut reader = BinaryFileReader::from(
            vec![0x01, 0x02, 0x03, 0x04, 0x05]
        );
        assert_eq!(reader.read_u16()?, 0x0102);
        assert_eq!(reader.read_u16()?, 0x0304);
        assert!(matches!(
            reader.read_u16(),
            Err(BinaryFileReaderError::BufferUnderflow {
                requested_bytes: 2,
                current_offset: 4,
                available_bytes: 1,
            }),
        ));
        assert_eq!(reader.read()?,0x05);

        Ok(())
    }
    

    #[test]
    fn test_expect() -> Result<(), BinaryFileReaderError> {

        let v: Vec<u8> = (0..=255).collect();
        let mut reader1 = BinaryFileReader::from(v);
        reader1.expect_peek(&[0,1,2,3,4,5])?;
        reader1.expect(&[0,1,2,3,4,5])?;
        assert_eq!(reader1.available_bytes(), 256 - 6);
        assert_eq!(reader1.current_offset(),6);
        reader1.expect_peek(&[6,7,8,9,10])?;
        reader1.expect(&[6,7,8,9,10])?;
        assert_eq!(reader1.available_bytes(), 256 - 11);
        assert_eq!(reader1.current_offset(),11);

        let v: Vec<u8> = (0..=255).collect();
        let mut a = BinaryFileReader::from(v);
        let mut b = a.split_off_front(128)?; 
        b.expect(&[0,1,2,3,4])?;
        a.expect(&[128,129,130,131,132])?;
        assert_eq!(b.available_bytes(), 123);
        assert_eq!(a.available_bytes(), 123);
        assert_eq!(b.current_offset(), 5);
        assert_eq!(a.current_offset(), 128 + 5);

        let v: Vec<u8> = (0..=255).collect();
        let mut reader = BinaryFileReader::from(v);
        assert!(reader.expect(&[1,2,3,4,5]).is_err());
        assert!(reader.expect(&[0,1,2,3,4,5,7]).is_err());
        assert!(reader.expect_peek(&[1,2,3,4,5]).is_err());
        assert!(reader.expect_peek(&[0,1,2,3,4,5,7]).is_err());
        assert!(reader.expect_peek(&[0,1,2,3,4,5]).is_ok());
        assert!(reader.expect(&[0,1,2,3,4,5]).is_ok());
        assert!(reader.expect(&[0,1,2,3,4,5]).is_err());

        Ok(())
    }
    
    
}
