use std::ffi::CString;

use async_trait::async_trait;

use crate::{
    error::ApplicationError,
    instruments::{
        command::Uni161dCommand, communication::common::Communication, reading::{Reading, Unit161dReading}
    },
};

/**
 * Sequence to send a command to the Uni-T 161D instrument.
 */
const SEQUENCE_SEND_CMD: [u8; 3] = [0xAB, 0xCD, 0x03];

/**
 * Module for the Unit161d instrument using HID API.
 */
pub struct Unit161dHid {
    // HID Device instance
    hiddevice: hidapi::HidDevice,
}

impl Unit161dHid {
    /**
     * Creates a new instance of Unit161dHid with the given HID API.
     *
     * # Arguments
     * `hid_device_path` - A string slice representing the path to the HID device.
     *
     * # Returns
     * A new Unit161dHid instance.
     */
    pub fn new(hid_device_path: &str) -> Result<Self, ApplicationError> {
        let api = hidapi::HidApi::new().map_err(|e| {
            ApplicationError::Hid(format!("Failed to create HID API instance: {}", e))
        })?;
        let c_path = CString::new(hid_device_path.to_string()).map_err(|e| {
            ApplicationError::Hid(format!(
                "Failed to create CString for HID device path: {}",
                e
            ))
        })?;
        let hiddevice = match api.open_path(&c_path) {
            Ok(dev) => dev,
            Err(e) => {
                return Err(ApplicationError::Hid(format!(
                    "Failed to open HID device at {}: {}",
                    hid_device_path, e
                )));
            }
        };
        Ok(Unit161dHid { hiddevice })
    }

    /**
     * Writes data to the HID device with length prefix.
     *
     * # Arguments
     * `data` - A byte slice representing the data to be written.
     */
    fn write_with_length(&self, data: &[u8]) -> Result<(), ApplicationError> {
        let len = data.len();
        let mut buf = vec![0u8; 1 + len];
        buf[0] = len as u8;
        buf[1..].copy_from_slice(data);
        self.hiddevice
            .write(&buf)
            .map_err(|e| ApplicationError::Hid(format!("Failed to write to HID device: {}", e)))?;
        Ok(())
    }

    /**
     * Reads a response from the HID device using a state machine.
     * # Returns
     * An Option containing the response bytes if successful, or None if failed.
     */
    fn read_response(&self) -> Result<Option<Vec<u8>>, ApplicationError> {
        let mut state = 0;
        let mut buf: Vec<u8> = Vec::new();
        let mut index: usize = 0;
        let mut sum: u32 = 0;
        loop {
            let mut x = [0u8; 64];
            match self.hiddevice.read(&mut x) {
                Ok(_) => {}
                Err(e) => {
                    return Err(ApplicationError::Hid(format!(
                        "Failed to read from HID device: {}",
                        e
                    )));
                }
            }
            for &b in &x[1..] {
                if state < 3 || index + 2 < buf.len() {
                    sum += b as u32;
                }

                match state {
                    0 => {
                        if b == 0xAB {
                            state = 1;
                        }
                    }
                    1 => {
                        if b == 0xCD {
                            state = 2;
                        } else {
                            return Err(ApplicationError::Hid(format!(
                                "Unexpected byte 0x{:02X} in state {}",
                                b, state
                            )));
                        }
                    }
                    2 => {
                        buf = vec![0u8; b as usize];
                        index = 0;
                        state = 3;
                    }
                    3 => {
                        buf[index] = b;
                        index += 1;
                        if index == buf.len() {
                            let received_sum =
                                ((buf[buf.len() - 2] as u16) << 8) + (buf[buf.len() - 1] as u16);
                            if sum != received_sum as u32 {
                                return Err(ApplicationError::Hid("Checksum mismatch".into()));
                            }
                            // Drop last 2 bytes (checksum)
                            buf.truncate(buf.len() - 2);
                            return Ok(Some(buf));
                        }
                    }
                    _ => {
                        return Err(ApplicationError::Hid(format!(
                            "Unexpected byte 0x{:02X} in state {}",
                            b, state
                        )));
                    }
                }
            }
        }
    }
}

#[async_trait(?Send)]
impl Communication for Unit161dHid {

    async fn command(
        &self,
        commands: Vec<String>,
    ) -> Result<Option<Vec<Box<dyn Reading>>>, ApplicationError> {
        let mut measurements: Vec<Box<dyn Reading>> = Vec::new();
        for command in commands {
            let mut cmd = Uni161dCommand::try_from(command)? as u16;
            let mut cmd_bytes = [0u8; 3];
            cmd_bytes[0] = (cmd & 0xff) as u8;
            cmd += 379;
            cmd_bytes[1] = (cmd >> 8) as u8;
            cmd_bytes[2] = (cmd & 0xff) as u8;
            let mut seq = Vec::new();
            seq.extend_from_slice(&SEQUENCE_SEND_CMD);
            seq.extend_from_slice(&cmd_bytes);
            let _ = self.write_with_length(&seq)?;
            if let Some(parsed_measurement) = self.read_response()?.and_then(Unit161dReading::parse)
            {
                measurements.push(Box::new(parsed_measurement));
            }
        }
        Ok(Some(measurements))

    }
}

mod test {
    use crate::instruments::command::Uni161dCommand;

    #[test]
    fn test_try_from_command() {
        assert_eq!(
            Uni161dCommand::try_from("Measure".to_string()).unwrap(),
            Uni161dCommand::Measure
        );
        assert_eq!(
            Uni161dCommand::try_from("MinMax".to_string()).unwrap(),
            Uni161dCommand::MinMax
        );
        assert!(
            Uni161dCommand::try_from("Unknown".to_string()).is_err()
        );
    }
}
