use std::{env::args, path::Path};

use layton_csv_parser as csv;

const FILE_PATH: &str = "data/localization.csv";

fn main()
{
	let mut args = args();
	args.next();

	match args.next()
	{
		Some(to_search) => match csv::find_dialog(Path::new(FILE_PATH), &to_search)
		{
			#[rustfmt::skip]
			Ok(result) => result.iter().for_each(|line|
			{
				line.split('\t')
					.filter(|line| *line != "")
					.for_each(|dialog| println!("{}", dialog));
				println!();
			}),
			Err(err) => eprintln!("[Error] {}", err)
		},
		None => eprintln!("[Error] {}", csv::Error::IncorrectArguments)
	}
}
