use async_trait::async_trait;
use nusb::{
    list_devices,
    transfer::{Buffer, Bulk, Out},
    DeviceInfo,
};

use crate::{
    error::ApplicationError,
    instruments::{
        instrument::{Command, Instrument},
        measurement::Measurement,
    },
};

const BULK_OUT_ADDRESS: u8 = 0x02;

/**
 * Module for the Peaktech4055mv instrument using USB.
 */
pub struct Peaktech4055mv {
    // HID Device instance
    device: DeviceInfo,
}

impl Peaktech4055mv {
    /**
     * Creates a new instance of Peaktech4055mv with the given USB device path.
     *
     * # Arguments
     * `device` - A string slice representing the path to the USB device.
     *
     * # Returns
     * A new Peaktech4055mv instance.
     */
    pub async fn new(device: &str) -> Result<Self, ApplicationError> {
        let device_path = device.split(":").collect::<Vec<&str>>();
        let vendor_id = device_path
            .first()
            .ok_or_else(|| ApplicationError::UsbError("Missing vendor ID".into()))?
            .parse()
            .map_err(|e| ApplicationError::UsbError(format!("Invalid vendor ID: {}", e)))?;
        let product_id = device_path
            .get(1)
            .ok_or_else(|| ApplicationError::UsbError("Missing product ID".into()))?
            .parse()
            .map_err(|e| ApplicationError::UsbError(format!("Invalid product ID: {}", e)))?;
        let device = list_devices()
            .await
            .map_err(|e| ApplicationError::UsbError(format!("Could not list usb devices: {}", e)))?
            .find(|dev| dev.vendor_id() == vendor_id && dev.product_id() == product_id)
            .ok_or_else(|| ApplicationError::UsbError("Peaktech4055mv device not found".into()))?;
        Ok(Self { device })
    }
}

#[async_trait(?Send)]
impl Instrument for Peaktech4055mv {
    /**
     * Sends a command to the instrument.
     *
     * # Arguments
     * `command` - A Command enum variant representing the command to be sent.
     */
    async fn command(&self, command: Command) -> Result<Option<Measurement>, ApplicationError> {
        let open_device =
            self.device.open().await.map_err(|e| {
                ApplicationError::UsbError(format!("Could not open usb device: {}", e))
            })?;
        let interface = open_device.claim_interface(0).await.map_err(|e| {
            ApplicationError::UsbError(format!("Could not open interface 0: {}", e))
        })?;

        let data = b"Apply:Sin 2 kHz, 5.2 Vpp, -0.2Vdc\n";

        // Get the endpoint and submit transfer
        let mut endpoint = interface
            .endpoint::<Bulk, Out>(BULK_OUT_ADDRESS)
            .map_err(|e| ApplicationError::UsbError(format!("Failed to get endpoint: {}", e)))?;

        let buffer = Buffer::from(data.to_vec());
        endpoint.submit(buffer);

        // Wait for completion
        let completion = endpoint.next_complete().await;

        match completion.status {
            Ok(()) => Ok(None),
            Err(e) => Err(ApplicationError::CommandError(format!(
                "Failed to send command {:?}: {:?}",
                command, e
            ))),
        }
    }

    /**
     * Returns the unique identifier of the instrument.
     *
     * # Returns
     * A String representing the device information.
     */
    fn get_device_info(&self) -> String {
        format!(
            "Peaktech4055mv USB - Vendor ID: {:04x}, Product ID: {:04x}",
            self.device.vendor_id(),
            self.device.product_id()
        )
    }
}
