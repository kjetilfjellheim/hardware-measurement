use crate::{error::ApplicationError, instruments::reading::Reading};

// Decoded modes
const MODE: [&str; 31] = [
    "ACV", "ACmV", "DCV", "DCmV", "Hz", "%", "OHM", "CONT", "DIDOE", "CAP", "°C", "°F", "DCuA",
    "ACuA", "DCmA", "ACmA", "DCA", "ACA", "HFE", "Live", "NCV", "LozV", "ACA", "DCA", "LPF",
    "AC/DC", "LPF", "AC+DC", "LPF", "AC+DC2", "INRUSH",
];

// Strings that could mean overload
const OVERLOAD: [&str; 8] = [".OL", "O.L", "OL.", "OL", "-.OL", "-O.L", "-OL.", "-OL"];

// Strings that indicate level of voltage detected >=50Vrms (50-60Hz)
const NCV: [&str; 6] = ["EF", "-", "--", "---", "----", "-----"];

// Get unit based on mode and range
fn get_unit(mode: &str, range: &str) -> Option<&'static str> {
    match (mode, range) {
        ("%", "0") => Some("%"),

        ("AC+DC", "1") => Some("A"),
        ("AC+DC2", "1") => Some("A"),

        ("AC/DC", "0") => Some("V"),
        ("AC/DC", "1") => Some("V"),
        ("AC/DC", "2") => Some("V"),
        ("AC/DC", "3") => Some("V"),

        ("ACA", "1") => Some("A"),

        ("ACV", "0") => Some("V"),
        ("ACV", "1") => Some("V"),
        ("ACV", "2") => Some("V"),
        ("ACV", "3") => Some("V"),

        ("ACmA", "0") => Some("mA"),
        ("ACmA", "1") => Some("mA"),

        ("ACmV", "0") => Some("mV"),

        ("ACuA", "0") => Some("uA"),
        ("ACuA", "1") => Some("uA"),

        ("CAP", "0") => Some("nF"),
        ("CAP", "1") => Some("nF"),
        ("CAP", "2") => Some("uF"),
        ("CAP", "3") => Some("uF"),
        ("CAP", "4") => Some("uF"),
        ("CAP", "5") => Some("mF"),
        ("CAP", "6") => Some("mF"),
        ("CAP", "7") => Some("mF"),

        ("CONT", "0") => Some("Ω"),

        ("DCA", "1") => Some("A"),

        ("DCV", "0") => Some("V"),
        ("DCV", "1") => Some("V"),
        ("DCV", "2") => Some("V"),
        ("DCV", "3") => Some("V"),

        ("DCmA", "0") => Some("mA"),
        ("DCmA", "1") => Some("mA"),

        ("DCmV", "0") => Some("mV"),

        ("DCuA", "0") => Some("uA"),
        ("DCuA", "1") => Some("uA"),

        ("DIDOE", "0") => Some("V"),

        ("Hz", "0") => Some("Hz"),
        ("Hz", "1") => Some("Hz"),
        ("Hz", "2") => Some("kHz"),
        ("Hz", "3") => Some("kHz"),
        ("Hz", "4") => Some("kHz"),
        ("Hz", "5") => Some("MHz"),
        ("Hz", "6") => Some("MHz"),
        ("Hz", "7") => Some("MHz"),

        ("LPF", "0") => Some("V"),
        ("LPF", "1") => Some("V"),
        ("LPF", "2") => Some("V"),
        ("LPF", "3") => Some("V"),

        ("LozV", "0") => Some("V"),
        ("LozV", "1") => Some("V"),
        ("LozV", "2") => Some("V"),
        ("LozV", "3") => Some("V"),

        ("OHM", "0") => Some("Ω"),
        ("OHM", "1") => Some("kΩ"),
        ("OHM", "2") => Some("kΩ"),
        ("OHM", "3") => Some("kΩ"),
        ("OHM", "4") => Some("MΩ"),
        ("OHM", "5") => Some("MΩ"),
        ("OHM", "6") => Some("MΩ"),

        ("°C", "0") => Some("°C"),
        ("°C", "1") => Some("°C"),

        ("°F", "0") => Some("°F"),
        ("°F", "1") => Some("°F"),

        ("HFE", "0") => Some("B"),

        ("NCV", "0") => Some("NCV"),

        _ => None,
    }
}

/**
 * Checks if the display value indicates overload.
 * # Arguments
 * `value` - A string slice representing the display value.
 *
 * # Returns
 * A boolean indicating whether the value represents an overload.
 */
fn is_overload(value: &str) -> bool {
    OVERLOAD.contains(&value)
}

/**
 * Checks if the display value indicates NCV (Non-Contact Voltage).
 * # Arguments
 * `value` - A string slice representing the display value.
 *
 * # Returns
 * A boolean indicating whether the value represents NCV.
 */
fn is_ncv(value: &str) -> bool {
    NCV.contains(&value)
}

/**
* Represents a measurement taken by an instrument.
 */
#[derive(Debug)]
pub struct Unit161dReading {
    pub original_bytes: Vec<u8>,
    pub mode: String,
    pub range: String,
    pub display_value: String,
    pub overload: bool,
    pub ncv: bool,
    pub decimal_value: Option<f64>, //Todo: Change number representation
    pub display_unit: String,
    pub progres: u16,
    pub max: bool,
    pub min: bool,
    pub hold: bool,
    pub rel: bool,
    pub auto: bool,
    pub battery: bool,
    pub hwwarning: bool,
    pub dc: bool,
    pub peak_max: bool,
    pub peak_min: bool,
    pub bar_polarity: bool,
}

impl Unit161dReading {
    /**
     * Creates a new Reading instance.
     *
     * # Arguments
     * `bytes` - A vector of bytes representing the raw measurement data.
     *
     * # Returns
     * A new Measurement instance.
     */
    pub fn parse(bytes: Vec<u8>) -> Option<Self> {
        // Ensure we have enough bytes, if not it's an invalid measurement
        if bytes.len() < 14 {
            return None;
        }
        let mode = MODE
            .get(bytes[0] as usize)
            .unwrap_or(&"Unknown")
            .to_string();
        let range = String::from_utf8_lossy(&bytes[1..2]).to_string();
        let display_value = String::from_utf8_lossy(&bytes[2..9]).trim().to_string();
        let overload = is_overload(&display_value);
        let ncv = is_ncv(&display_value);
        let decimal_value = if overload || ncv {
            None
        } else {
            display_value.parse::<f64>().ok()
        };
        let display_unit = get_unit(&mode, &range).unwrap_or("Unknown").to_string();
        let progres: u16 = bytes[9] as u16 * 10 + bytes[10] as u16;
        let max = bytes[11] & 8 > 0;
        let min = bytes[11] & 4 > 0;
        let hold = bytes[11] & 2 > 0;
        let rel = bytes[11] & 1 > 0;
        let auto = bytes[12] & 4 > 0;
        let battery = bytes[12] & 2 > 0;
        let hwwarning = bytes[12] & 1 > 0;
        let dc = bytes[13] & 8 > 0;
        let peak_max = bytes[13] & 4 > 0;
        let peak_min = bytes[13] & 2 > 0;
        let bar_polarity = bytes[13] & 1 > 0;

        Some(Unit161dReading {
            original_bytes: bytes.clone(),
            mode,
            range,
            display_value,
            overload,
            ncv,
            decimal_value,
            display_unit,
            progres,
            max,
            min,
            hold,
            rel,
            auto,
            battery,
            hwwarning,
            dc,
            peak_max,
            peak_min,
            bar_polarity,
        })
    }
}

impl Reading for Unit161dReading {
    /**
     * Returns the measurement data in CSV format.
     *
     * # Returns
     * A Result containing a String in CSV format or an ApplicationError.
     */
    fn get_csv(&self) -> Result<String, ApplicationError> {
        Ok(format!(
            "{},{},{},{},{},{:?},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.mode,
            self.range,
            self.display_value,
            self.overload,
            self.ncv,
            self.decimal_value,
            self.display_unit,
            self.progres,
            self.max,
            self.min,
            self.hold,
            self.rel,
            self.auto,
            self.battery,
            self.hwwarning,
            self.dc,
            self.peak_max,
            self.peak_min,
            self.bar_polarity
        ))
    }

    /**
     * Returns the raw measurement data as a byte vector. 
     *
     * # Returns
     * A Result containing a byte vector with the raw data or an ApplicationError.
     */
    fn get_raw(&self) -> Result<Vec<u8>, ApplicationError> {
        Ok(self.original_bytes.clone())
    }

    /**
     * Returns the measurement data as a string.
     *
     * # Returns
     * A Result containing a String with the raw representation or an ApplicationError.
     */
    fn get_raw_string(&self) -> Result<String,ApplicationError> {
        String::from_utf8(self.original_bytes.clone()).map_err(|e| {
            ApplicationError::General(format!("Failed to convert raw bytes to string: {}", e))
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_unit161d_reading_parse() {
        let raw_data = vec![
            2, 0, b'1', b'2', b'3', b'.', b'4', b'5', b'6', 5, 0, 0b00001110, 0b00000111,
            0b00001111,
        ];
        let reading = Unit161dReading::parse(raw_data).unwrap();

        assert_eq!(reading.mode, "DCV");
        assert_eq!(reading.range, "\0");
        assert_eq!(reading.display_value, "123.456");
        assert_eq!(reading.overload, false);
        assert_eq!(reading.ncv, false);
        assert_eq!(reading.decimal_value, Some(123.456));
        assert_eq!(reading.display_unit, "Unknown");
        assert_eq!(reading.progres, 50);
        assert_eq!(reading.max, true);
        assert_eq!(reading.min, true);
        assert_eq!(reading.hold, true);
        assert_eq!(reading.rel, false);
        assert_eq!(reading.auto, true);
        assert_eq!(reading.battery, true);
        assert_eq!(reading.hwwarning, true);
        assert_eq!(reading.dc, true);
        assert_eq!(reading.peak_max, true);
        assert_eq!(reading.peak_min, true);
        assert_eq!(reading.bar_polarity, true);
    }

    #[test]
    fn test_unit161d_reading_get_csv() {
        let reading = Unit161dReading {
            original_bytes: vec![],
            mode: "DCV".to_string(),
            range: "\0".to_string(),
            display_value: "123.456".to_string(),
            overload: false,
            ncv: false,
            decimal_value: Some(123.456),
            display_unit: "V".to_string(),
            progres: 50,
            max: true,
            min: true,
            hold: true,
            rel: false,
            auto: true,
            battery: true,
            hwwarning: true,
            dc: true,
            peak_max: true,
            peak_min: true,
            bar_polarity: true,
        };

        let csv = reading.get_csv().unwrap();
        let expected_csv = "DCV,\0,123.456,false,false,Some(123.456),V,50,true,true,true,false,true,true,true,true,true,true,true";
        assert_eq!(csv, expected_csv);
    }

    #[test]
    fn test_overload_detection() {
        let overload_values = vec![".OL", "O.L", "OL.", "OL", "-.OL", "-O.L", "-OL.", "-OL"];
        for value in overload_values {
            assert!(is_overload(value));
        }

        let non_overload_values = vec!["123.45", "0.00", "9999", "NCV"];
        for value in non_overload_values {
            assert!(!is_overload(value));
        }
    }

    #[test]
    fn test_ncv_detection() {
        let ncv_values = vec!["EF", "-", "--", "---", "----", "-----"];
        for value in ncv_values {
            assert!(is_ncv(value));
        }
        let non_ncv_values = vec!["123.45", "0.00", "9999", "OL"];
        for value in non_ncv_values {
            assert!(!is_ncv(value));
        }
    }
}
