use clap::{Parser, ValueEnum};

/// Hardware measurement arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {

    /// HID device path
    #[arg(long)]
    pub hid: String,

    /// Send command to the instrument
    #[arg(long, value_enum)]
    pub command: Command,

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
 * Enum representing various commands that can be sent to the instrument.
 */
#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum Command {
    /// Perform a measurement
    Measure,
    /// Enable Min/Max mode
    MinMax,
    /// Disable Min/Max mode
    NotMinMax,
    /// Set range manually
    Range,
    /// Enable Auto mode
    Auto,
    /// Enable Relative mode
    Rel,
    /// Select input 2  
    Select2,
    /// Hold the current measurement
    Hold,
    /// Turn on the backlight lamp
    Lamp,
    /// Select input 1
    Select1,
    /// Enable Peak Min/Max mode
    PMinMax,
    /// Disable Peak Min/Max mode
    NotPeak,
}