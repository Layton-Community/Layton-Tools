#![forbid(unsafe_code, unstable_features)]

use std::{env, ffi::OsStr, fs, fmt};

const KEYS: &str = "keys	en	fr	de	es";

pub struct Builder
{
	dir: fs::ReadDir
}

impl Builder
{
	pub fn new(mut args: env::Args) -> Result<Builder, Error>
	{
		// skip executable name
		args.next();

		let file_path = match args.next()
		{
			Some(arg) => arg,
			None => return Err(Error::IncorrectArguments)
		};

		match fs::read_dir(file_path)
		{
			Ok(dir) =>
			{
				eprintln!("[Info] Found folder.");

				Ok(Builder { dir })
			}
			Err(_) => Err(Error::FolderNotFound)
		}
	}

	pub fn run(self) -> Result<String, Box<dyn std::error::Error>>
	{
		let mut lines: Vec<String> = vec![KEYS.to_string()];

		for files in self.dir
		{
			let file = files?;
			let path = file.path();

			if path.extension() == Some(OsStr::new("tsv"))
			{
				#[rustfmt::skip]
				let data: Vec<String> = fs::read_to_string(path)?
					.lines()
					.filter_map(|str| (str != KEYS).then_some(String::from(str)))
					.collect();

				lines.append(&mut data.into());
			}
			else
			{
				return Err(Box::new(Error::FileExtention));
			}
		}

		Ok(lines.join("\n"))
	}
}

#[derive(Debug)]
pub enum Error
{
	IncorrectArguments,
	FolderNotFound,
	FileExtention,
	Success
}

impl std::error::Error for Error {}

impl fmt::Display for Error
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			Error::IncorrectArguments => write!(f, "Incorrect number of arguments!"),
			Error::FolderNotFound => write!(f, "The folder does not exist!"),
			Error::FileExtention => write!(f, "Only .tsv files are allowed!"),
			Error::Success => write!(f, "Success!")
		}
	}
}
