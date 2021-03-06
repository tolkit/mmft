use thiserror::Error;

// custom error for no STDIN found.
#[derive(Error, Debug)]
pub enum StdinError {
    #[error("[-]\tSTDIN did not contain any sequence(s).")]
    NoSequence,
}

#[derive(Error, Debug)]
pub enum RegexError {
    #[error("[-]\tCould not compile regex. See https://docs.rs/regex/1.5.4/regex/index.html for examples.")]
    CouldNotCompile,
}

#[derive(Error, Debug)]
pub enum FastaWriteError {
    #[error("[-]\tCould not write to file.")]
    CouldNotWrite,
}

#[derive(Error, Debug)]
pub enum UTF8FormatError {
    #[error("[-]\tThe fasta was not UTF8 correct.")]
    NotUTF8,
}

#[derive(Error, Debug)]
pub enum RegionError {
    #[error("[-]\tCould not extract the string from the CLI.")]
    CouldNotUnwrap,
    #[error("[-]\tCould not write to file.")]
    CouldNotParse,
    #[error("[-]\tCould not extract region. Is the range overlapping zero, or larger than the length of the chromosome?")]
    SeqExtractError,
}
