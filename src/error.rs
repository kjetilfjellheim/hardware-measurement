use std::fmt::Debug;

/**
 * Enum representing application-level errors.
 */
pub enum ApplicationError {
    /// Error related to USB device operations
    Usb(String),
    /// Error related to HID device operations
    Hid(String),
    /// Error related to command execution
    Command(String),
    /// General application error
    General(String),
}

impl Debug for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::Usb(msg) => write!(f, "USB Error: {}", msg),
            ApplicationError::Hid(msg) => write!(f, "HID Error: {}", msg),
            ApplicationError::Command(msg) => write!(f, "Command Error: {}", msg),
            ApplicationError::General(msg) => write!(f, "General Error: {}", msg),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ApplicationError;

    #[test]
    fn test_debug_usb_error() {
        let error = ApplicationError::Usb("Device not found".into());
        assert_eq!(format!("{:?}", error), "USB Error: Device not found");
    }

    #[test]
    fn test_debug_hid_error() {
        let error = ApplicationError::Hid("Failed to open HID device".into());
        assert_eq!(format!("{:?}", error), "HID Error: Failed to open HID device");
    }

    #[test]
    fn test_debug_command_error() {
        let error = ApplicationError::Command("Invalid command".into());
        assert_eq!(format!("{:?}", error), "Command Error: Invalid command");
    }

    #[test]
    fn test_debug_general_error() {
        let error = ApplicationError::General("An unknown error occurred".into());
        assert_eq!(format!("{:?}", error), "General Error: An unknown error occurred");
    }
}