use crate::error::ParsingError;
use crate::models::*;
use std::fs::File;
use std::io::Read;

pub fn parse_musr_root_file(file_path: &str) -> Result<MusrRootFile, ParsingError> {
    // Open the binary file
    let mut file = File::open(file_path)?;

    // Read the entire contents of the file into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Parse the buffer into a MusrRootFile struct
    MusrRootFile::parse(&buffer)
        .ok_or_else(|| ParsingError::ParseError("Failed to parse MUSR Root File".into()))
}
