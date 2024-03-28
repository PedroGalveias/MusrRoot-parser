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
    run_header: RunHeader,
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
struct RunHeader {
    run_info: RunInfo,
    detector_info: DetectorInfo,
    sample_environment_info: SampleEnvironmentInfo,
    magnetic_field_environment_info: MagneticFieldEnvironmentInfo,
    beamline_info: BeamlineInfo,
}

#[derive(Debug)]
struct RunInfo {
    version: String,
    generic_validator_url: String,
    specific_validator_url: String,
    generator: String,
    file_name: String,
    run_title: String,
    run_number: i64,
    run_start_time: String,
    run_stop_time: String,
    // TODO: Implement TMusrRunPhysicalQuantity
    // run_duration: TMusrRunPhysicalQuantity,
    laboratory: String,
    instrument: String,
    // TODO: Implement TMusrRunPhysicalQuantity
    // muon_beam_momentum: TMusrRunPhysicalQuantity,
    muon_species: String,
    muon_source: String,
    setup: String,
    comment: String,
    sample_name: String,
    // TODO: Implement TMusrRunPhysicalQuantity
    // sample_temperature: TMusrRunPhysicalQuantity,
    // TODO: Implement TMusrRunPhysicalQuantity
    // sample_magnetic_field: TMusrRunPhysicalQuantity,
    no_of_histos: i64
    // TODO: Implement TMusrRunPhysicalQuantity
    // time_resolution: TMusrRunPhysicalQuantity,
    // TODO: Implement TIntVector
    // redGreen_offsets: TIntVector,
}

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
struct SampleEnvironmentInfo {
    cryo: String,
}

#[derive(Debug)]
struct MagneticFieldEnvironmentInfo {
    magnet_name: String,
}

#[derive(Debug)]
struct BeamlineInfo {
    name: String,
}


impl MusrRootFile {
    // Method to parse binary data into MusrRoot struct
    fn parse(bytes: &[u8]) -> Option<MusrRootFile> {
        let histos = Histos::parse(bytes)?;
        let run_header = RunHeader::parse(bytes)?;
        Some(MusrRootFile { histos, run_header })
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

impl RunHeader {
    // Method to parse binary data into RunHeader struct
    fn parse(bytes: &[u8]) -> Option<RunHeader> {
        // Ensure that there are enough bytes to parse the entire RunHeader struct
        let header_size = size_of::<RunHeader>();
        if bytes.len() < header_size {
            return None;
        }

        // Define the sizes of each sub-struct to calculate offsets
        let run_info_size = size_of::<RunInfo>();
        let detector_info_size = size_of::<DetectorInfo>();
        let sample_env_info_size = size_of::<SampleEnvironmentInfo>();
        let mag_field_env_info_size = size_of::<MagneticFieldEnvironmentInfo>();
        let beamline_info_size = size_of::<BeamlineInfo>();

        // Parse RunInfo struct
        let run_info = RunInfo::parse(&bytes[0..run_info_size])?;

        // Parse DetectorInfo struct
        let detector_info = DetectorInfo::parse(&bytes[run_info_size..run_info_size + detector_info_size])?;

        // Parse SampleEnvironmentInfo struct
        let sample_env_info = SampleEnvironmentInfo::parse(&bytes[run_info_size + detector_info_size..run_info_size + detector_info_size + sample_env_info_size])?;

        // Parse MagneticFieldEnvironmentInfo struct
        let mag_field_env_info = MagneticFieldEnvironmentInfo::parse(&bytes[run_info_size + detector_info_size + sample_env_info_size..run_info_size + detector_info_size + sample_env_info_size + mag_field_env_info_size])?;

        // Parse BeamlineInfo struct
        let beamline_info = BeamlineInfo::parse(&bytes[run_info_size + detector_info_size + sample_env_info_size + mag_field_env_info_size..])?;

        // Construct and return the RunHeader struct
        Some(RunHeader { run_info, detector_info, sample_environment_info: sample_env_info, magnetic_field_environment_info: mag_field_env_info, beamline_info })
    }
}

impl RunInfo {
    // Method to parse binary data into RunInfo struct
    fn parse(bytes: &[u8]) -> Option<RunInfo> {
        // Ensure that there are enough bytes to parse the entire RunInfo struct
        let struct_size = size_of::<RunInfo>();
        if bytes.len() < struct_size {
            return None;
        }

        // Convert the byte slices into strings
        let version = String::from_utf8_lossy(&bytes[0..16]).trim_end_matches('\0').to_string();
        let generic_validator_url = String::from_utf8_lossy(&bytes[16..32]).trim_end_matches('\0').to_string();
        let specific_validator_url = String::from_utf8_lossy(&bytes[32..48]).trim_end_matches('\0').to_string();
        let generator = String::from_utf8_lossy(&bytes[48..64]).trim_end_matches('\0').to_string();
        let file_name = String::from_utf8_lossy(&bytes[64..80]).trim_end_matches('\0').to_string();
        let run_title = String::from_utf8_lossy(&bytes[80..96]).trim_end_matches('\0').to_string();
        let run_number = i64::from_le_bytes(bytes[96..104].try_into().unwrap());
        let run_start_time = String::from_utf8_lossy(&bytes[104..120]).trim_end_matches('\0').to_string();
        let run_stop_time = String::from_utf8_lossy(&bytes[120..136]).trim_end_matches('\0').to_string();
        let laboratory = String::from_utf8_lossy(&bytes[136..152]).trim_end_matches('\0').to_string();
        let instrument = String::from_utf8_lossy(&bytes[152..168]).trim_end_matches('\0').to_string();
        let muon_species = String::from_utf8_lossy(&bytes[176..192]).trim_end_matches('\0').to_string();
        let muon_source = String::from_utf8_lossy(&bytes[192..208]).trim_end_matches('\0').to_string();
        let setup = String::from_utf8_lossy(&bytes[208..224]).trim_end_matches('\0').to_string();
        let comment = String::from_utf8_lossy(&bytes[224..240]).trim_end_matches('\0').to_string();
        let sample_name = String::from_utf8_lossy(&bytes[240..256]).trim_end_matches('\0').to_string();
        let no_of_histos = i64::from_le_bytes(bytes[256..264].try_into().unwrap());

        // Create and return the RunInfo struct
        Some(RunInfo {
            version,
            generic_validator_url,
            specific_validator_url,
            generator,
            file_name,
            run_title,
            run_number,
            run_start_time,
            run_stop_time,
            laboratory,
            instrument,
            muon_species,
            muon_source,
            setup,
            comment,
            sample_name,
            no_of_histos,
        })
    }
}

impl DetectorInfo {
    // Method to parse binary data into DetectorInfo struct
    fn parse(bytes: &[u8]) -> Option<DetectorInfo> {
        // Ensure that there are enough bytes to parse the entire DetectorInfo struct
        let struct_size = size_of::<DetectorInfo>();
        if bytes.len() < struct_size {
            return None;
        }

        // Calculate the number of detectors based on the size of each detector struct
        let detector_size = size_of::<Detector>();
        let num_detectors = bytes.len() / detector_size;

        // Create a vector to store parsed Detector structs
        let mut detectors = Vec::with_capacity(num_detectors);

        // Parse each detector from the binary data
        for i in 0..num_detectors {
            let start = i * detector_size;
            let end = start + detector_size;
            let detector_bytes = &bytes[start..end];
            if let Some(detector) = Detector::parse(detector_bytes) {
                detectors.push(detector);
            } else {
                return None; // Failed to parse detector, return None
            }
        }

        // Create and return the DetectorInfo struct
        Some(DetectorInfo { detectors })
    }
}

impl Detector {
    // Method to parse binary data into Detector struct
    fn parse(bytes: &[u8]) -> Option<Detector> {
        // Ensure that there are enough bytes to parse the entire Detector struct
        let struct_size = size_of::<Detector>();
        if bytes.len() < struct_size {
            return None;
        }

        // Convert the byte slices into strings
        let name = String::from_utf8_lossy(&bytes[0..16]).trim_end_matches('\0').to_string();

        // Parse the remaining fields from the byte slice
        let histo_number = i64::from_le_bytes(bytes[16..24].try_into().unwrap());
        let histo_length = i64::from_le_bytes(bytes[24..32].try_into().unwrap());
        let time_zero_bin = f64::from_le_bytes(bytes[32..40].try_into().unwrap());
        let first_good_bin = i64::from_le_bytes(bytes[40..48].try_into().unwrap());
        let last_good_bin = i64::from_le_bytes(bytes[48..56].try_into().unwrap());

        // Create and return the Detector struct
        Some(Detector {
            name,
            histo_number,
            histo_length,
            time_zero_bin,
            first_good_bin,
            last_good_bin,
        })
    }
}

impl SampleEnvironmentInfo {
    // Method to parse binary data into SampleEnvironmentInfo struct
    fn parse(bytes: &[u8]) -> Option<SampleEnvironmentInfo> {
        // Ensure that there are enough bytes to parse the entire SampleEnvironmentInfo struct
        let struct_size = size_of::<SampleEnvironmentInfo>();
        if bytes.len() < struct_size {
            return None;
        }

        // Convert the byte slices into strings
        let cryo = String::from_utf8_lossy(&bytes[0..16]).trim_end_matches('\0').to_string();

        // Create and return the SampleEnvironmentInfo struct
        Some(SampleEnvironmentInfo { cryo })
    }
}

impl MagneticFieldEnvironmentInfo {
    // Method to parse binary data into MagneticFieldEnvironmentInfo struct
    fn parse(bytes: &[u8]) -> Option<MagneticFieldEnvironmentInfo> {
        // Ensure that there are enough bytes to parse the entire MagneticFieldEnvironmentInfo struct
        let struct_size = size_of::<MagneticFieldEnvironmentInfo>();
        if bytes.len() < struct_size {
            return None;
        }

        // Convert the byte slices into strings
        let magnet_name = String::from_utf8_lossy(&bytes[0..16]).trim_end_matches('\0').to_string();

        // Create and return the MagneticFieldEnvironmentInfo struct
        Some(MagneticFieldEnvironmentInfo { magnet_name })
    }
}

impl BeamlineInfo {
    // Method to parse binary data into BeamlineInfo struct
    fn parse(bytes: &[u8]) -> Option<BeamlineInfo> {
        // Ensure that there are enough bytes to parse the entire BeamlineInfo struct
        let struct_size = size_of::<BeamlineInfo>();
        if bytes.len() < struct_size {
            return None;
        }

        // Convert the byte slices into strings
        let name = String::from_utf8_lossy(&bytes[0..16]).trim_end_matches('\0').to_string();

        // Create and return the BeamlineInfo struct
        Some(BeamlineInfo { name })
    }
}


fn musr_root_file_parser() -> Result<(), Box<dyn Error>> {
    // Open the binary file
    let mut file = File::open("./src/lem23_his_0001.root")?;

    // Read the entire contents of the file into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Parse the binary data
    if let Some(musr_root_file) = MusrRootFile::parse(&buffer) {
        println!("Parsed data: {:?}", musr_root_file);
    } else {
        eprintln!("Failed to parse MUSR Root File");
    }

    Ok(())
}
