use async_trait::async_trait;

use crate::{
    error::ApplicationError,
    instruments::instrument::{Communication, Reading},
};
use nusb::{
    list_devices,
    transfer::{Buffer, Bulk, Out},
    DeviceInfo,
};

/**
 * USB Bulk OUT endpoint address for Peaktech4055mv.
 */
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
            .ok_or_else(|| ApplicationError::Usb("Missing vendor ID".into()))?
            .parse()
            .map_err(|e| ApplicationError::Usb(format!("Invalid vendor ID: {}", e)))?;
        let product_id = device_path
            .get(1)
            .ok_or_else(|| ApplicationError::Usb("Missing product ID".into()))?
            .parse()
            .map_err(|e| ApplicationError::Usb(format!("Invalid product ID: {}", e)))?;
        let device = list_devices()
            .await
            .map_err(|e| ApplicationError::Usb(format!("Could not list usb devices: {}", e)))?
            .find(|dev| dev.vendor_id() == vendor_id && dev.product_id() == product_id)
            .ok_or_else(|| ApplicationError::Usb("Peaktech4055mv device not found".into()))?;
        Ok(Self { device })
    }
}

#[async_trait(?Send)]
impl Communication for Peaktech4055mv {
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
        let parsed_commands: Vec<Box<dyn Peaktech4055mvCommand>> = commands
            .iter()
            .map(
                |cmd| -> Result<Box<dyn Peaktech4055mvCommand>, ApplicationError> {
                    cmd.clone().try_into()
                },
            )
            .collect::<Result<Vec<_>, _>>()?;

        let open_device =
            self.device.open().await.map_err(|e| {
                ApplicationError::Usb(format!("Could not open usb device: {}", e))
            })?;
        let interface = open_device.claim_interface(0).await.map_err(|e| {
            ApplicationError::Usb(format!("Could not open interface 0: {}", e))
        })?;
        // Get the endpoint and submit transfer
        let mut endpoint = interface
            .endpoint::<Bulk, Out>(BULK_OUT_ADDRESS)
            .map_err(|e| ApplicationError::Usb(format!("Failed to get endpoint: {}", e)))?;

        for command in parsed_commands {
            let command_string = command.to_command_string();

            let buffer = Buffer::from(command_string.as_bytes().to_vec());

            endpoint.submit(buffer);
            let completion = endpoint.next_complete().await;

            match completion.status {
                Ok(()) => {}
                Err(e) => {
                    return Err(ApplicationError::Command(format!(
                        "Failed to send command {:?}: {:?}",
                        command_string, e
                    )))
                }
            }
        }
        Ok(None)
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

/**
 * Trait for Peaktech4055mv commands.
 */
trait Peaktech4055mvCommand {
    /// Parses a command string into a Peaktech4055mvCommand.
    fn parse(cmd: &str) -> Result<Self, ApplicationError>
    where
        Self: Sized;
    /// Converts the command into a string representation.
    fn to_command_string(&self) -> String;
}

/**
 * Structs representing different Peaktech4055mv commands.
 */
struct Peaktech4055mvCommandApply {
    /// Waveform type for the apply command
    waveform: Waveform,
    /// Frequency setting
    frequency: Option<String>,
    /// Amplitude setting
    amplitude: Option<String>,
    /// Offset setting
    offset: Option<String>,
}

/**
 * Struct representing the Reset command for Peaktech4055mv.
 */
struct Peaktech4055mvCommandReset;

/**
 * Struct representing a raw command for Peaktech4055mv.
 */
struct Peaktech4055mvCommandRaw {
    /// Raw command string
    command: String,
}

impl Peaktech4055mvCommand for Peaktech4055mvCommandApply {
    fn parse(cmd: &str) -> Result<Self, ApplicationError> {
        let parts: Vec<&str> = cmd["Apply:".len()..].split(',').collect();
        let waveform_str = parts[0].split(' ').next().unwrap_or("").trim();
        let waveform = match waveform_str {
            "Sin" => Waveform::Sin,
            "Squ" => Waveform::Squ,
            "Ramp" => Waveform::Ramp,
            "Noise" => Waveform::Noise,
            "PPulse" => Waveform::PPulse,
            "NPulse" => Waveform::NPulse,
            "Stair" => Waveform::Stair,
            "HSine" => Waveform::HSine,
            "LSine" => Waveform::LSine,
            "Rexp" => Waveform::Rexp,
            "RLog" => Waveform::RLog,
            "Tang" => Waveform::Tang,
            "Sinc" => Waveform::Sinc,
            "Round" => Waveform::Round,
            "Card" => Waveform::Card,
            "Quake" => Waveform::Quake,
            _ => {
                return Err(ApplicationError::Command(format!(
                    "Unknown waveform: {}",
                    waveform_str
                )))
            }
        };
        let frequency: Option<String> = parts
            .first()
            .and_then(|s| s.split(' ').nth(1))
            .map(|s| s.trim().to_string());
        let amplitude: Option<String> = parts
            .len()
            .checked_sub(2)
            .map(|_| parts[1].trim().to_string());
        let offset: Option<String> = parts
            .len()
            .checked_sub(3)
            .map(|_| parts[2].trim().to_string());

        if parts.len() > 3 {
            return Err(ApplicationError::Command(
                "Too many parameters for Apply command".into(),
            ));
        }
        if waveform_str.is_empty() {
            return Err(ApplicationError::Command(
                "Waveform type is required for Apply command".into(),
            ));
        }
        if amplitude.is_some() && frequency.is_none() {
            return Err(ApplicationError::Command(
                "Frequency cannot be empty if provided".into(),
            ));
        }
        if offset.is_some() && amplitude.is_none() {
            return Err(ApplicationError::Command(
                "Amplitude cannot be empty if provided".into(),
            ));
        }
        Ok(Peaktech4055mvCommandApply {
            waveform,
            frequency,
            amplitude,
            offset,
        })
    }

    /**
     * Converts the command into a string representation.
     * Example : "Apply:Sin 10kHz, 1.2, 0.5\n"
     * Example : "Apply:Sin 10kHz, 1.2\n"
     * Example : "Apply:Sin 10kHz\n"
     * Example : "Apply:Sin\n"
     * # Returns
     * A String representing the command.
     */
    fn to_command_string(&self) -> String {
        let mut command = format!("Apply:{:?}", self.waveform);
        if let Some(freq) = &self.frequency {
            command.push_str(&format!(" {}", freq));
        }
        if let Some(amp) = &self.amplitude {
            command.push_str(&format!(", {}", amp));
        }
        if let Some(off) = &self.offset {
            command.push_str(&format!(", {}", off));
        }
        command.push('\n');
        command
    }
}

impl Peaktech4055mvCommand for Peaktech4055mvCommandReset {
    /**
    * Parses the Reset command.
    *
    * # Arguments
    * `cmd` - A string slice representing the command.
    *
    * # Returns
    * A Peaktech4055mvCommandReset instance.
     */
    fn parse(_cmd: &str) -> Result<Self, ApplicationError> {
        Ok(Peaktech4055mvCommandReset)
    }

    /**
     * Converts the command into a string representation.
     *
     * # Returns
     * A String representing the command.
     */
    fn to_command_string(&self) -> String {
        "*RST\n".to_string()
    }
}

impl Peaktech4055mvCommand for Peaktech4055mvCommandRaw {

    /**
    * Parses a raw command string.
    *
    * # Arguments
    * `cmd` - A string slice representing the command.
    *
    * # Returns
    * A Peaktech4055mvCommandRaw instance.
    */
    fn parse(cmd: &str) -> Result<Self, ApplicationError> {
        let mut str = cmd.replace("Raw:", "");
        str.push('\n');
        Ok(Peaktech4055mvCommandRaw { command: str })
    }

    /**
     * Converts the command into a string representation.
     *
     * # Returns
     * A String representing the command.
     */
    fn to_command_string(&self) -> String {
        self.command.clone()
    }
}

/**
 * Enum representing supported waveforms for the Peaktech4055mv.
 */
#[derive(Debug, PartialEq)]
pub enum Waveform {
    Sin,
    Squ,
    Ramp,
    Noise,
    PPulse,
    NPulse,
    Stair,
    HSine,
    LSine,
    Rexp,
    RLog,
    Tang,
    Sinc,
    Round,
    Card,
    Quake,
}

/**
 * Implements TryFrom<String> for Peaktech4055mvCommand trait objects.
 * This allows for convenient conversion from a command string to a boxed command object.
 */
impl TryFrom<String> for Box<dyn Peaktech4055mvCommand> {
    type Error = ApplicationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("Apply:") {
            Peaktech4055mvCommandApply::parse(&value)
                .map(|cmd| Box::new(cmd) as Box<dyn Peaktech4055mvCommand>)
        } else if value == "Reset" {
            Peaktech4055mvCommandReset::parse(&value)
                .map(|cmd| Box::new(cmd) as Box<dyn Peaktech4055mvCommand>)
        } else {
            Peaktech4055mvCommandRaw::parse(&value)
                .map(|cmd| Box::new(cmd) as Box<dyn Peaktech4055mvCommand>)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instruments::peaktech4055mv::instrumentation::Peaktech4055mvCommand;

    const APPLY_COMMAND_SIN: &str = "Apply:Sin 10kHz, 1.2, 0.5\n";
    const APPLY_COMMAND_SQU: &str = "Apply:Squ 10kHz, 1.5, 0.1\n";
    const APPLY_COMMAND_SQU2: &str = "Apply:Squ 10kHz, 1.5\n";
    const APPLY_COMMAND_SQU3: &str = "Apply:Squ 10kHz\n";
    const APPLY_COMMAND_SQU4: &str = "Apply:Squ\n";
    const RESET_COMMAND: &str = "*RST\n";

    #[test]
    fn test_successful_apply_commands() {
        let cmd: Box<dyn Peaktech4055mvCommand> =
            "Apply:Sin 10kHz, 1.2, 0.5".to_string().try_into().unwrap();
        assert_eq!(cmd.to_command_string(), APPLY_COMMAND_SIN);

        let cmd: Box<dyn Peaktech4055mvCommand> =
            "Apply:Squ 10kHz, 1.5, 0.1".to_string().try_into().unwrap();
        assert_eq!(cmd.to_command_string(), APPLY_COMMAND_SQU);

        let cmd: Box<dyn Peaktech4055mvCommand> =
            "Apply:Squ 10kHz, 1.5".to_string().try_into().unwrap();
        assert_eq!(cmd.to_command_string(), APPLY_COMMAND_SQU2);

        let cmd: Box<dyn Peaktech4055mvCommand> = "Apply:Squ 10kHz".to_string().try_into().unwrap();
        assert_eq!(cmd.to_command_string(), APPLY_COMMAND_SQU3);

        let cmd: Box<dyn Peaktech4055mvCommand> = "Apply:Squ".to_string().try_into().unwrap();
        assert_eq!(cmd.to_command_string(), APPLY_COMMAND_SQU4);
    }

    #[test]
    fn test_successful_reset() {
        let cmd: Box<dyn Peaktech4055mvCommand> = "Reset".to_string().try_into().unwrap();
        assert_eq!(cmd.to_command_string(), RESET_COMMAND);
    }

    #[test]
    fn test_successful_raw() {
        let cmd: Box<dyn Peaktech4055mvCommand> = "Raw:Apply:Sin 10kHz, 1.2, 0.5"
            .to_string()
            .try_into()
            .unwrap();
        assert_eq!(cmd.to_command_string(), APPLY_COMMAND_SIN);
    }

    #[test]
    fn test_invalid_peaktech4055mv_command_from_string() {
        let command_str = "Apply:Sin, 10kHz,,".to_string();
        let command: Result<Box<dyn Peaktech4055mvCommand>, _> = command_str.try_into();
        assert!(command.is_err());
    }
}
