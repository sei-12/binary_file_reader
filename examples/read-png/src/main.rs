use std::{fmt::Error, fs};

use binary_file_reader::BinaryFileReader;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let buffer = fs::read("./sample-files/1.png").unwrap();
    let mut reader = BinaryFileReader::new(&buffer);
    
    // Read PNG signature
    reader.expect(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])?;

    // Read IHDR Chunk
    let ihdt_chunk_lenght = reader.read_u32()?;
    
    // let chunk_name 
    
    println!("Hello, world!");
    
    Ok(())
}