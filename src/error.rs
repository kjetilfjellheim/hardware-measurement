#[repr(u64)]
#[derive(Debug)]
pub enum ApplicationError {
    /// Error related to USB device operations
    UsbError(String) = 1,
    /// Error related to HID device operations
    HidError(String) = 2,
    /// Error related to command execution
    CommandError(String) = 3,
    /// General application error
    GeneralError(String) = 4,
}
