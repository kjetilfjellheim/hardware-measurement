use async_trait::async_trait;

use crate::{error::ApplicationError, instruments::measurement::Measurement};

/**
 * Defines the Instrument trait for hardware measurement instruments.
 */
#[async_trait(?Send)]
pub trait Instrument {
    // Do command specific to the instrument
    async fn command(&self, commands: Vec<String>) -> Result<Option<Measurement>, ApplicationError>;
    // Returns the unique identifier of the instrument
    fn get_device_info(&self) -> String;
}
