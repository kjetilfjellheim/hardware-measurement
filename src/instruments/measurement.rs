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

fn is_overload(value: &str) -> bool {
    OVERLOAD.contains(&value)
}

fn is_ncv(value: &str) -> bool {
    NCV.contains(&value)
}

/**
* Represents a measurement taken by an instrument.
 */
#[derive(Debug)]
pub struct Measurement {
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

impl Measurement {
    /**
     * Creates a new Measurement instance.
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

        Some(Measurement {
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
