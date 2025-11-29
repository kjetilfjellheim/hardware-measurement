use crate::{error::ApplicationError, instruments::reading::Reading,};

#[derive(Debug)]
pub struct ScpiRawReading {
    data: Vec<u8>,
}

impl ScpiRawReading {
    /**
     * Creates a new instance of ScpiRawReading with the given data.
     *
     * # Arguments
     * `data` - A vector of bytes representing the reading data.
     *
     * # Returns
     * A new ScpiReading instance.
     */
    pub fn new(data: Vec<u8>) -> Self {
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
    fn get_raw(&self) -> Result<Vec<u8>, ApplicationError> {
        Ok(self.data.clone())
    }

    /**
     * Returns the raw measurement data as a String.
     *
     * # Returns
     * A Result containing a String with the raw data or an ApplicationError.
     */
    fn get_raw_string(&self) -> Result<String, ApplicationError> {
        match String::from_utf8(self.data.clone()) {
            Ok(s) => Ok(s),
            Err(e) => Err(ApplicationError::General(format!(
                "Failed to convert raw data to string: {}",
                e
            ))),
        }
    }
}