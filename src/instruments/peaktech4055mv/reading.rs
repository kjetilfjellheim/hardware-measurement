use crate::{error::ApplicationError, instruments::instrument::Reading};

struct Peaktech4055mvReading {

}

impl Reading for Peaktech4055mvReading {
    fn get_csv(&self) -> Result<String, ApplicationError> {
        Ok("value1,value2,value3".to_string())
    }
}