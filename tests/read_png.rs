use std::fs;

use binary_file_reader::{error::BinaryFileReaderError, BinaryFileReader};

#[test]
fn test() -> Result<(), BinaryFileReaderError> {
    let f = read_png("sample-files/1.png")?;
    let expect = vec![
        Chunk::Ihdr {
            width: 100,
            height: 100,
            bit_depth: 8,
            color_type: 2,
            compression_method: 0,
            fileter_method: 0,
            interlace_method: 0,
        },
        Chunk::Unknown,
        Chunk::Phys {
            px_per_unit_x: 11811,
            px_per_unit_y: 11811,
            unit_specifier: 1,
        },
        Chunk::Time {
            y: 2025,
            m: 1,
            d: 28,
        },
        Chunk::Text("Comment\0Created with GIMP".to_string()),
        Chunk::Unknown,
    ];
    assert_eq!(f, expect);

    Ok(())
}

fn read_png(path: &str) -> Result<Vec<Chunk>, BinaryFileReaderError> {
    let buffer = fs::read(path).expect("fault to read file");
    let mut reader = BinaryFileReader::new(&buffer);

    reader.expect(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])?;

    let mut chunks = Vec::new();

    loop {
        let chunk_length = reader.read_u32()?;
        let chunk_name = reader.read_utf8(4)?;
        let mut chunk_reader = reader.split_off_front(chunk_length as usize)?;
        let _crc = reader.read_u32()?;

        let chunk = match chunk_name {
            "IHDR" => Chunk::Ihdr {
                width: chunk_reader.read_u32()?,
                height: chunk_reader.read_u32()?,
                bit_depth: chunk_reader.read_u8()?,
                color_type: chunk_reader.read_u8()?,
                compression_method: chunk_reader.read_u8()?,
                fileter_method: chunk_reader.read_u8()?,
                interlace_method: chunk_reader.read_u8()?,
            },
            "tEXt" => Chunk::Text(
                chunk_reader
                    .read_utf8(chunk_reader.available_bytes())?
                    .to_string(),
            ),
            "pHYs" => Chunk::Phys {
                px_per_unit_x: chunk_reader.read_u32()?,
                px_per_unit_y: chunk_reader.read_u32()?,
                unit_specifier: chunk_reader.read_u8()?,
            },
            "tIME" => Chunk::Time {
                y: chunk_reader.read_u16()?,
                m: chunk_reader.read_u8()?,
                d: chunk_reader.read_u8()?,
            },
            "IEND" => {
                break;
            }
            _ => Chunk::Unknown,
        };

        chunks.push(chunk);
    }

    Ok(chunks)
}

#[derive(Debug, PartialEq)]
enum Chunk {
    Ihdr {
        width: u32,
        height: u32,
        bit_depth: u8,
        color_type: u8,
        compression_method: u8,
        fileter_method: u8,
        interlace_method: u8,
    },

    Text(String),

    Phys {
        px_per_unit_x: u32,
        px_per_unit_y: u32,
        unit_specifier: u8,
    },

    Time {
        y: u16,
        m: u8,
        d: u8,
    },

    Unknown,
}
