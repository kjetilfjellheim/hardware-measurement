use async_trait::async_trait;

use crate::{arguments::{Args, Device}, error::ApplicationError, instruments::{communication::{scpiusb::ScpiUsb, unit161d::Unit161dHid}, reading::{Reading}}};

const DEFAULT_USB_INTERFACE_NUM: u8 = 0;
const DEFAULT_USB_BULK_IN_ADDRESS: u8 = 0x81;
const DEFAULT_USB_BULK_OUT_ADDRESS: u8 = 0x01;

const PEAKTECH_4055MV_USB_INTERFACE_NUM: u8 = 0;
const PEAKTECH_4055MV_USB_BULK_IN_ADDRESS: u8 = 0x82;
const PEAKTECH_4055MV_USB_BULK_OUT_ADDRESS: u8 = 0x02;

#[async_trait(?Send)]
pub trait Communication {
    /**
     * Sends a command to the instrument.
     *
     * # Arguments
     * `command` - A Command enum variant representing the command to be sent.
     * 
     * # Returns
     * A Result containing an optional vector of Reading trait objects or an ApplicationError.
     */
    async fn command(
        &self,
        commands: Vec<String>,
    ) -> Result<Option<Vec<Box<dyn Reading>>>, ApplicationError>;
}

/**
 * Factory function to create a Communication device based on the provided arguments.
 *
 * # Arguments
 * `args` - An Args struct containing the configuration for the desired device.
 *
 * # Returns
 * A Result containing a boxed Communication trait object or an ApplicationError.
 */
pub async fn get_communication_device(args: &Args) -> Result<Box<dyn Communication>, ApplicationError> {
    match args.device {
        Device::Unit161d => {
            let hid = args.hid.as_ref().ok_or_else(|| ApplicationError::Hid("HID device not provided".into()))?;
            let hid_device = Unit161dHid::new(hid)?;
            Ok(Box::new(hid_device))            
        }
        Device::GenericScpiUsb => {
            let usb = args.usb.as_ref().ok_or_else(|| ApplicationError::Usb("USB device not provided".into()))?;
            let scpi_usb_device = ScpiUsb::new(usb, args.clone().reader, args.interface_number.unwrap_or(DEFAULT_USB_INTERFACE_NUM), args.bulk_in_address.unwrap_or(DEFAULT_USB_BULK_IN_ADDRESS), args.bulk_out_address.unwrap_or(DEFAULT_USB_BULK_OUT_ADDRESS)).await?;
            Ok(Box::new(scpi_usb_device))
        }
        Device::Peaktech4055mvUsb => {
            let usb = args.usb.as_ref().ok_or_else(|| ApplicationError::Usb("USB device not provided".into()))?;
            let scpi_usb_device = ScpiUsb::new(usb, args.clone().reader, args.interface_number.unwrap_or(PEAKTECH_4055MV_USB_INTERFACE_NUM), args.bulk_in_address.unwrap_or(PEAKTECH_4055MV_USB_BULK_IN_ADDRESS), PEAKTECH_4055MV_USB_BULK_OUT_ADDRESS).await?;
            Ok(Box::new(scpi_usb_device))
        }
    }
}