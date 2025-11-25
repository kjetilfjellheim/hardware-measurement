use std::fmt::Debug;

/**
 * Enum representing application-level errors.
 */
pub enum ApplicationError {
    /// Error related to USB device operations
    UsbError(String),
    /// Error related to HID device operations
    HidError(String),
    /// Error related to command execution
    CommandError(String),
    /// General application error
    GeneralError(String),
}

impl Debug for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::UsbError(msg) => write!(f, "USB Error: {}", msg),
            ApplicationError::HidError(msg) => write!(f, "HID Error: {}", msg),
            ApplicationError::CommandError(msg) => write!(f, "Command Error: {}", msg),
            ApplicationError::GeneralError(msg) => write!(f, "General Error: {}", msg),
        }
    }
}