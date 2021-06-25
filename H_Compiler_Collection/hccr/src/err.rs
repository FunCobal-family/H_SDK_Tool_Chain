use std::io;

#[derive(Debug)]
pub enum CliError {
    IoError(io::Error),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::IoError(err)
    }
}
