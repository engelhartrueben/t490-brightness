use clap::Parser;
use std::io::Result;

mod brightness;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Increase the brightness
    #[arg(short, long)]
    increase: Option<i32>,

    /// Decrease the brightness
    #[arg(short, long)]
    decrease: Option<i32>,

    /// Minimum brightness allowed
    #[arg(long)]
    min: Option<i32>,

    /// Maximum brightness allowed
    #[arg(long)]
    max: Option<i32>,
}

// Different generations of think pads / different computers, probably have different files
// HOWEVER, I am giving this file root:root access, WE CANNOT LET THIS PROGRAM WRITE TO ANYTHING
// ELSE. Some polcy required....
const BRIGHTNESS_FILE: &str = "/sys/class/backlight/intel_backlight/brightness";

fn main() -> Result<()> {
    let cli = Args::parse();

    let mut brightness = match brightness::Brightness::new(BRIGHTNESS_FILE.to_string()) {
        Err(err) => panic!("ERROR: {err:?}"),
        Ok(b) => b,
    };

    if let Some(increase) = cli.increase {
        let max = cli.max;
        let res = brightness.up_brightness(increase, max);

        match res {
            Err(err) => println!("ERROR: {err:?}"),
            _ => (),
        };
    };

    if let Some(decrease) = cli.decrease {
        let min = cli.min.unwrap_or_else(|| 0);
        let res = brightness.down_brightness(decrease, min);

        match res {
            Err(err) => println!("ERROR: {err:?}"),
            _ => (),
        };
    }

    Ok(())
}
