mod arguments;
mod error;
mod instruments;

use crate::{error::ApplicationError, instruments::instrument::Communication};
use arguments::Args;

/**
 * Main entry point for the hardware measurement application.
 */
#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let args = Args::parse_args();
    let instrument: Box<dyn Communication> = instruments::instrument::get_device(&args).await?;
    let reading = instrument
        .command(args.commands.to_vec())
        .await?;
    if let Some(reading) = reading {
        match args.format.unwrap_or(arguments::Format::Raw) {
            arguments::Format::Csv => println!("{:?}", reading.get_csv()?),
            arguments::Format::Raw => println!("{:?}", reading.get_raw()?),
        }
    }
    Ok(())
}