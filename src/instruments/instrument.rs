use async_trait::async_trait;

use crate::{error::ApplicationError, instruments::measurement::Measurement};

/**
 * Defines the Instrument trait for hardware measurement instruments.
 */
#[async_trait(?Send)]
pub trait Instrument {
    // Do command specific to the instrument
    async fn command(&self, command: Command) -> Result<Option<Measurement>, ApplicationError>;
    // Returns the unique identifier of the instrument
    fn get_device_info(&self) -> String;
}

/**
 * Enum representing various commands that can be sent to the instrument.
 */
#[derive(Debug)]
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
    Measure,
}

impl From<crate::arguments::Command> for Command {
    fn from(command: crate::arguments::Command) -> Self {
        match command {
            crate::arguments::Command::MinMax => Command::MinMax,
            crate::arguments::Command::NotMinMax => Command::NotMinMax,
            crate::arguments::Command::Range => Command::Range,
            crate::arguments::Command::Auto => Command::Auto,
            crate::arguments::Command::Rel => Command::Rel,
            crate::arguments::Command::Select2 => Command::Select2,
            crate::arguments::Command::Hold => Command::Hold,
            crate::arguments::Command::Lamp => Command::Lamp,
            crate::arguments::Command::Select1 => Command::Select1,
            crate::arguments::Command::PMinMax => Command::PMinMax,
            crate::arguments::Command::NotPeak => Command::NotPeak,
            crate::arguments::Command::Measure => Command::Measure,
        }
    }
}
