
mod cli;
mod error;
mod peeler;

use cli::Cli;
use error::RspError;

fn main() -> Result<(), RspError> {
    let cli = Cli::new();
    cli.run()
}
