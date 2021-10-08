use crate::utils::{error, parse::parse_region, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use std::io;
use std::path::Path;

pub fn extract_region(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = matches.values_of("fasta");
    let region = matches.value_of("region");

    let region = match region {
        Some(r) => r,
        None => bail!(error::RegionError::CouldNotUnwrap),
    };

    let parsed_region = parse_region(region)?;

    // writer here?
    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f {
                let basename = Path::new(el).file_name().unwrap().to_str().unwrap();

                let reader = fasta::Reader::from_file(el).expect("[-]\tPath invalid.");
                for record in reader.records() {
                    let record = record.expect("[-]\tError during fasta record parsing.");
                    let id = record.id();
                    let seq = record.seq().get(parsed_region[0]..parsed_region[1]);

                    let seq_res = match seq {
                        Some(s) => s,
                        None => bail!(error::RegionError::SeqExtractError),
                    };
                    // write to stdout
                    let description =
                        format!("{}: {}-{}", basename, parsed_region[0], parsed_region[1]);
                    writer
                        .write(id, Some(&description), seq_res)
                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut records = fasta::Reader::new(io::stdin()).records();
                while let Some(Ok(record)) = records.next() {
                    let id = record.id();
                    let seq = record.seq().get(parsed_region[0]..parsed_region[1]);

                    let seq_res = match seq {
                        Some(s) => s,
                        None => bail!(error::RegionError::SeqExtractError),
                    };
                    // write to stdout
                    let description = format!("{}-{}", parsed_region[0], parsed_region[1]);
                    writer
                        .write(id, Some(&description), seq_res)
                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                }
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}
