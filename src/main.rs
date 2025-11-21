mod arguments;

extern crate hidapi;

use std::ffi::{CString};

use arguments::Args;

fn write_with_length(device: &hidapi::HidDevice, data: &[u8]) {
    let len = data.len();
    let mut buf = vec![0u8; 1 + len];
    buf[0] = len as u8;
    buf[1..].copy_from_slice(data);
    device.write(&buf).unwrap();
}

fn read_response(device: &hidapi::HidDevice) -> Option<Vec<u8>> {
    // State machine: 0=init, 1=0xAB received, 2=0xCD received, 3=we have length
    let mut state = 0;
    let mut buf: Vec<u8> = Vec::new();
    let mut index: usize = 0;
    let mut sum: u32 = 0;

    loop {
        let mut x = [0u8; 64];
        match device.read(&mut x) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Read error: {}", e);
                return None;
            }
        }
        for &b in &x[1..] {
            // Sum all bytes except last 2
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
                        let received_sum = ((buf[buf.len() - 2] as u16) << 8) 
                                         + (buf[buf.len() - 1] as u16);
                        println!("Calculated sum=0x{:04X} expected sum=0x{:04X}", 
                                sum, received_sum);
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

fn main() {
    let args = Args::parse_args();

    let hid_path = match args.hid {
        Some(path) => path,
        None => {
            eprintln!("HID device path not provided. Use --hid <path> to specify it.");
            std::process::exit(1);
        }
    };

    let api = hidapi::HidApi::new().unwrap();
    let c_path = CString::new(hid_path.to_string()).unwrap();
    let device = match api.open_path(&c_path) {
        Ok(dev) => dev,
        Err(e) => {
            eprintln!("Failed to open HID device at {}: {}", hid_path, e);
            std::process::exit(1);
        }
    };
    println!("Successfully opened HID device at {}", hid_path);

    device.get_device_info().map(|info| {
        println!("Manufacturer: {:?}", info.manufacturer_string());
        println!("Product: {:?}", info.product_string());
        println!("Serial Number: {:?}", info.serial_number());
    });

    write_with_length(&device, &[0xAB, 0xCD, 0x03, 0x5E, 0x01, 0xD9]);

    let res_bytes = read_response(&device);

    if let Some(bytes) = res_bytes {
        println!("Received bytes: {:02X?}", bytes);
        println!("Received string: {}", String::from_utf8_lossy(&bytes));
    } else {
        eprintln!("Failed to read a valid response from the device.");
    }    
    println!("HID communication completed.");

}