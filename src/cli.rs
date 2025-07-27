use crate::error::RspError;
use crate::peeler::Peeler;
use clap::{Arg, ArgMatches, Command};

pub struct Cli;

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> Result<(), RspError> {
        let matches = self.build_cli().get_matches();

        match matches.subcommand() {
            Some(("peel", sub_matches)) => self.handle_peel_command(sub_matches),
            _ => {
                eprintln!("No command provided. Use --help for available commands.");
                Ok(())
            }
        }
    }

    fn build_cli(&self) -> Command {
        Command::new("rsp")
            .about("Raw String Peeler - Convert escaped strings in YAML to readable format")
            .version(env!("CARGO_PKG_VERSION"))
            .subcommand(
                Command::new("peel")
                    .about("Peel raw strings from YAML files")
                    .arg(
                        Arg::new("file")
                            .help("The YAML file to process (use stdin if not provided)")
                            .required(false)
                            .value_name("FILE"),
                    )
                    .arg(
                        Arg::new("output")
                            .short('o')
                            .long("output")
                            .help("Output file (default: stdout)")
                            .value_name("OUTPUT_FILE"),
                    ),
            )
    }

    fn handle_peel_command(&self, matches: &ArgMatches) -> Result<(), RspError> {
        let output_file = matches.get_one::<String>("output");
        let peeler = Peeler::new();

        match matches.get_one::<String>("file") {
            Some(input_file) => peeler.peel_file(input_file, output_file),
            None => peeler.peel_stdin(output_file),
        }
    }
}
