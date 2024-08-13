use std::mem::size_of;

#[derive(Debug)]
pub struct MusrRootFile {
    pub histos: Histos,
    pub run_header: RunHeader,
}

#[derive(Debug)]
pub struct Histos {
    pub decay_ana_module: DecayAnaModule,
    pub sc_ana_module: SCAnaModule,
}

#[derive(Debug)]
pub struct DecayAnaModule {
    pub h_decay: Vec<HDecay>,
}

#[derive(Debug)]
pub struct HDecay {
    // Here it is assumed that there are hypothetical red / green data with electric field on/off
    //  and light on/off, and hence 4 data sets per detector, and 8 detectors of the instrument:
    //      left/forward, top/forward, right/forward, bottom/forward, left/backward, top/backward, right/backward, bottom/backward.
    //
    // E.g.,  hDecay001 # left/forward, electric field off, light off
    //
    // Aditional comments: as can be seen the histograms are continuous numbered until there is a red / green mode switch where the histogram number “jumps” (e.g. from 008 to 011).
    // In order to fill in the different red / green histograms an offset is added (here 10, 20, and 30).
    //
    // Example:
    //
    // hDecay007 # right/backward, electric field off, light off
    // hDecay008 # bottom/backward, electric field off, light off
    // hDecay011 # left/forward, electric field on, light off
    // hDecay012 # top/forward, electric field on, light off
    //
    // Check PSI doc link in the README file for more information.
    pub field1: f64,
    pub field2: f64,
    pub field3: f64,
}

#[derive(Debug)]
pub struct SCAnaModule {
    pub h_sample_temperature: f64,
    pub h_sample_magnetic_field: f64,
}

// In the RunHeader (except for the last part of it, the RunSummary, all fields follow this rule: <number> - <label>: <value> -@<type>.
// Example:
//  000 - Version: $Id: TMusrRunHeader.cpp 5092 2012-03-13 07:47:00Z nemu $ -@0
//  001 - Generic Validator URL: http://lmu.web.psi.ch/facilities/software/MusrRoot/Validation/MusrRoot.xsd -@0
//  009 - Run Duration: 17305 sec -@3
//
// RunSummary examples:
// 0000 - Wed Oct  5 01:30:37 2011 Run 2856 started.
// 0001 - Wed Oct  5 02:02:51 2011 Run 2856 stopped.
// 0002 -
// 0003 - LCO, T=170.02(K), wTF ~30(G)/5.18(A), Tr/Sa=15.02/8.50(kV), E=5.63(keV), LEDb off, BP off
// 0004 - =========================================================================================
#[derive(Debug)]
pub struct RunHeader {
    pub run_info: RunInfo,
    pub detector_info: DetectorInfo,
    pub sample_environment_info: SampleEnvironmentInfo,
    pub magnetic_field_environment_info: MagneticFieldEnvironmentInfo,
    pub beamline_info: BeamlineInfo,
}

// TMusrRunPhysicalQuantity is a data type (represents Physical quantities) that can be described as: <property name>: <value> +- <estimated error> <unit>; SP: <demand>; <description>. It can also contain various representations.
// Check link for documentation: https://lmu.web.psi.ch/musrfit/user/html/musr-root.html#musr-run-physical-quantity
//
// TIntVector is a collection of integers.
//
// TStringVector is a Collection of strings.
//
// TDoubleVector is a collection of floating point numbers.
//
// Check link for documentation ("TMusrRunHeader Concept" section): https://lmu.web.psi.ch/musrfit/user/html/musr-root.html
#[derive(Debug)]
pub struct RunInfo {
    pub version: String,                // Git version of `TMusrRunHeader`
    pub generic_validator_url: String,  // URL
    pub specific_validator_url: String, // URL
    pub generator: String,              // program which wrote the MusrRoot file e.g., nemu_analyzer
    pub file_name: String, // file name of the MusrRoot file e.g., deltat_tdc_gps_4295.root
    pub run_title: String,
    pub run_number: i64,
    pub run_start_time: String, // ISO 8601 date time
    pub run_stop_time: String,  // ISO 8601 date time
    // TODO: Missing `Run Duration` attribute of type TMusrRunPhysicalQuantity. e.g., "run duration in sec"
    pub laboratory: String, // e.g., PSI
    pub instrument: String, // e.g., GPS
    // TODO: Missing `Muon Beam Momentum` attribute of type TMusrRunPhysicalQuantity e.g. "28.1 MeV/c"
    pub muon_species: String, //  positive or negative muon
    pub muon_source: String,  // e.g. “Target E - Low Energy Muons” or “Target M” …
    pub setup: String,
    pub comment: String,
    pub sample_name: String,
    // TODO: Missing `Sample Temperature` attribute of type TMusrRunPhysicalQuantity e.g. 3.21 +- 0.05 K; SP: 3.2; CF1
    // TODO: Missing `Sample Magnetic Field` attribute of type TMusrRunPhysicalQuantity e.g. 350.002 +- 0.005 G; SP: 350; WEW
    pub no_of_histos: i64,
    // TODO: Missing `Time Resolution` attribute of type TMusrRunPhysicalQuantity e.g. 0.1953125 ns
    // TODO: Missing `RedGreen Offsets` attribute of type TIntVector e.g. 0; 20
}

#[derive(Debug)]
pub struct DetectorInfo {
    pub detectors: Vec<Detector>,
}

#[derive(Debug)]
pub struct Detector {
    pub name: String,       // detector name, e.g. Left-NPP
    pub histo_number: i64, // histogram number. This number corresponds to the histogram number in the histos/DecayAnaModule sub-tree.
    pub histo_length: i64, // length of the histogram (in bins)
    pub time_zero_bin: f64, // The type is Double_t since for the high-field spectrometer at PSI an Int_t representation would be not good enough.
    pub first_good_bin: i64,
    pub last_good_bin: i64,
}

#[derive(Debug)]
pub struct SampleEnvironmentInfo {
    pub cryo: String, // name of the used cryostat/oven, e.g. Konti-2
}

#[derive(Debug)]
pub struct MagneticFieldEnvironmentInfo {
    pub magnet_name: String, // name of the used magnet, e.g. WEW. In case of ZF measurements, there might be an entry like ZF.
}

#[derive(Debug)]
pub struct BeamlineInfo {
    pub name: String, // name of the beamline, e.g. piM3.2
}

impl MusrRootFile {
    pub fn parse(bytes: &[u8]) -> Option<MusrRootFile> {
        let histos = Histos::parse(bytes)?;
        let run_header = RunHeader::parse(bytes)?;
        Some(MusrRootFile { histos, run_header })
    }
}

impl Histos {
    pub fn parse(bytes: &[u8]) -> Option<Histos> {
        let decay_ana_module = DecayAnaModule::parse(bytes)?;
        let sc_ana_module = SCAnaModule::parse(bytes)?;
        Some(Histos {
            decay_ana_module,
            sc_ana_module,
        })
    }
}

impl HDecay {
    pub fn parse(bytes: &[u8]) -> Option<HDecay> {
        if bytes.len() < size_of::<HDecay>() {
            return None;
        }

        let field1_bytes = &bytes[0..8];
        let field2_bytes = &bytes[8..16];
        let field3_bytes: &[u8] = &bytes[16..24];

        let field1 = f64::from_le_bytes(field1_bytes.try_into().unwrap());
        let field2 = f64::from_le_bytes(field2_bytes.try_into().unwrap());
        let field3 = f64::from_le_bytes(field3_bytes.try_into().unwrap());

        Some(HDecay {
            field1,
            field2,
            field3,
        })
    }
}

impl DecayAnaModule {
    pub fn parse(bytes: &[u8]) -> Option<DecayAnaModule> {
        let h_decay_size = size_of::<HDecay>();
        if bytes.len() < h_decay_size {
            return None;
        }

        let num_h_decays = bytes.len() / h_decay_size;
        let mut h_decays = Vec::with_capacity(num_h_decays);

        for i in 0..num_h_decays {
            let start = i * h_decay_size;
            let end = start + h_decay_size;
            let h_decay_bytes = &bytes[start..end];

            if let Some(h_decay) = HDecay::parse(h_decay_bytes) {
                h_decays.push(h_decay);
            } else {
                return None;
            }
        }

        Some(DecayAnaModule { h_decay: h_decays })
    }
}

impl SCAnaModule {
    pub fn parse(bytes: &[u8]) -> Option<SCAnaModule> {
        let struct_size = size_of::<SCAnaModule>();
        if bytes.len() < struct_size {
            return None;
        }

        let temperature_bytes = &bytes[0..8];
        let magnetic_field_bytes = &bytes[8..16];

        let temperature = f64::from_le_bytes(temperature_bytes.try_into().unwrap());
        let magnetic_field = f64::from_le_bytes(magnetic_field_bytes.try_into().unwrap());

        Some(SCAnaModule {
            h_sample_temperature: temperature,
            h_sample_magnetic_field: magnetic_field,
        })
    }
}

impl RunHeader {
    pub fn parse(bytes: &[u8]) -> Option<RunHeader> {
        let header_size = size_of::<RunHeader>();
        if bytes.len() < header_size {
            return None;
        }

        let run_info_size = size_of::<RunInfo>();
        let detector_info_size = size_of::<DetectorInfo>();
        let sample_env_info_size = size_of::<SampleEnvironmentInfo>();
        let mag_field_env_info_size = size_of::<MagneticFieldEnvironmentInfo>();
        let beamline_info_size = size_of::<BeamlineInfo>(); // TODO: review! var is not being used

        let run_info = RunInfo::parse(&bytes[0..run_info_size])?;
        let detector_info =
            DetectorInfo::parse(&bytes[run_info_size..run_info_size + detector_info_size])?;
        let sample_env_info = SampleEnvironmentInfo::parse(
            &bytes[run_info_size + detector_info_size
                ..run_info_size + detector_info_size + sample_env_info_size],
        )?;
        let mag_field_env_info = MagneticFieldEnvironmentInfo::parse(
            &bytes[run_info_size + detector_info_size + sample_env_info_size
                ..run_info_size
                    + detector_info_size
                    + sample_env_info_size
                    + mag_field_env_info_size],
        )?;
        let beamline_info = BeamlineInfo::parse(
            &bytes[run_info_size
                + detector_info_size
                + sample_env_info_size
                + mag_field_env_info_size..],
        )?;

        Some(RunHeader {
            run_info,
            detector_info,
            sample_environment_info: sample_env_info,
            magnetic_field_environment_info: mag_field_env_info,
            beamline_info,
        })
    }
}

impl RunInfo {
    pub fn parse(bytes: &[u8]) -> Option<RunInfo> {
        let struct_size = size_of::<RunInfo>();
        if bytes.len() < struct_size {
            return None;
        }

        let version = String::from_utf8_lossy(&bytes[0..16])
            .trim_end_matches('\0')
            .to_string();
        let generic_validator_url = String::from_utf8_lossy(&bytes[16..32])
            .trim_end_matches('\0')
            .to_string();
        let specific_validator_url = String::from_utf8_lossy(&bytes[32..48])
            .trim_end_matches('\0')
            .to_string();
        let generator = String::from_utf8_lossy(&bytes[48..64])
            .trim_end_matches('\0')
            .to_string();
        let file_name = String::from_utf8_lossy(&bytes[64..80])
            .trim_end_matches('\0')
            .to_string();
        let run_title = String::from_utf8_lossy(&bytes[80..96])
            .trim_end_matches('\0')
            .to_string();
        let run_number = i64::from_le_bytes(bytes[96..104].try_into().unwrap());
        let run_start_time = String::from_utf8_lossy(&bytes[104..120])
            .trim_end_matches('\0')
            .to_string();
        let run_stop_time = String::from_utf8_lossy(&bytes[120..136])
            .trim_end_matches('\0')
            .to_string();
        let laboratory = String::from_utf8_lossy(&bytes[136..152])
            .trim_end_matches('\0')
            .to_string();
        let instrument = String::from_utf8_lossy(&bytes[152..168])
            .trim_end_matches('\0')
            .to_string();
        let muon_species = String::from_utf8_lossy(&bytes[176..192])
            .trim_end_matches('\0')
            .to_string();
        let muon_source = String::from_utf8_lossy(&bytes[192..208])
            .trim_end_matches('\0')
            .to_string();
        let setup = String::from_utf8_lossy(&bytes[208..224])
            .trim_end_matches('\0')
            .to_string();
        let comment = String::from_utf8_lossy(&bytes[224..240])
            .trim_end_matches('\0')
            .to_string();
        let sample_name = String::from_utf8_lossy(&bytes[240..256])
            .trim_end_matches('\0')
            .to_string();
        let no_of_histos = i64::from_le_bytes(bytes[256..264].try_into().unwrap());

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
    pub fn parse(bytes: &[u8]) -> Option<DetectorInfo> {
        let struct_size = size_of::<DetectorInfo>();
        if bytes.len() < struct_size {
            return None;
        }

        let detector_size = size_of::<Detector>();
        let num_detectors = bytes.len() / detector_size;
        let mut detectors = Vec::with_capacity(num_detectors);

        for i in 0..num_detectors {
            let start = i * detector_size;
            let end = start + detector_size;
            let detector_bytes = &bytes[start..end];
            if let Some(detector) = Detector::parse(detector_bytes) {
                detectors.push(detector);
            } else {
                return None;
            }
        }

        Some(DetectorInfo { detectors })
    }
}

impl Detector {
    pub fn parse(bytes: &[u8]) -> Option<Detector> {
        let struct_size = size_of::<Detector>();
        if bytes.len() < struct_size {
            return None;
        }

        let name = String::from_utf8_lossy(&bytes[0..16])
            .trim_end_matches('\0')
            .to_string();
        let histo_number = i64::from_le_bytes(bytes[16..24].try_into().unwrap());
        let histo_length = i64::from_le_bytes(bytes[24..32].try_into().unwrap());
        let time_zero_bin = f64::from_le_bytes(bytes[32..40].try_into().unwrap());
        let first_good_bin = i64::from_le_bytes(bytes[40..48].try_into().unwrap());
        let last_good_bin = i64::from_le_bytes(bytes[48..56].try_into().unwrap());

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
    pub fn parse(bytes: &[u8]) -> Option<SampleEnvironmentInfo> {
        let struct_size = size_of::<SampleEnvironmentInfo>();
        if bytes.len() < struct_size {
            return None;
        }

        let cryo = String::from_utf8_lossy(&bytes[0..16])
            .trim_end_matches('\0')
            .to_string();

        Some(SampleEnvironmentInfo { cryo })
    }
}

impl MagneticFieldEnvironmentInfo {
    pub fn parse(bytes: &[u8]) -> Option<MagneticFieldEnvironmentInfo> {
        let struct_size = size_of::<MagneticFieldEnvironmentInfo>();
        if bytes.len() < struct_size {
            return None;
        }

        let magnet_name = String::from_utf8_lossy(&bytes[0..16])
            .trim_end_matches('\0')
            .to_string();

        Some(MagneticFieldEnvironmentInfo { magnet_name })
    }
}

impl BeamlineInfo {
    pub fn parse(bytes: &[u8]) -> Option<BeamlineInfo> {
        let struct_size = size_of::<BeamlineInfo>();
        if bytes.len() < struct_size {
            return None;
        }

        let name = String::from_utf8_lossy(&bytes[0..16])
            .trim_end_matches('\0')
            .to_string();

        Some(BeamlineInfo { name })
    }
}
