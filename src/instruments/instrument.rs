use async_trait::async_trait;

use crate::{
    arguments::{self, Args},
    error::ApplicationError,
    instruments::{Peaktech4055mv, Unit161dHid},
};

/**
 * Defines the Instrument trait for hardware measurement instruments.
 */
#[async_trait(?Send)]
pub trait Communication {
    /**
     * Sends commands to the instrument and retrieves measurement data.
     *
     * # Arguments
     * `commands` - A vector of strings representing commands to send to the instrument.
     *
     * # Returns
     * A Result containing an optional boxed Reading instance or an ApplicationError.
     */
    async fn command(
        &self,
        commands: Vec<String>,
    ) -> Result<Option<Box<dyn Reading>>, ApplicationError>;
    /**
     * Returns information about the device.
     *
     * # Returns
     * A String containing device information.
     */
    fn get_device_info(&self) -> String;
}

/**
 * Defines the Reading trait for measurement data returned by instruments.
 */
#[async_trait(?Send)]
pub trait Reading {
    /**
     * Returns the measurement data in CSV format.
     *
     * # Returns
     * A Result containing a String in CSV format or an ApplicationError.
     */
    fn get_csv(&self) -> Result<String, ApplicationError>;
}

pub async fn get_device(args: &Args) -> Result<Box<dyn Communication>, ApplicationError> {
    match args.device {
        arguments::Device::Unit161d => get_unit161d(args),
        arguments::Device::Peaktech4055mv => get_peaktech4055mv(args).await,
    }
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
fn get_unit161d(args: &Args) -> Result<Box<dyn Communication>, ApplicationError> {
    let hid = match &args.hid {
        Some(hid_path) => hid_path,
        None => {
            return Err(ApplicationError::General(
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
async fn get_peaktech4055mv(args: &Args) -> Result<Box<dyn Communication>, ApplicationError> {
    let usb = match &args.usb {
        Some(usb_path) => usb_path,
        None => {
            return Err(ApplicationError::General(
                "Peaktech4055mv requires usb path".into(),
            ))
        }
    };
    match Peaktech4055mv::new(usb).await {
        Ok(device) => Ok(Box::new(device)),
        Err(e) => Err(e),
    }
}
