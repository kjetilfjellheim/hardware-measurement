use crate::{error::ApplicationError, instruments::Command};

/**
 * Struct representing a SCPI command.
 */
pub struct ScpiRawCommand {
    /// The SCPI command string.
    command: String,
}

impl ScpiRawCommand {
    /**
     * Creates a new ScpiRawCommand instance.
     *
     * # Arguments
     * `command` - A string representing the SCPI command.
     *
     * # Returns
     * A new ScpiCommand instance.
     */
    pub fn new(command: &str) -> Self {
        Self { command: command.to_string() }
    }   
}

impl Command for ScpiRawCommand {
    /**
     * Converts the command into a byte vector.
     *
     * # Returns
     * A vector of bytes representing the command.
     */
    fn to_command(&self) -> Vec<u8> {
        let mut cmd = self.command.clone();
        cmd.push('\n');
        cmd.into_bytes()
    }

    /**
     * Indicates if the command is a query.
     *
     * # Returns
     * true if the command is a query, false otherwise.
     */
    fn is_query(&self) -> bool {
        self.command.ends_with('?')
    }
}

impl std::fmt::Debug for ScpiRawCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ScpiRawCommand: {}", self.command)
    }
}

impl TryFrom<String> for Box<dyn Command> {
    type Error = ApplicationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Box::new(ScpiRawCommand::new(&value)))
    }
}   


#[cfg(test)]
mod test {
    use crate::instruments::Command;


    const APPLY_COMMAND_SIN: &str = "Apply:Sin 10kHz, 1.2, 0.5\n";
    const APPLY_COMMAND_SQU: &str = "Apply:Squ 10kHz, 1.5, 0.1\n";
    const APPLY_COMMAND_SQU2: &str = "Apply:Squ 10kHz, 1.5\n";
    const APPLY_COMMAND_SQU3: &str = "Apply:Squ 10kHz\n";
    const APPLY_COMMAND_SQU4: &str = "Apply:Squ\n";

    #[test]
    fn test_successful_apply_commands() {
        let cmd: Box<dyn Command> =
            "Apply:Sin 10kHz, 1.2, 0.5".to_string().try_into().unwrap();
        assert_eq!(cmd.to_command(), APPLY_COMMAND_SIN.as_bytes());

        let cmd: Box<dyn Command> =
            "Apply:Squ 10kHz, 1.5, 0.1".to_string().try_into().unwrap();
        assert_eq!(cmd.to_command(), APPLY_COMMAND_SQU.as_bytes());
        let cmd: Box<dyn Command> =
            "Apply:Squ 10kHz, 1.5".to_string().try_into().unwrap();
        assert_eq!(cmd.to_command(), APPLY_COMMAND_SQU2.as_bytes());

        let cmd: Box<dyn Command> = "Apply:Squ 10kHz".to_string().try_into().unwrap();
        assert_eq!(cmd.to_command(), APPLY_COMMAND_SQU3.as_bytes());

        let cmd: Box<dyn Command> = "Apply:Squ".to_string().try_into().unwrap();
        assert_eq!(cmd.to_command(), APPLY_COMMAND_SQU4.as_bytes());
    }

}
