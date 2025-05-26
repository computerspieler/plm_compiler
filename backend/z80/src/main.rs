use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process::exit;

mod parser;

fn show_help_and_die() {
	println!(concat!(
		"./zasm [ARGUMENTS] [INPUT FILES]\n",
		"-h: Show this message\n",
		"-l: List all the supported instructions\n",
		"-o [FILE]: Set the output file",
		"-D NAME VALUE: Define a global variable",
	));
	exit(0);
}

#[derive(Default)]
struct ParsedArguments {
	input_files_path: Vec<String>,
	output_path: Option<String>,
	definitions: HashMap<String, i32>,
}

fn parse_arguments() -> ParsedArguments {
    let mut output = ParsedArguments::default();
    let mut args = env::args();
    
    args.next(); // Skip the first one, it's the executable's path
	while let Some(arg) = args.next() {
		match arg.as_str() {
			| "-h" => {
				show_help_and_die();
			}
			| "-o" => match args.next() {
				| None => {
					panic!("No path has been provided with '-o'");
				}
				| Some(path) => {
					if let Some(_) = output.output_path {
						panic!("The output path has been specified multiple times");
					}
					output.output_path = Some(path);
				}
			},
			| "-l" => {
				z80::instruction::list_instructions();
				exit(0);
			}
			| "-D" => {
				let first_arg = args.next();
				let second_arg = args.next();
				match (first_arg, second_arg) {
					| (Some(_), None) | (None, _) => {
						panic!("Not enough arguments for -D");
					}
					| (Some(name), Some(val)) => match i32::from_str_radix(val.as_str(), 10) {
						| Err(_) => {
							panic!("Invalid value for {}: {}", name, val);
						}
						| Ok(val) => {
							output.definitions.insert(name.to_ascii_uppercase(), val);
						}
					},
				}
			}

			| _ => {
				output.input_files_path.push(arg);
			}
		}
	}

    output
}

fn main() {
    let args = parse_arguments();
    
	if args.input_files_path.len() == 0 {
		show_help_and_die();
	}

	for path in args.input_files_path {
		match File::options()
			.read(true)
			.write(false)
			.open(&path)
		{
			| Err(e) => {
				panic!("Unable to open {}: {}", path, e);
			}
			| Ok(file) => {
                let reader = BufReader::new(file);

			}
		}
	}
}
