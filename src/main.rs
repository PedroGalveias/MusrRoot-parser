use druid::{PlatformError, WidgetExt};
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::mem::size_of;

#[tokio::main]
async fn main() -> Result<(), PlatformError> {
    // Call your function and match the result
    match musr_root_file_parser() {
        Ok(()) => {
            // Handle the successful case
            println!("Musr Root File Parser function executed successfully!")
        }
        Err(err) => {
            // Handle the error case
            eprintln!("Error: {}", err)
        }
    }
    Ok(())
}
#[derive(Debug)]
struct MusrRootFile {
    histos: Histos,
}

#[derive(Debug)]
struct Histos {
    decay_ana_module: DecayAnaModule,
    sc_ana_module: SCAnaModule,
}

#[derive(Debug)]
struct DecayAnaModule {
    h_decay: Vec<HDecay>,
}

#[derive(Debug)]
struct HDecay {
    // Define fields specific to hDecay
    // For example:
    field1: f64,
    field2: f64,
    field3: f64,
}

#[derive(Debug)]
struct SCAnaModule {
    // Define fields specific to SCAnaModule
    h_sample_temperature: f64,
    h_sample_magnetic_field: f64,
}

#[derive(Debug)]
struct RunHeader {}

#[derive(Debug)]
struct RunInfo {}

#[derive(Debug)]
struct DetectorInfo {
    detectors: Vec<Detector>,
}

#[derive(Debug)]
struct Detector {
    name: String,
    histo_number: i64,
    histo_length: i64,
    time_zero_bin: f64,
    first_good_bin: i64,
    last_good_bin: i64,
}

#[derive(Debug)]
struct SampleEnvironmentInfo {}

#[derive(Debug)]
struct MagneticFieldEnvironmentInfo {}

#[derive(Debug)]
struct BeamlineInfo {}


impl MusrRootFile {
    // Method to parse binary data into MusrRoot struct
    fn parse(bytes: &[u8]) -> Option<MusrRootFile> {
        let histos = Histos::parse(bytes)?;
        Some(MusrRootFile { histos })
    }
}

impl Histos {
    // Method to parse binary data into Histos struct
    fn parse(bytes: &[u8]) -> Option<Histos> {
        let decay_ana_module = DecayAnaModule::parse(bytes)?;
        let sc_ana_module = SCAnaModule::parse(bytes)?;
        Some(Histos { decay_ana_module, sc_ana_module })
    }
}

impl HDecay {
    // Method to parse binary data into HDecay struct
    fn parse(bytes: &[u8]) -> Option<HDecay> {
        // Check if there are enough bytes to parse the HDecay struct
        if bytes.len() < size_of::<HDecay>() {
            return None;
        }

        // Extract the fields from the binary data
        // Example: Assuming field1, field2, field3 are all f64 values
        let field1_bytes = &bytes[0..8];
        let field2_bytes = &bytes[8..16];
        let field3_bytes = &bytes[16..24];

        // Convert the bytes into f64 values using from_le_bytes for little-endian bytes
        let field1 = f64::from_le_bytes(field1_bytes.try_into().unwrap());
        let field2 = f64::from_le_bytes(field2_bytes.try_into().unwrap());
        let field3 = f64::from_le_bytes(field3_bytes.try_into().unwrap());

        // Create and return the HDecay struct
        Some(HDecay { field1, field2, field3 })
    }
}

impl DecayAnaModule {
    // Method to parse binary data into DecayAnaModule struct
    fn parse(bytes: &[u8]) -> Option<DecayAnaModule> {
        // Calculate the size of each HDecay struct in bytes
        let h_decay_size = size_of::<HDecay>();

        // Check if there are enough bytes to parse at least one HDecay struct
        if bytes.len() < h_decay_size {
            return None;
        }

        // Calculate the number of HDecay structs in the binary data
        let num_h_decays = bytes.len() / h_decay_size;

        // Create a vector to store the parsed HDecay structs
        let mut h_decays = Vec::with_capacity(num_h_decays);

        // Iterate over the binary data to parse each HDecay struct
        for i in 0..num_h_decays {
            // Calculate the start and end positions of the current HDecay struct in the byte slice
            let start = i * h_decay_size;
            let end = start + h_decay_size;

            // Extract the bytes for the current HDecay struct
            let h_decay_bytes = &bytes[start..end];

            // Parse the HDecay struct from the bytes
            if let Some(h_decay) = HDecay::parse(h_decay_bytes) {
                h_decays.push(h_decay);
            } else {
                // Failed to parse HDecay struct, return None
                return None;
            }
        }

        // Return the parsed DecayAnaModule struct
        Some(DecayAnaModule { h_decay: h_decays })
    }
}

impl SCAnaModule {
    // Method to parse binary data into SCAnaModule struct
    fn parse(bytes: &[u8]) -> Option<SCAnaModule> {
        // Implement parsing logic for SCAnaModule
        // Example: Assuming h_sample_temperature and h_sample_magnetic_field are both f64 values

        // Check if there are enough bytes to parse the SCAnaModule struct
        let struct_size = size_of::<SCAnaModule>();
        if bytes.len() < struct_size {
            return None;
        }

        // Extract the fields from the binary data
        let temperature_bytes = &bytes[0..8];
        let magnetic_field_bytes = &bytes[8..16];

        // Convert the bytes into f64 values using from_le_bytes for little-endian bytes
        let temperature = f64::from_le_bytes(temperature_bytes.try_into().unwrap());
        let magnetic_field = f64::from_le_bytes(magnetic_field_bytes.try_into().unwrap());

        // Create and return the SCAnaModule struct
        Some(SCAnaModule { h_sample_temperature: temperature, h_sample_magnetic_field: magnetic_field })
    }
}

impl RunInfo {}

fn musr_root_file_parser() -> Result<(), Box<dyn Error>> {
    // Open the binary file
    let mut file = File::open("./src/lem23_his_0001.root")?;

    // Read the entire contents of the file into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Parse the binary data
    if let Some(data) = MusrRootFile::parse(&buffer) {
        println!("Parsed data: {:?}", data);
    } else {
        eprintln!("Failed to parse MUSR Root File");
    }

    Ok(())
}
