use std::{env, fs, path::Path};

use layton_tsv_formatter as tsv;

const FILE_NAME: &str = "localization.csv";

fn main()
{
	let args = env::args().collect::<Vec<String>>();
	let path = args.get(1);

	match tsv::Builder::new(env::args())
	{
		Ok(builder) => match builder.run()
		{
			Ok(result) => print_to_file(Path::new(path.expect("Path already checked")), &result),
			Err(err) => eprintln!("[Error] {}", err)
		},
		Err(err) => eprintln!("[Error] {}", err)
	};
}

fn print_to_file(path: &Path, result: &String)
{
	match fs::write(path.join(FILE_NAME), result)
	{
		Ok(_) => eprintln!("[Info] {}", tsv::Error::Success),
		Err(err) => eprintln!("[Error] {}", err)
	}
	eprintln!("[Info] File location: {}", path.join(FILE_NAME).to_string_lossy());
}
