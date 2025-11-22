mod arguments;
mod instruments;

use arguments::Args;

use crate::instruments::{instrument::{self, Instrument}, unit161d::Unit161dHid};

fn main() {
    let args = Args::parse_args();
    let instrument = Unit161dHid::new(
        &args.hid
    );
    let measurement = instrument.get_measurement();
    if let Some(measurement) = measurement {
        println!("Measurement: {:?}", measurement);
    } else {
        eprintln!("Failed to get measurement from the instrument.");
    }
}