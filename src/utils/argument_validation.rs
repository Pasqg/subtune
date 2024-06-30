use std::str::FromStr;
use crate::utils::file_extension;
use crate::utils::visualization::{ColorScheme, ResamplingStrategy};

pub(crate) fn validate_arguments(input_file: &str,
                                 output_file: &str,
                                 resampling_strategy: &str,
                                 color_scheme: &str) -> Result<(), String> {
    valid_input_extension(input_file)?;
    valid_output_extension(output_file)?;

    let is_resampling_strategy_valid = ResamplingStrategy::from_str(resampling_strategy);
    if is_resampling_strategy_valid.is_err() {
        return Err(is_resampling_strategy_valid.err().unwrap());
    }

    let is_color_scheme_valid = ColorScheme::from_str(color_scheme);
    if is_color_scheme_valid.is_err() {
        return Err(is_color_scheme_valid.err().unwrap());
    }

    Ok(())
}

fn valid_input_extension(input_file: &str) -> Result<(), String> {
    let extension = file_extension(input_file);
    match extension {
        None => Err("Only .wav format is supported, but input file has no extension!".to_string()),
        Some("wav") => Ok(()),
        Some(extension) => Err(format!("Only .wav format is supported, but input format is .{}!", extension)),
    }
}

fn valid_output_extension(output_file: &str) -> Result<(), String> {
    let extension = file_extension(output_file);
    match extension {
        None => Err("Only .png format is supported for output, but output file has no extension!".to_string()),
        Some("png") => Ok(()),
        Some(extension) => Err(format!("Only .png format is supported for output, but output format is .{}!", extension)),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::argument_validation::validate_arguments;

    #[test]
    fn both_input_and_output_are_valid() {
        assert_eq!(validate_arguments("input.wav", "output.png", "max", "heatmap"), Ok(()));
    }

    #[test]
    fn input_has_wrong_extension() {
        assert_eq!(validate_arguments("input.wav2", "output.png", "max", "heatmap"),
                   Err("Only .wav format is supported, but input format is .wav2!".to_string()));
    }

    #[test]
    fn input_has_no_extension() {
        assert_eq!(validate_arguments("input", "output.png", "max", "heatmap"),
                   Err("Only .wav format is supported, but input file has no extension!".to_string()));
    }

    #[test]
    fn output_has_wrong_extension() {
        assert_eq!(validate_arguments("input.wav", "output", "max", "heatmap"),
                   Err("Only .png format is supported for output, but output file has no extension!".to_string()));
    }
}