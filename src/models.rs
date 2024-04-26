#[derive(Debug)]
pub(crate) struct MusrRootFile {
    pub(crate) histos: Histos,
    pub(crate) run_header: RunHeader,
}

#[derive(Debug)]
pub(crate) struct Histos {
    pub(crate) decay_ana_module: DecayAnaModule,
    pub(crate) sc_ana_module: SCAnaModule,
}

#[derive(Debug)]
pub(crate) struct DecayAnaModule {
    pub(crate) h_decay: Vec<HDecay>,
}

#[derive(Debug)]
pub(crate) struct HDecay {
    // Define fields specific to HDecay
    pub(crate) name: String,
    pub(crate) histo_number: i64,
    pub(crate) histo_length: i64,
    pub(crate) time_zero_bin: f64,
    pub(crate) first_good_bin: i64,
    pub(crate) last_good_bin: i64,
}

#[derive(Debug)]
pub(crate) struct SCAnaModule {
    // Define fields specific to SCAnaModule
    // TODO: Implement TMusrRunPhysicalQuantity
    pub(crate) h_sample_temperature: String,
    // TODO: Implement TMusrRunPhysicalQuantity
    pub(crate) h_sample_magnetic_field: String,
}

#[derive(Debug)]
pub(crate) struct RunHeader {
    pub(crate) run_info: RunInfo,
    pub(crate) detector_info: DetectorInfo,
    pub(crate) sample_environment_info: SampleEnvironmentInfo,
    pub(crate) magnetic_field_environment_info: MagneticFieldEnvironmentInfo,
    pub(crate) beamline_info: BeamlineInfo,
}

#[derive(Debug)]
pub(crate) struct RunInfo {
    pub(crate) version: String,
    pub(crate) generic_validator_url: String,
    pub(crate) specific_validator_url: String,
    pub(crate) generator: String,
    pub(crate) file_name: String,
    pub(crate) run_title: String,
    pub(crate) run_number: i64,
    pub(crate) run_start_time: String,
    pub(crate) run_stop_time: String,
    // TODO: Implement TMusrRunPhysicalQuantity
    // run_duration: TMusrRunPhysicalQuantity,
    pub(crate) laboratory: String,
    pub(crate) instrument: String,
    // TODO: Implement TMusrRunPhysicalQuantity
    // muon_beam_momentum: TMusrRunPhysicalQuantity,
    pub(crate) muon_species: String,
    pub(crate) muon_source: String,
    pub(crate) setup: String,
    pub(crate) comment: String,
    pub(crate) sample_name: String,
    // TODO: Implement TMusrRunPhysicalQuantity
    // sample_temperature: TMusrRunPhysicalQuantity,
    // TODO: Implement TMusrRunPhysicalQuantity
    // sample_magnetic_field: TMusrRunPhysicalQuantity,
    pub(crate) no_of_histos: i64, // TODO: Implement TMusrRunPhysicalQuantity
                                  // time_resolution: TMusrRunPhysicalQuantity,
                                  // TODO: Implement TIntVector
                                  // redGreen_offsets: TIntVector,
}

#[derive(Debug)]
pub(crate) struct DetectorInfo {
    pub(crate) detectors: Vec<Detector>,
}

#[derive(Debug)]
pub(crate) struct Detector {
    pub(crate) name: String,
    pub(crate) histo_number: i64,
    pub(crate) histo_length: i64,
    pub(crate) time_zero_bin: f64,
    pub(crate) first_good_bin: i64,
    pub(crate) last_good_bin: i64,
}

#[derive(Debug)]
pub(crate) struct SampleEnvironmentInfo {
    cryo: String,
}

#[derive(Debug)]
pub(crate) struct MagneticFieldEnvironmentInfo {
    magnet_name: String,
}

#[derive(Debug)]
pub(crate) struct BeamlineInfo {
    name: String,
}
