use clap::Parser;

/// Hardware measurement tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {

    /// HID device path
    #[arg(long)]
    pub hid: Option<String>,

}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }
}