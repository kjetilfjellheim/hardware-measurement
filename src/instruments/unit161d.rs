use std::ffi::CString;

use crate::instruments::{instrument::Instrument, measurement::Measurement};

const CMD_MEASURE: [u8; 6] = [0xAB, 0xCD, 0x03, 0x5E, 0x01, 0xD9];

pub enum Commands {
    MinMax = 65,
    NotMinMax = 66,
    Range = 70,
    Auto = 71,
    Rel = 72,
    Select2 = 73,
    Hold = 74,
    Lamp = 75,
    Select1 = 76,
    PMinMax = 77,
    NotPeak = 78,
}

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
    pub fn new(hid_device_path: &str) -> Self {
        let api = hidapi::HidApi::new().unwrap();
        let c_path = CString::new(hid_device_path.to_string()).unwrap();
        let hiddevice = match api.open_path(&c_path) {
            Ok(dev) => dev,
            Err(e) => {
                eprintln!("Failed to open HID device at {}: {}", hid_device_path, e);
                std::process::exit(1);
            }
        };
        Unit161dHid { hiddevice }
    }

    /**
     * Writes data to the HID device with length prefix.
     *
     * # Arguments
     * `data` - A byte slice representing the data to be written.
     */
    fn write_with_length(&self, data: &[u8]) {
        let len = data.len();
        let mut buf = vec![0u8; 1 + len];
        buf[0] = len as u8;
        buf[1..].copy_from_slice(data);
        self.hiddevice.write(&buf).unwrap();
    }

    /**
     * Reads a response from the HID device using a state machine.
     * # Returns
     * An Option containing the response bytes if successful, or None if failed.
     */
    fn read_response(&self) -> Option<Vec<u8>> {
        // State machine: 0=init, 1=0xAB received, 2=0xCD received, 3=we have length
        let mut state = 0;
        let mut buf: Vec<u8> = Vec::new();
        let mut index: usize = 0;
        let mut sum: u32 = 0;

        loop {
            let mut x = [0u8; 64];
            match self.hiddevice.read(&mut x) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    return None;
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
                            eprintln!("Unexpected byte 0x{:02X} in state {}", b, state);
                            state = 0;
                            sum = 0;
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
                            println!(
                                "Calculated sum=0x{:04X} expected sum=0x{:04X}",
                                sum, received_sum
                            );
                            if sum != received_sum as u32 {
                                eprintln!("Checksum mismatch");
                                return None;
                            }
                            // Drop last 2 bytes (checksum)
                            buf.truncate(buf.len() - 2);
                            return Some(buf);
                        }
                    }
                    _ => {
                        eprintln!("Unexpected byte 0x{:02X} in state {}", b, state);
                    }
                }
            }
        }
    }
}

impl Instrument for Unit161dHid {
    /**
     * Returns the unique identifier of the instrument.
     *
     * # Returns
     * A String representing the device information.
     */
    fn get_device_info(&self) -> String {
        self.hiddevice
            .get_device_info()
            .map(|info| {
                format!(
                    "Unit161d HID - Manufacturer: {:?}, Product: {:?}, Serial Number: {:?}",
                    info.manufacturer_string(),
                    info.product_string(),
                    info.serial_number()
                )
            })
            .unwrap_or("Unit161d HID - Unknown Device".to_string())
    }

    /**'
     * Performs a measurement and returns the result.
     *
     * # Returns
     * An Option containing the Measurement if successful, or None if failed.
     */
    fn get_measurement(&self) -> Option<Measurement> {
        self.write_with_length(&CMD_MEASURE);
        let res_bytes = self.read_response();
        if let Some(res_bytes) = res_bytes {
            return Some(Measurement::new(res_bytes));
        } else {
            return None;
        };
        
    }
}
