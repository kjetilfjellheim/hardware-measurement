use async_trait::async_trait;

use crate::{
    arguments, error::ApplicationError, instruments::{
        Command, instrument::Communication, readers::{Reading, ScpiRawReading}
    }
};
use nusb::{
    list_devices,
    transfer::{Buffer, Bulk, Out},
    DeviceInfo,
};

/**
 * USB Bulk OUT endpoint address for ScpiUsb.
 */
const BULK_OUT_ADDRESS: u8 = 0x02;
/**
 * USB Bulk IN endpoint address for ScpiUsb.
 */
const BULK_IN_ADDRESS: u8 = 0x82;
/**
 * Module for the ScpiUsb instrument using USB.
 */
pub struct ScpiUsb {
    // HID Device instance
    device: DeviceInfo,
    // Reader for interpreting instrument responses
    reader: arguments::Reader,
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
    pub async fn new(device: &str, reader: Option<arguments::Reader>) -> Result<Self, ApplicationError> {
        
        let device = list_devices()
            .await
            .map_err(|e| ApplicationError::Usb(format!("Could not list usb devices: {}", e)))?
            .find(|dev| format!("{:x}:{:x}", dev.vendor_id(), dev.product_id()) == device)
            .ok_or_else(|| ApplicationError::Usb("ScpiUsb device not found".into()))?;
        Ok(Self { device, reader:reader.unwrap_or(arguments::Reader::ScpiRawReader) } )
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
    fn get_reading(&self, data: Vec<String>) -> Box<dyn Reading> {
        match self.reader {
            arguments::Reader::ScpiRawReader => Box::new(ScpiRawReading::new(data))
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
    ) -> Result<Option<Box<dyn Reading>>, ApplicationError> {
        let mut response: Option<Vec<String>> = None;

        let parsed_commands: Vec<Box<dyn Command>> = commands
            .iter()
            .map(|cmd_str| {
                let command: Result<Box<dyn Command>, ApplicationError> =
                    cmd_str.clone().try_into();
                command
            })
            .collect::<Result<Vec<Box<dyn Command>>, ApplicationError>>()?;

        let open_device = self
            .device
            .open()
            .await
            .map_err(|e| ApplicationError::Usb(format!("Could not open usb device: {}", e)))?;
        let interface = open_device
            .claim_interface(0)
            .await
            .map_err(|e| ApplicationError::Usb(format!("Could not open interface 0: {}", e)))?;
        // Get the endpoint and submit transfer
        let mut endpoint_out = interface
            .endpoint::<Bulk, Out>(BULK_OUT_ADDRESS)
            .map_err(|e| ApplicationError::Usb(format!("Failed to get endpoint: {}", e)))?;

        let mut endpoint_in = interface
            .endpoint::<Bulk, nusb::transfer::In>(BULK_IN_ADDRESS)
            .map_err(|e| ApplicationError::Usb(format!("Failed to get endpoint: {}", e)))?;

        for command in parsed_commands {
            let command_bytes = command.to_command();

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

            if command.is_query() {
                let read_buffer = Buffer::new(2048);
                endpoint_in.submit(read_buffer);
                let completion = endpoint_in.next_complete().await;

                match completion.status {
                    Ok(()) => {
                        let data = completion.buffer.to_vec();
                        let response_str =
                            String::from_utf8_lossy(&data[..completion.actual_len]).to_string();
                        let resp = response.get_or_insert(Vec::new());
                        resp.push(response_str);
                    }
                    Err(e) => {
                        return Err(ApplicationError::Command(format!(
                            "Failed to read response for command {:?}: {:?}",
                            command, e
                        )))
                    }
                }
            }
        }

        Ok(match response {
            Some(data) => Some(self.get_reading(data)),
            None => None,
        })
    }
}
