use binary_file_reader::BinaryFileReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buffer = std::fs::read("./sample-files/1.png").unwrap();

    let mut reader = BinaryFileReader::new(&buffer);

    // Read PNG signature
    reader.expect(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])?;

    loop {
        let chunk_length = reader.read_u32()?;
        let chunk_name = reader.read_utf8(4)?;
        let mut chunk_reader = reader.split_off_front(chunk_length as usize)?;
        let _crc = reader.read_u32()?;

        println!();
        println!("Chunk Name: {}", chunk_name);
        println!("  {:<20}: {}", "length", chunk_length);

        match chunk_name {
            #[rustfmt::skip]
            "IHDR" => {
                println!("  {:<20}: {}", "width", chunk_reader.read_u32()?);
                println!("  {:<20}: {}", "height", chunk_reader.read_u32()?);
                println!("  {:<20}: {}", "bit depth", chunk_reader.read_u8()?);
                println!("  {:<20}: {}", "color type", chunk_reader.read_u8()?);
                println!("  {:<20}: {}", "compression method", chunk_reader.read_u8()?);
                println!("  {:<20}: {}", "filter method", chunk_reader.read_u8()?);
                println!("  {:<20}: {}", "interlace method", chunk_reader.read_u8()?);
            }
            "tEXt" => {
                println!("  {}", chunk_reader.read_utf8(chunk_length as usize)?)
            }
            "pHYs" => {
                println!("  {:<20}: {}", "PX per unit, X axis", chunk_reader.read_u32()?);
                println!("  {:<20}: {}", "PX per unit, Y axis", chunk_reader.read_u32()?);
                println!("  {:<20}: {}", "Unit specifier", chunk_reader.read_u8()?);
            }
            "tIME" => {
                println!(
                    "  {}/{}/{} {}:{}:{}",
                    chunk_reader.read_u16()?,
                    chunk_reader.read_u8()?,
                    chunk_reader.read_u8()?,
                    chunk_reader.read_u8()?,
                    chunk_reader.read_u8()?,
                    chunk_reader.read_u8()?,
                )
            }
            "IEND" => {
                break;
            }
            _ => { /* Unknown Chunk */ }
        }
    }

    Ok(())
}
