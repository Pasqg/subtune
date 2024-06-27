use std::str::FromStr;
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
    let split: Vec<&str> = input_file.split('.').collect();
    if split.len() == 1 {
        return Err("Only .wav format is supported, but input file has no extension!".to_string());
    }
    if split.len() > 1 {
        let extension = split[split.len() - 1];
        if extension != "wav" {
            return Err(format!("Only .wav format is supported, but input format is .{}!", extension));
        }
    }
    Ok(())
}

fn valid_output_extension(output_file: &str) -> Result<(), String> {
    let split: Vec<&str> = output_file.split('.').collect();
    if split.len() == 1 {
        return Err("Only .png format is supported for output, but output file has no extension!".to_string());
    }
    if split.len() > 1 {
        let extension = split[split.len() - 1];
        if extension != "png" {
            return Err(format!("Only .png format is supported for output, but output format is .{}!", extension));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::utils::argument_validation::validate_arguments;

    #[test]
    fn both_input_and_output_are_valid() {
        assert_eq!(validate_arguments("input.wav", "output.png"), Ok(()));
    }

    #[test]
    fn input_has_wrong_extension() {
        assert_eq!(validate_arguments("input.wav2", "output.png"),
                   Err("Only .wav format is supported, but input format is .wav2!".to_string()));
    }

    #[test]
    fn input_has_no_extension() {
        assert_eq!(validate_arguments("input", "output.png"),
                   Err("Only .wav format is supported, but input file has no extension!".to_string()));
    }

    #[test]
    fn output_has_wrong_extension() {
        assert_eq!(validate_arguments("input.wav", "output"),
                   Err("Only .png format is supported for output, but output file has no extension!".to_string()));
    }
}