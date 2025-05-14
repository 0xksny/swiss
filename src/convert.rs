use crate::util::{Input, Output, Type};

use clap::error::ErrorKind;

pub fn run<R: std::io::Read, W: std::io::Write>(
    command: &mut clap::Command,
    input: &mut Input<R>,
    output: &mut Output<W>,
) -> Option<()> {
    let input_contents = input.read().unwrap_or_else(|_| {
        command.error(ErrorKind::Io, "Failed to read input").exit();
    });

    let output_contents = match (input.type_.clone(), output.type_.clone()) {
        (Type::JSON, Type::YAML) => serde_json::from_str::<serde_json::Value>(&input_contents)
            .map(|json| serde_yaml::to_string(&json).unwrap())
            .unwrap_or_else(|_| {
                command
                    .error(ErrorKind::InvalidValue, "Failed to convert JSON to YAML")
                    .exit();
            }),
        (Type::YAML, Type::JSON) => serde_yaml::from_str::<serde_yaml::Value>(&input_contents)
            .map(|yaml| serde_json::to_string_pretty(&yaml).unwrap())
            .unwrap_or_else(|_| {
                command
                    .error(ErrorKind::InvalidValue, "Failed to convert YAML to JSON")
                    .exit();
            }),
        _ => {
            command
                .error(ErrorKind::InvalidValue, "Unsupported conversion type")
                .exit();
        }
    };

    output.write(output_contents.trim()).unwrap_or_else(|_| {
        command
            .error(ErrorKind::Io, "Failed to write output file")
            .exit();
    });

    return Some(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_json_to_yaml() {
        let mut input = Input {
            type_: Type::JSON,
            reader: std::io::Cursor::new(r#"{"key":"value"}"#),
        };
        let mut output = Output {
            type_: Type::YAML,
            writer: Vec::new(),
        };

        let mut command = clap::Command::new("test");

        run(&mut command, &mut input, &mut output).unwrap();

        assert_eq!(String::from_utf8(output.writer).unwrap(), r#"key: value"#,);
    }

    // TODO: Write test for YAML to JSON
}
