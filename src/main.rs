mod convert;
mod util;

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        input: Option<String>,
        output: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let mut root_command = Cli::command();

    match cli.command {
        Some(Commands::Convert { input, output }) => {
            let command = root_command.find_subcommand_mut("convert").unwrap();

            let input_path = input.unwrap_or_else(|| {
                command
                    .error(ErrorKind::MissingRequiredArgument, "Input file is required")
                    .exit();
            });
            let output_path = output.unwrap_or_else(|| {
                command
                    .error(
                        ErrorKind::MissingRequiredArgument,
                        "Output file is required",
                    )
                    .exit();
            });

            let input_type = util::get_type_from_path(&input_path).unwrap_or_else(|| {
                command
                    .error(ErrorKind::InvalidValue, "Invalid input file type")
                    .exit();
            });
            let output_type = util::get_type_from_path(&output_path).unwrap_or_else(|| {
                command
                    .error(ErrorKind::InvalidValue, "Invalid output file type")
                    .exit();
            });

            let mut input = util::Input {
                type_: input_type,
                reader: std::fs::File::open(input_path).unwrap_or_else(|_| {
                    command
                        .error(ErrorKind::Io, "Failed to open input file")
                        .exit();
                }),
            };
            let mut output = util::Output {
                type_: output_type,
                writer: std::fs::File::create(output_path).unwrap_or_else(|_| {
                    command
                        .error(ErrorKind::Io, "Failed to create output file")
                        .exit();
                }),
            };

            convert::run(command, &mut input, &mut output);
        }
        None => {}
    }
}
