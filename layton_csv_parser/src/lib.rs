#![forbid(unsafe_code, unstable_features)]

use std::{fmt, fs, path::Path};

pub fn find_dialog(path: &Path, key: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>
{
	let length = key.len() - 1;
	let lines = fs::read_to_string(path)?
		.lines()
		.filter(|line| line[0..=length] == *key)
		.map(|line| String::from(line))
		.collect::<Vec<String>>();

	match lines.is_empty()
	{
		false => Ok(lines),
		true => Err(Box::new(Error::DialogNotFound))
	}
}

#[derive(Debug)]
pub enum Error
{
	IncorrectArguments,
	DialogNotFound
}

impl std::error::Error for Error {}

impl fmt::Display for Error
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			Error::IncorrectArguments => write!(f, "Incorrect number of arguments!"),
			Error::DialogNotFound => write!(f, "The dialog was not found")
		}
	}
}
