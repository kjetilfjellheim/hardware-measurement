use clap::{Parser, ValueEnum};

/// Hardware measurement arguments
#[derive(Parser, Debug, Clone, PartialEq)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Measurement device
    #[arg(long)]
    pub device: Device,

    /// HID device path
    #[arg(long)]
    pub hid: Option<String>,

    /// USB device path (vendor_id:product_id)
    #[arg(long)]
    pub usb: Option<String>,

    /// Send commands to the instrument
    /// Suppported commands are instrument specific.
    ///
    /// Unit161d
    /// MinMax, NotMinMax, Range, Auto, Rel, Select2, Hold, Lamp, Select1, PMinMax, NotPeak, Measure
    /// GenericScpiUsb (Peaktech4055mv)
    /// Apply:Waveform [Frequency, Amplitude, Offset]
    #[arg(long="command", num_args=1..)]
    pub commands: Vec<String>,

    /// Reader type for interpreting instrument responses.. For scpi devices the default is ScpiRawReader.
    #[arg(long)]
    pub reader: Option<Reader>,

    /// Output format. The default is Raw.
    #[arg(long)]
    pub format: Option<Format>,
}

impl Args {
    /**
     * Parses command-line arguments and returns an Args instance.
     *
     * # Returns
     * An Args instance containing the parsed arguments.
     */
    pub fn parse_args() -> Self {
        Args::parse()
    }
}

/**
 * Enum representing supported measurement devices.
 */
#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum Device {
    Unit161d,
    GenericScpiUsb,
}
/**
 * Enum representing supported reader types.
 */
#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum Reader {
    ScpiRawReader,    
}

/**
 * Enum representing supported output formats.
 */
#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum Format {
    Csv,
    Raw,
}

#[cfg(test)]
mod test {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_parse_args() {
        let args = Args::parse_from(&[
            "test_program",
            "--device",
            "unit161d",
            "--hid",
            "/dev/hidraw0",
            "--command",
            "Measure",
            "--command",
            "Hold",
        ]);

        assert_eq!(args.device, Device::Unit161d);
        assert_eq!(args.hid, Some("/dev/hidraw0".to_string()));
        assert_eq!(args.commands, vec!["Measure".to_string(), "Hold".to_string()]);
    }

    #[test]
    fn test_parse_args_peaktech() {
        let args = Args::parse_from(&[
            "test_program",
            "--device",
            "generic-scpi-usb",
            "--usb",
            "1234:5678",
            "--command",
            "Apply:Waveform 1000, 5, 0"
        ]);

        assert_eq!(args.device, Device::GenericScpiUsb);
        assert_eq!(args.usb, Some("1234:5678".to_string()));
        assert_eq!(args.commands, vec!["Apply:Waveform 1000, 5, 0".to_string()]);
    }
}