use crate::instruments::measurement::Measurement;

/**
 * Defines the Instrument trait for hardware measurement instruments.
 */
pub trait Instrument {
    // Returns the unique identifier of the instrument
    fn get_device_info(&self) -> String;
    // Performs a measurement and returns the result
    fn get_measurement(&self) -> Option<Measurement>;
}

