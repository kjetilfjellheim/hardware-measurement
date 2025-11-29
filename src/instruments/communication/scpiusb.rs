use async_trait::async_trait;

use crate::{
    arguments,
    error::ApplicationError,
    instruments::{
        communication::common::Communication, reading::{Reading, ScpiRawReading}
    },
};
use nusb::{
    list_devices,
    transfer::{Buffer, Bulk, Out},
    DeviceInfo,
};

/**
 * Module for the ScpiUsb instrument using USB.
 */
pub struct ScpiUsb {
    /** 
    * USB Device Info 
    */
    device: DeviceInfo,
    /**
    * Reader type for interpreting instrument responses.
    */
    reader: arguments::Reader,
    /**
     * USB interface number.
     */
    interface_number: u8,
    /**
     * USB Bulk IN endpoint address.
     */
    bulk_in_address: u8,
    /**
     * USB Bulk OUT endpoint address.
     */
    bulk_out_address: u8,
}

impl ScpiUsb {
    /**
     * Creates a new instance of ScpiUsb with the given USB device path.
     *
     * # Arguments
     * `device` - A string slice representing the path to the USB device.
     *
     * # Returns
     * A new ScpiUsb instance.
     */
    pub async fn new(
        device: &str,
        reader: Option<arguments::Reader>,
        interface_number: u8,
        bulk_in_address: u8,
        bulk_out_address: u8,
    ) -> Result<Self, ApplicationError> {
        let device = list_devices()
            .await
            .map_err(|e| ApplicationError::Usb(format!("Could not list usb devices: {}", e)))?
            .find(|dev| format!("{:x}:{:x}", dev.vendor_id(), dev.product_id()) == device)
            .ok_or_else(|| ApplicationError::Usb("ScpiUsb device not found".into()))?;
        Ok(Self {
            device,
            reader: reader.unwrap_or(arguments::Reader::ScpiRawReader),
            interface_number,
            bulk_in_address,
            bulk_out_address,
        })
    }

    /**
     * Creates a Reading instance based on the configured reader type.
     *
     * # Arguments
     * `data` - A vector of strings representing the instrument response data.
     *
     * # Returns
     * A boxed Reading instance.
     */
    fn get_reading(&self, data: Vec<u8>) -> Box<dyn Reading> {
        match self.reader {
            arguments::Reader::ScpiRawReader => Box::new(ScpiRawReading::new(data)),
        }
    }
}

#[async_trait(?Send)]
impl Communication for ScpiUsb {
    /**
     * Sends a command to the instrument.
     *
     * # Arguments
     * `command` - A Command enum variant representing the command to be sent.
     */
    async fn command(
        &self,
        commands: Vec<String>,
    ) -> Result<Option<Vec<Box<dyn Reading>>>, ApplicationError> {
        let open_device = self
            .device
            .open()
            .await
            .map_err(|e| ApplicationError::Usb(format!("Could not open usb device: {}", e)))?;
        // Claim the interface
        let interface = open_device
            .claim_interface(self.interface_number)
            .await
            .map_err(|e| ApplicationError::Usb(format!("Could not open interface {}: {}", self.interface_number, e)))?;
        // Get the endpoint and submit transfer
        let mut endpoint_out = interface
            .endpoint::<Bulk, Out>(self.bulk_out_address)
            .map_err(|e| ApplicationError::Usb(format!("Failed to get endpoint {}: {}", self.bulk_out_address, e)))?;

        let mut endpoint_in = interface
            .endpoint::<Bulk, nusb::transfer::In>(self.bulk_in_address)
            .map_err(|e| ApplicationError::Usb(format!("Failed to get endpoint {}: {}", self.bulk_in_address, e)))?;

        let mut response: Vec<Box<dyn Reading>> = Vec::new();

        for command in commands {

            let command_bytes = if command.clone().ends_with('\n') {
                command.clone().into_bytes()
            } else {
                let mut cmd_bytes = command.clone().into_bytes();
                cmd_bytes.push(b'\n');
                cmd_bytes
            };

            let buffer = Buffer::from(command_bytes);

            endpoint_out.submit(buffer);
            let completion = endpoint_out.next_complete().await;

            match completion.status {
                Ok(()) => {}
                Err(e) => {
                    return Err(ApplicationError::Command(format!(
                        "Failed to send command {:?}: {:?}",
                        command, e
                    )))
                }
            }

            let data_as_vec: Option<Vec<u8>> = if command.contains('?') {
                let read_buffer = Buffer::new(2000000);
                endpoint_in.submit(read_buffer);
                let completion = endpoint_in.next_complete().await;

                match completion.status {
                    Ok(()) => {
                        let data = completion.buffer.to_vec();
                        Some(data)
                    }
                    Err(e) => {
                        return Err(ApplicationError::Command(format!(
                            "Failed to read response for command {:?}: {:?}",
                            command, e
                        )))
                    }
                }
            } else {
                None
            };

            if let Some(data) = data_as_vec {
                response.push(self.get_reading(data));
            }
        }

        Ok(match response.is_empty() {
            false => Some(response),
            true => None,
        })
    }
}
