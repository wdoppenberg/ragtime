use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Generic error: {0}")]
    Generic(&'static str),
	
	#[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

pub type AppResult<T> = Result<T, Error>;