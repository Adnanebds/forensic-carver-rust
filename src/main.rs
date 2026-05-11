use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// not ideal i know but for now fine
const BUFFER_SIZE: usize = 4096;

fn read_file<R: Read>(mut reader: R) -> Result<(), Box<dyn Error>> {
    // this bool is used to determine if we are currently carving out a JPEG file
    let mut is_carving = false;
    let mut buffer = [0_u8; BUFFER_SIZE];
    // start of jpeg file marker to know where to start writing the bytes to the output file
    let target = [0xFF, 0xD8];
    // create the output file where we will write the carved JPEG data
    let mut file = File::create("resolved.jpg")?;
    loop {
        // read a chunk of data into the buffer
        let count = reader.read(&mut buffer)?;
        // if count is 0, it means we've reached the end of the file
        if count == 0 {
            break;
        }

        // check if the target marker is found in the buffer
        if let Some(start_index) = buffer.windows(2).position(|window| window == target) {
            is_carving = true;
            // Process the buffer starting from start_index
            file.write_all(&buffer[start_index..])?;
            // check for the end of the JPEG file marker so we know when to stop writing the data to the output file
        } else if let Some(end_index) = buffer.windows(2).position(|window| window == [0xFF, 0xD9])
        {
            is_carving = false;
            file.write_all(&buffer[..end_index + 2])?;
        // this is for when we are in the middle of carving a jpeg file so no start or end marker is true here.
        } else if is_carving {
            file.write_all(&buffer)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .nth(2) returns Option<String> i did this instead of manual indexing to avoid potential out of bounds error
    let path = env::args().nth(2);

    if let Some(real_path) = path {
        let input = File::open(real_path)?;
        let reader = BufReader::new(input);
        read_file(reader)?;
    } else {
        eprintln!("Error: Missing path argument at index 2");
    }
    Ok(())
}
