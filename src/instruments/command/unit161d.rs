use crate::error::ApplicationError;

/**
 * Enum representing various commands for the Uni-T 161D instrument.
 */
#[derive(Debug, PartialEq)]
pub enum Uni161dCommand {
    Measure = 94,
    MinMax = 65,
    NotMinMax = 66,
    Range = 70,
    Auto = 71,
    Rel = 72,
    Select2 = 73,
    Hold = 74,
    Lamp = 75,
    Select1 = 76,
    PMinMax = 77,
    NotPeak = 78,
}

impl TryFrom<String> for Uni161dCommand {
    type Error = ApplicationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Measure" => Ok(Uni161dCommand::Measure),
            "MinMax" => Ok(Uni161dCommand::MinMax),
            "NotMinMax" => Ok(Uni161dCommand::NotMinMax),
            "Range" => Ok(Uni161dCommand::Range),
            "Auto" => Ok(Uni161dCommand::Auto),
            "Rel" => Ok(Uni161dCommand::Rel),
            "Select2" => Ok(Uni161dCommand::Select2),
            "Hold" => Ok(Uni161dCommand::Hold),
            "Lamp" => Ok(Uni161dCommand::Lamp),
            "Select1" => Ok(Uni161dCommand::Select1),
            "PMinMax" => Ok(Uni161dCommand::PMinMax),
            "NotPeak" => Ok(Uni161dCommand::NotPeak),
            _ => Err(ApplicationError::Command(format!(
                "Unknown command: {}",
                value
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instruments::command::unit161d::Uni161dCommand;

    #[test]
    fn test_try_from_command() {
        assert_eq!(
            Uni161dCommand::try_from("Measure".to_string()).unwrap(),
            Uni161dCommand::Measure
        );
        assert_eq!(
            Uni161dCommand::try_from("MinMax".to_string()).unwrap(),
            Uni161dCommand::MinMax
        );
        assert!(Uni161dCommand::try_from("Unknown".to_string()).is_err());
    }
}