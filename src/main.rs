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

    match cli.command {
        Some(Commands::Convert { input, output }) => {
            let input_path = input.unwrap_or_else(|| {
                Cli::command()
                    .error(ErrorKind::MissingRequiredArgument, "Input file is required")
                    .exit();
            });

            let output_path = output.unwrap_or_else(|| {
                Cli::command()
                    .error(
                        ErrorKind::MissingRequiredArgument,
                        "Output file is required",
                    )
                    .exit();
            });

            let input_type = util::get_type_from_path(&input_path).unwrap_or_else(|| {
                Cli::command()
                    .error(ErrorKind::InvalidValue, "Invalid input file type")
                    .exit();
            });
            let input_contents = std::fs::read_to_string(&input_path).unwrap_or_else(|_| {
                Cli::command()
                    .error(ErrorKind::Io, "Failed to read input file")
                    .exit();
            });

            let output_type = util::get_type_from_path(&output_path).unwrap_or_else(|| {
                Cli::command()
                    .error(ErrorKind::InvalidValue, "Invalid output file type")
                    .exit();
            });

            let output_contents = match (input_type, output_type) {
                (util::Type::JSON, util::Type::YAML) => {
                    serde_json::from_str::<serde_json::Value>(&input_contents)
                        .map(|json| serde_yaml::to_string(&json).unwrap())
                        .unwrap_or_else(|_| {
                            Cli::command()
                                .error(ErrorKind::InvalidValue, "Failed to convert JSON to YAML")
                                .exit();
                        })
                }
                (util::Type::YAML, util::Type::JSON) => {
                    serde_yaml::from_str::<serde_yaml::Value>(&input_contents)
                        .map(|yaml| serde_json::to_string(&yaml).unwrap())
                        .unwrap_or_else(|_| {
                            Cli::command()
                                .error(ErrorKind::InvalidValue, "Failed to convert YAML to JSON")
                                .exit();
                        })
                }
                _ => {
                    Cli::command()
                        .error(ErrorKind::InvalidValue, "Unsupported conversion type")
                        .exit();
                }
            };

            std::fs::write(&output_path, output_contents).unwrap_or_else(|_| {
                Cli::command()
                    .error(ErrorKind::Io, "Failed to write output file")
                    .exit();
            });
        }
        None => {}
    }
}
