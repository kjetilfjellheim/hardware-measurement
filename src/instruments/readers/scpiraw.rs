use crate::{error::ApplicationError, instruments::readers::Reading,};

#[derive(Debug)]
pub struct ScpiRawReading {
    data: Vec<String>,
}

impl ScpiRawReading {
    /**
     * Creates a new instance of ScpiRawReading with the given data.
     *
     * # Arguments
     * `data` - A vector of strings representing the reading data.
     *
     * # Returns
     * A new ScpiReading instance.
     */
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }
}

impl Reading for ScpiRawReading {
    /**
     * Not supported for ScpiRawReading.
     *
     * # Returns
     * Always Err.
     */
    fn get_csv(&self) -> Result<String, ApplicationError> {
        Err(ApplicationError::General(
            "ScpiRawReading does not support CSV format".into(),
        ))
    }
    /**
     * Returns the raw measurement data as a string.
     *
     * # Returns
     * A Result containing a String with the raw data or an ApplicationError.
     */
    fn get_raw(&self) -> Result<String, ApplicationError> {
        Ok(self.data.join("\n"))
    }
}