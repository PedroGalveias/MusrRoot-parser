use std::error::Error;

use crate::error::parse_error;
use crate::models::*;

// Entry point
pub(crate) fn run(input: &[u8]) -> Result<MusrRootFile, Box<dyn Error>> {
    match crate::musr_root_file_parser::musr_root_file_parser::parse_musr_root_file_bytes(input) {
        Ok(musr_root_file) => Ok(musr_root_file),
        Err(_err) => Err(Box::new(parse_error(nom::error::ErrorKind::Fail))),
    }
}

mod musr_root_file_parser {
    use nom::bytes::complete::{tag, take_until};
    use nom::combinator::map_res;
    use nom::IResult;
    use nom::multi::{count, many0};
    use nom::number::complete::{be_f64, be_i64};
    use nom::sequence::terminated;
    use crate::models::*;

    // Entry point for parsing the MusrRootFile
    pub(crate) fn parse_musr_root_file_bytes(input: &[u8]) -> Result<MusrRootFile, nom::error::Error<&[u8]>> {
        match parse_musr_root_file(input) {
            Ok((_, musr_root_file)) => Ok(musr_root_file),
            Err(_err) => Err(nom::error::Error::new(input, nom::error::ErrorKind::Fail)),
        }
    }

    fn parse_musr_root_file(input: &[u8]) -> IResult<&[u8], MusrRootFile> {
        let (input, histos) = parse_histos(input)?;
        let (input, run_header) = parse_run_header(input)?;

        Ok((
            input,
            MusrRootFile {
                histos,
                run_header,
            },
        ))
    }


    fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
        map_res(take_until("\0"), |v: &[u8]| {
            std::str::from_utf8(v).map(|s| s.to_string())
        })(input)
    }

    fn parse_detector(input: &[u8]) -> IResult<&[u8], Detector> {
        let (input, name) = parse_string(input)?;
        let (input, histo_number) = be_i64(input)?;
        let (input, histo_length) = be_i64(input)?;
        let (input, time_zero_bin) = be_f64(input)?;
        let (input, first_good_bin) = be_i64(input)?;
        let (input, last_good_bin) = be_i64(input)?;

        Ok((
            input,
            Detector{
                name,
                histo_number,
                histo_length,
                time_zero_bin,
                first_good_bin,
                last_good_bin
            },
        ))
    }

    fn parse_detector_vector(input: &[u8]) -> IResult<&[u8], Vec<Detector>> {
        // Use the `many0` combinator to parse zero or more instances of Detector
        many0(terminated(parse_detector, tag("\0")))(input)
    }

    fn parse_run_info(input: &[u8]) -> IResult<&[u8], RunInfo> {
        let (input, version) = parse_string(input)?;
        let (input, generic_validator_url) = parse_string(input)?;
        let (input, specific_validator_url) = parse_string(input)?;
        let (input, generator) = parse_string(input)?;
        let (input, file_name) = parse_string(input)?;
        let (input, run_title) = parse_string(input)?;
        let (input, run_number) = be_i64(input)?;
        let (input, run_start_time) = parse_string(input)?;
        let (input, run_stop_time) = parse_string(input)?;
        // TODO: run_duration. Implement TMusrRunPhysicalQuantity
        let (input, laboratory) = parse_string(input)?;
        let (input, instrument) = parse_string(input)?;
        // TODO: muon_beam_momentum. Implement TMusrRunPhysicalQuantity
        let (input, muon_species) = parse_string(input)?;
        let (input, muon_source) = parse_string(input)?;
        let (input, setup) = parse_string(input)?;
        let (input, comment) = parse_string(input)?;
        let (input, sample_name) = parse_string(input)?;
        // TODO: sample_temperature. Implement TMusrRunPhysicalQuantity
        // TODO: sample_magnetic_field. Implement TMusrRunPhysicalQuantity
        let (input, no_of_histos) = be_i64(input)?;
        // TODO: time_resolution. Implement TMusrRunPhysicalQuantity
        // TODO: redGreen_offsets. Implement TMusrRunPhysicalQuantity

        Ok((
            input,
            RunInfo {
                version,
                generic_validator_url,
                specific_validator_url,
                generator,
                file_name,
                run_title,
                run_number,
                run_start_time,
                run_stop_time,
                // TODO: run_duration. Implement TMusrRunPhysicalQuantity
                laboratory,
                instrument,
                // TODO: muon_beam_momentum. Implement TMusrRunPhysicalQuantity
                muon_species,
                muon_source,
                setup,
                comment,
                sample_name,
                // TODO: sample_temperature. Implement TMusrRunPhysicalQuantity
                // TODO: sample_magnetic_field. Implement TMusrRunPhysicalQuantity
                no_of_histos,
                // TODO: time_resolution. Implement TMusrRunPhysicalQuantity
                // TODO: redGreen_offsets. Implement TMusrRunPhysicalQuantity
            }
        ))
    }

    fn parse_detector_info(input: &[u8]) -> IResult<&[u8], DetectorInfo> {
        let (input, detectors) = parse_detector_vector(input)?;

        Ok((
            input,
            DetectorInfo {
                detectors
            },
        ))
    }

    fn parse_histos(input: &[u8]) -> IResult<&[u8], Histos> {
        let (input, decay_ana_module) = parse_decay_ana_module(input)?;
        let (input, sc_ana_module) = parse_sc_ana_module(input)?;

        Ok((
            input,
            Histos {
                decay_ana_module,
                sc_ana_module,
            },
        ))
    }

    fn parse_decay_ana_module(input: &[u8]) -> IResult<&[u8], DecayAnaModule> {
        let (input, h_decay) = count(parse_h_decay, 3)(input)?;
        // TODO: Add parsing for other fields of DecayAnaModule
        Ok((
            input,
            DecayAnaModule { h_decay }
        ))
    }

    fn parse_h_decay(input: &[u8]) -> IResult<&[u8], HDecay> {
        let (input, name) = parse_string(input)?;
        let (input, histo_number) = be_i64(input)?;
        let (input, histo_length) = be_i64(input)?;
        let (input, time_zero_bin) = be_f64(input)?;
        let (input, first_good_bin) = be_i64(input)?;
        let (input, last_good_bin) = be_i64(input)?;

        Ok((
            input,
            HDecay {
                name,
                histo_number,
                histo_length,
                time_zero_bin,
                first_good_bin,
                last_good_bin
            }
        ))
    }

    fn parse_sc_ana_module(input: &[u8]) -> IResult<&[u8], SCAnaModule> {
        let (input, temperature) = parse_string(input)?;
        let (input, magnetic_field) = parse_string(input)?;

        Ok((
            input,
            SCAnaModule {
                h_sample_temperature: temperature,
                h_sample_magnetic_field: magnetic_field,
            },
        ))
    }

    fn parse_run_header(input: &[u8]) -> IResult<&[u8], RunHeader> {
        let (input, run_info) = parse_run_info(input)?;
        let (input, detector_info) = parse_detector_info(input)?;
        let (input, sample_environment_info) = parse_sample_environment_info(input)?;
        let (input, magnetic_field_environment_info) = parse_magnetic_field_environment_info(input)?;
        let (input, beamline_info) = parse_beamline_info(input)?;

        Ok((
            input,
            RunHeader {
                run_info,
                detector_info,
                sample_environment_info,
                magnetic_field_environment_info,
                beamline_info
            }
        ))
    }

    fn parse_sample_environment_info(input: &[u8]) -> IResult<&[u8], SampleEnvironmentInfo> {
        todo!()
    }

    fn parse_magnetic_field_environment_info(input: &[u8]) -> IResult<&[u8], MagneticFieldEnvironmentInfo> {
        todo!()
    }

    fn parse_beamline_info(input: &[u8]) -> IResult<&[u8], BeamlineInfo> {
        todo!()
    }
}
