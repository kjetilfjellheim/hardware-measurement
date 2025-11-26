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
    /// Peaktech4055mv
    /// Apply:Waveform [Frequency, Amplitude, Offset]
    #[arg(long="command", num_args=1..)]
    pub commands: Vec<String>,
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
    Peaktech4055mv,
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
            "peaktech4055mv",
            "--usb",
            "1234:5678",
            "--command",
            "Apply:Waveform 1000, 5, 0"
        ]);

        assert_eq!(args.device, Device::Peaktech4055mv);
        assert_eq!(args.usb, Some("1234:5678".to_string()));
        assert_eq!(args.commands, vec!["Apply:Waveform 1000, 5, 0".to_string()]);
    }
}