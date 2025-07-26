use clap::{Arg, Command, ArgMatches};
use crate::error::RspError;
use crate::peeler::Peeler;

pub struct Cli;

impl Cli {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> Result<(), RspError> {
        let matches = self.build_cli().get_matches();
        
        match matches.subcommand() {
            Some(("peel", sub_matches)) => {
                self.handle_peel_command(sub_matches)
            }
            _ => {
                eprintln!("No command provided. Use --help for available commands.");
                Ok(())
            }
        }
    }

    fn build_cli(&self) -> Command {
        Command::new("rsp")
            .about("Raw String Peeler - Convert escaped strings in YAML to readable format")
            .version("0.1.0")
            .subcommand(
                Command::new("peel")
                    .about("Peel raw strings from YAML files")
                    .arg(
                        Arg::new("file")
                            .help("The YAML file to process")
                            .required(true)
                            .value_name("FILE")
                    )
                    .arg(
                        Arg::new("output")
                            .short('o')
                            .long("output")
                            .help("Output file (default: stdout)")
                            .value_name("OUTPUT_FILE")
                    )
            )
    }

    fn handle_peel_command(&self, matches: &ArgMatches) -> Result<(), RspError> {
        let input_file = matches.get_one::<String>("file")
            .ok_or_else(|| RspError::Processing("Input file is required".to_string()))?;
        
        let output_file = matches.get_one::<String>("output");
        
        let peeler = Peeler::new();
        peeler.peel_file(input_file, output_file)
    }
}