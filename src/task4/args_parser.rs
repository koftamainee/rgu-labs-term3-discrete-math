use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct CLIArgs {
    pub help: bool,
    pub input_format: Option<InputFormat>,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub start_vertex: Option<usize>,
    pub end_vertex: Option<usize>,
}

#[derive(Debug)]
pub enum InputFormat {
    Edges,
    Matrix,
    List,
}

#[derive(Debug, Error)]
pub enum ArgError {
    #[error("Input file was not provided")]
    MissingInputFile,

    #[error("More than one input format specified (-e / -m / -l)")]
    MultipleInputFormats,

    #[error("No input format key specified (expected -e, -m, or -l)")]
    MissingFormatKey,

    #[error("Missing value for flag {0}")]
    MissingFormatPath(String),

    #[error("Invalid numeric value: {0}")]
    InvalidNumber(String),

    #[error("Unknown flag: {0}")]
    UnknownFlag(String),
}

impl From<ParseIntError> for ArgError {
    fn from(_: ParseIntError) -> Self {
        ArgError::InvalidNumber("Failed to parse number".into())
    }
}

pub fn parse_args(args: &[String]) -> Result<CLIArgs, ArgError> {
    let mut cfg = CLIArgs::default();

    if args.is_empty() {
        return Err(ArgError::MissingFormatKey);
    }

    let mut i = 0;
    while i < args.len() {
        let flag = &args[i];
        i += 1;

        match flag.as_str() {
            "-h" => {
                cfg.help = true;
                return Ok(cfg);
            }
            "-e" | "-m" | "-l" => {
                if cfg.input_format.is_some() {
                    return Err(ArgError::MultipleInputFormats);
                }

                if i >= args.len() {
                    return Err(ArgError::MissingFormatPath(flag.clone()));
                }

                let format = match flag.as_str() {
                    "-e" => InputFormat::Edges,
                    "-m" => InputFormat::Matrix,
                    "-l" => InputFormat::List,
                    _ => unreachable!(),
                };

                cfg.input_format = Some(format);
                cfg.input_file = Some(args[i].clone());
                i += 1;
            }
            "-o" => {
                if i >= args.len() {
                    return Err(ArgError::MissingFormatPath(flag.clone()));
                }
                cfg.output_file = Some(args[i].clone());
                i += 1;
            }
            "-n" => {
                if i >= args.len() {
                    return Err(ArgError::MissingFormatPath(flag.clone()));
                }
                cfg.start_vertex = Some(args[i].parse()?);
                i += 1;
            }

            "-d" => {
                if i >= args.len() {
                    return Err(ArgError::MissingFormatPath(flag.clone()));
                }
                cfg.end_vertex = Some(args[i].parse()?);
                i += 1;
            }
            other => return Err(ArgError::UnknownFlag(other.to_string())),
        }
    }

    if cfg.input_format.is_none() {
        return Err(ArgError::MissingFormatKey);
    }

    if cfg.input_file.is_none() {
        return Err(ArgError::MissingInputFile);
    }

    Ok(cfg)
}
