mod musr_root_file_parser;
mod models;
mod error;

use std::error::Error;
use std::fs::File;
use std::io::Read;


fn main() -> Result<(), Box<dyn Error>> {
    // Open the binary file
    let mut file = File::open("./src/lem23_his_0001.root")?;

    // Read the entire contents of the file into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Parse a MUSR Root file (lemXX.root) to a MusrRootFile struct
    match musr_root_file_parser::run(&buffer) {
        Ok(parsed_file) => {
            // Print the parsed MusrRootFile
            println!("{:?}", parsed_file);
            Ok(())
        }
        Err(err) => {
            // Handle parsing error
            eprintln!("Parsing error: {:?}", err);
            Err(err.into())
        }
    }
}
