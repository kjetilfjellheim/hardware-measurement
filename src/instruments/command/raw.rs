/**
 * Trait for SCPI command representation.
 */
pub trait Command: std::fmt::Debug {
    /// Converts the command into a byte vector.
    fn to_command(&self) -> Vec<u8>;
    /// Indicates if the command is a query.
    fn is_query(&self) -> bool;
}
