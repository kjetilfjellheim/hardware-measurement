use async_trait::async_trait;

use crate::{
    arguments::{self, Args},
    error::ApplicationError,
    instruments::{ScpiUsb, Unit161dHid, readers::Reading},
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

}

pub async fn get_device(args: &Args) -> Result<Box<dyn Communication>, ApplicationError> {
    match args.device {
        arguments::Device::Unit161d => get_unit161d(args),
        arguments::Device::GenericScpiUsb => get_scpiusb_device(args).await,
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
 * Initializes and returns a ScpiUsb instrument instance.
 *
 * # Arguments
 * `args` - An Args instance containing command-line arguments.
 *
 * # Returns
 * A Result containing a boxed Instrument instance or an ApplicationError.
 */
async fn get_scpiusb_device(args: &Args) -> Result<Box<dyn Communication>, ApplicationError> {
    let usb = match &args.usb {
        Some(usb_device_id) => usb_device_id,
        None => {
            return Err(ApplicationError::General(
                "ScpiUsb requires usb device".into(),
            ))
        }
    };
    match ScpiUsb::new(usb, args.reader.clone()).await {
        Ok(device) => Ok(Box::new(device)),
        Err(e) => Err(e),
    }
}
