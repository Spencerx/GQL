use crate::printer::OutputFormatKind;

/// Arguments for GitQL
#[derive(Debug, PartialEq)]
pub struct Arguments {
    pub repos: Vec<String>,
    pub analysis: bool,
    pub pagination: bool,
    pub page_size: usize,
    pub enable_line_editor: bool,
    pub output_format: OutputFormatKind,
}

/// Create a new instance of Arguments with the default settings
impl Arguments {
    fn new() -> Arguments {
        Arguments {
            repos: vec![],
            analysis: false,
            pagination: false,
            page_size: 10,
            enable_line_editor: false,
            output_format: OutputFormatKind::Table,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    ReplMode(Arguments),
    QueryMode(String, Arguments),
    ScriptMode(String, Arguments),
    Help,
    Version,
    Error(String),
}

pub fn parse_arguments(args: &[String]) -> Command {
    let args_len = args.len();

    if args.iter().any(|i| i == "--help" || i == "-h") {
        return Command::Help;
    }

    if args.iter().any(|i| i == "--version" || i == "-v") {
        return Command::Version;
    }

    let mut optional_query: Option<String> = None;
    let mut optional_script_file: Option<String> = None;
    let mut arguments = Arguments::new();

    let mut arg_index = 1;
    loop {
        if arg_index >= args_len {
            break;
        }

        let arg = &args[arg_index];
        if !arg.starts_with('-') {
            return Command::Error(format!("Unknown argument {arg}"));
        }

        match arg.as_ref() {
            "--repos" | "-r" => {
                arg_index += 1;
                if arg_index >= args_len {
                    let message = format!("Argument {arg} must be followed by one or more path");
                    return Command::Error(message);
                }

                loop {
                    if arg_index >= args_len {
                        break;
                    }

                    let repo = &args[arg_index];
                    if !repo.starts_with('-') {
                        arguments.repos.push(repo.to_string());
                        arg_index += 1;
                        continue;
                    }

                    break;
                }
            }
            "--query" | "-q" => {
                arg_index += 1;
                if arg_index >= args_len {
                    let message = format!("Argument {arg} must be followed by the query");
                    return Command::Error(message);
                }

                optional_query = Some(args[arg_index].to_string());
                arg_index += 1;
            }
            "--script" | "-s" => {
                arg_index += 1;
                if arg_index >= args_len {
                    let message = format!("Argument {arg} must be followed by the file");
                    return Command::Error(message);
                }

                optional_script_file = Some(args[arg_index].to_string());
                arg_index += 1;
            }
            "--analysis" | "-a" => {
                arguments.analysis = true;
                arg_index += 1;
            }
            "--pagination" | "-p" => {
                arguments.pagination = true;
                arg_index += 1;
            }
            "--pagesize" | "-ps" => {
                arg_index += 1;
                if arg_index >= args_len {
                    let message = format!("Argument {arg} must be followed by the page size");
                    return Command::Error(message);
                }

                let page_size_result = args[arg_index].parse::<usize>();
                if page_size_result.is_err() {
                    return Command::Error("Invalid page size".to_string());
                }

                let page_size = page_size_result.ok().unwrap();
                arguments.page_size = page_size;
                arg_index += 1;
            }
            "--editor" | "-e" => {
                arguments.enable_line_editor = true;
                arg_index += 1;
            }
            "--output" | "-o" => {
                arg_index += 1;
                if arg_index >= args_len {
                    let message = format!("Argument {arg} must be followed by output format");
                    return Command::Error(message);
                }

                let output_type = &args[arg_index].to_lowercase();
                if output_type == "csv" {
                    arguments.output_format = OutputFormatKind::CSV;
                } else if output_type == "json" {
                    arguments.output_format = OutputFormatKind::JSON;
                } else if output_type == "table" || output_type == "render" {
                    arguments.output_format = OutputFormatKind::Table;
                } else if output_type == "yaml" {
                    arguments.output_format = OutputFormatKind::YAML;
                } else {
                    return Command::Error("Invalid output format".to_string());
                }

                arg_index += 1;
            }
            _ => return Command::Error(format!("Unknown command {arg}")),
        }
    }

    // Add the current directory if no repository is passed
    if arguments.repos.is_empty() {
        let current_dir = std::env::current_dir();
        if current_dir.is_ok() {
            arguments.repos.push(
                current_dir
                    .ok()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap_or(".")
                    .to_string(),
            );
        } else {
            return Command::Error("Missing repositories paths".to_string());
        }
    }

    if let Some(script_file) = optional_script_file {
        Command::ScriptMode(script_file, arguments)
    } else if let Some(query) = optional_query {
        Command::QueryMode(query, arguments)
    } else {
        Command::ReplMode(arguments)
    }
}

pub fn print_help_list() {
    println!("GitQL is a SQL like query language to run on local repositories");
    println!();
    println!("Usage: gitql [OPTIONS]");
    println!();
    println!("Options:");
    println!("-r,  --repos <REPOS>        Path for local repositories to run query on");
    println!("-s,  --script <file>        Script file contains one or more query");
    println!("-q,  --query <GitQL Query>  GitQL query to run on selected repositories");
    println!("-p,  --pagination           Enable print result with pagination");
    println!("-ps, --pagesize             Set pagination page size [default: 10]");
    println!("-o,  --output               Set output format [render, json, csv]");
    println!("-a,  --analysis             Print Query analysis");
    println!("-e,  --editor               Enable GitQL Rich Line Editor");
    println!("-h,  --help                 Print GitQL help");
    println!("-v,  --version              Print GitQL Current Version");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_arguments() {
        let arguments = vec!["gitql".to_string()];
        let command = parse_arguments(&arguments);
        assert!(matches!(command, Command::ReplMode { .. }));
    }

    #[test]
    fn test_repl_arguments() {
        let arguments = vec!["gitql".to_string(), "--repos".to_string(), ".".to_string()];
        let command = parse_arguments(&arguments);
        assert!(matches!(command, Command::ReplMode { .. }));
    }

    #[test]
    fn test_query_arguments() {
        let arguments = vec![
            "gitql".to_string(),
            "-q".to_string(),
            "Select * from table".to_string(),
        ];
        let command = parse_arguments(&arguments);
        assert!(matches!(command, Command::QueryMode { .. }));
    }

    #[test]
    fn test_arguments_with_help() {
        let arguments = vec![
            "gitql".to_string(),
            "dummy".to_string(),
            "--help".to_string(),
        ];
        let command = parse_arguments(&arguments);
        assert_eq!(command, Command::Help);
    }

    #[test]
    fn test_arguments_with_version() {
        let arguments = vec![
            "gitql".to_string(),
            "dummy".to_string(),
            "--version".to_string(),
        ];
        let command = parse_arguments(&arguments);
        assert_eq!(command, Command::Version);
    }

    #[test]
    fn test_arguments_with_valid_page_size() {
        let arguments = vec![
            "gitql".to_string(),
            "--pagesize".to_string(),
            "10".to_string(),
        ];
        let command = parse_arguments(&arguments);
        assert!(!matches!(command, Command::Error { .. }));
    }

    #[test]
    fn test_arguments_with_invalid_page_size() {
        let arguments = vec![
            "gitql".to_string(),
            "--pagesize".to_string(),
            "-".to_string(),
        ];
        let command = parse_arguments(&arguments);
        assert!(matches!(command, Command::Error { .. }));
    }

    #[test]
    fn test_arguments_with_valid_output_format() {
        let arguments = vec![
            "gitql".to_string(),
            "--output".to_string(),
            "csv".to_string(),
        ];
        let command = parse_arguments(&arguments);
        assert!(!matches!(command, Command::Error { .. }));
    }

    #[test]
    fn test_arguments_with_invalid_output_format() {
        let arguments = vec![
            "gitql".to_string(),
            "--output".to_string(),
            "text".to_string(),
        ];
        let command = parse_arguments(&arguments);
        assert!(matches!(command, Command::Error { .. }));
    }
}
