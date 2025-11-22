use crate::instruments::{measurement::Measurement, unit161d::Uni161dCommand};

/**
 * Defines the Instrument trait for hardware measurement instruments.
 */
pub trait Instrument {
    // Do command specific to the instrument
    fn command(&self, command: Command);
    // Returns the unique identifier of the instrument
    fn get_device_info(&self) -> String;
    // Performs a measurement and returns the result
    fn get_measurement(&self) -> Option<Measurement>;
}

/**
 * Enum representing various commands that can be sent to the instrument.
 */
pub enum Command {
    MinMax,
    NotMinMax,
    Range,
    Auto,
    Rel,
    Select2,
    Hold,
    Lamp,
    Select1,
    PMinMax,
    NotPeak,
}