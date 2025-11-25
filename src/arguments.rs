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
    #[arg(long, num_args=1.., name = "command")]
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
