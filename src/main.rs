mod error;
mod models;
mod musr_root_file_parser;

use crate::error::ParsingError;
use crate::musr_root_file_parser::parse_musr_root_file;

fn main() -> Result<(), ParsingError> {
    // Get the file path from command line arguments
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //     eprintln!("Usage: {} <path to binary file>", args[0]);
    //     std::process::exit(1);
    // }
    // let file_path = &args[1];
    // let mut file = File::open("./src/lem24_his_2000.root")?;

    // Parse the file
    let musr_root_file = parse_musr_root_file("./src/lem24_his_2000.root")?;

    // Do something with the parsed data
    println!("{:?}", musr_root_file);

    Ok(())
}
