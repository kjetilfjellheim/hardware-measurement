mod arguments;
mod error;
mod instruments;

use crate::{
    error::ApplicationError,
    instruments::{instrument::Instrument, peaktech4055mv::Peaktech4055mv, unit161d::Unit161dHid},
};
use arguments::Args;

/**
 * Main entry point for the hardware measurement application.
 */
#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let args = Args::parse_args();
    let instrument: Box<dyn Instrument> = match args.device {
        arguments::Device::Unit161d => get_unit161d(&args)?,
        arguments::Device::Peaktech4055mv => get_peaktech4055mv(&args).await?,
    };
    let measurement = instrument.command(args.command.into()).await?;
    if let Some(measurement) = measurement {
        println!("{:?}", measurement);
    }
    Ok(())
}

/**
* Initializes and returns a Unit161d instrument instance.
*
* # Arguments
* `args` - An Args instance containing command-line arguments.
*
* # Returns
* A Result containing a boxed Instrument instance or an ApplicationError.
 */
fn get_unit161d(args: &Args) -> Result<Box<dyn Instrument>, ApplicationError> {
    let hid = match &args.hid {
        Some(hid_path) => hid_path,
        None => {
            return Err(ApplicationError::GeneralError(
                "Unit161d requires hid path".into(),
            ))
        }
    };
    match Unit161dHid::new(hid) {
        Ok(device) => Ok(Box::new(device)),
        Err(e) => Err(e),
    }
}

/**
 * Initializes and returns a Peaktech4055mv instrument instance.
 *
 * # Arguments
 * `args` - An Args instance containing command-line arguments.
 *
 * # Returns
 * A Result containing a boxed Instrument instance or an ApplicationError.
 */
async fn get_peaktech4055mv(args: &Args) -> Result<Box<dyn Instrument>, ApplicationError> {
    let usb = match &args.usb {
        Some(usb_path) => usb_path,
        None => {
            return Err(ApplicationError::GeneralError(
                "Peaktech4055mv requires usb path".into(),
            ))
        }
    };
    match Peaktech4055mv::new(usb).await {
        Ok(device) => Ok(Box::new(device)),
        Err(e) => Err(e),
    }
}
