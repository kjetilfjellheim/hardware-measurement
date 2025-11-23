mod arguments;
mod instruments;

use arguments::Args;

use crate::{instruments::{instrument::{Instrument}, unit161d::Unit161dHid}};

fn main() {
    let args = Args::parse_args();
    let instrument = Unit161dHid::new(
        &args.hid
    );

    let measurement =instrument.command(args.command.into());
    println!("{:?}", measurement);
    
}