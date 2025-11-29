use async_trait::async_trait;

use crate::error::ApplicationError;

/**
 * Defines the Reading trait for measurement data returned by instruments.
 */
#[async_trait(?Send)]
pub trait Reading {
    /**
     * Returns the measurement data in CSV format.
     *
     * # Returns
     * A Result containing a String in CSV format or an ApplicationError.
     */
    fn get_csv(&self) -> Result<String, ApplicationError>;

    /**
     * Returns the raw measurement data as a byte vector.
     *
     * # Returns
     * The raw measurement as a byte vector.
     */
    fn get_raw(&self) -> Result<Vec<u8>, ApplicationError>;

    /**
     * Returns the raw measurement data as a String.
     *
     * # Returns
     * The raw measurement as a String.
     */
    fn get_raw_string(&self) -> Result<String, ApplicationError>;
}