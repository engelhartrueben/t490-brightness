use std::collections::HashMap;
use std::env;
use std::io::Result;

mod brightness;
// Different generations of think pads / different computers, probably have different files
// HOWEVER, I am giving this file root:root access, WE CANNOT LET THIS PROGRAM WRITE TO ANYTHING
// ELSE. Some polcy required....
const BRIGHTNESS_FILE: &str = "/sys/class/backlight/intel_backlight/brightness";

fn main() -> Result<()> {
    let args: HashMap<String, String> = get_arg_pairs().unwrap();

    // This program used to allow the user to specify the file to change, thinking that other
    // systems could benefit. HOWEVER, this approach of changing
    // /sys/class/backlight/intel_backlight/brightness requires this file to be root:root.
    // Too dangerous.

    let mut brightness: brightness::Brightness =
        brightness::Brightness::new(BRIGHTNESS_FILE.to_string());

    // This could obviously be done differently. We should
    // instead grab all the keys present in the arguments
    // and iterate thorugh them via a match statement.
    if (&args).contains_key("-d") {
        let amt = match (&args).get("-d") {
            Some(st) => {
                // nested match. EW
                match st.parse::<i32>() {
                    Err(err) => {
                        println!("ERROR| Failed to parse brightness arg: {err}");
                        0i32
                    }
                    Ok(v) => v,
                }
            }
            None => 0i32,
        };

        match brightness.down_brightness(amt, (&args).get("-mi")) {
            Err(err) => println!("ERROR| Failed to decrease brightness: {err}"),
            Ok(_) => (),
        };
    };

    if (&args).contains_key("-u") {
        let amt = match (&args).get("-u") {
            Some(st) => {
                // nested match. EW
                match st.parse::<i32>() {
                    Err(err) => {
                        println!("ERROR| Failed to parse brightness arg: {err}");
                        0i32
                    }
                    Ok(v) => v,
                }
            }
            None => 0i32,
        };

        match brightness.up_brightness(amt, (&args).get("-ma")) {
            Err(err) => println!("ERROR| Failed to increase brightness: {err}"),
            Ok(_) => (),
        };
    };

    // If the program makes it to here, do we notify considering main should have exited/failed
    // already
    Ok(())
}

// returns argument pairs
fn get_arg_pairs() -> Option<HashMap<String, String>> {
    // vector of args supplied by the cmd
    let args: Vec<String> = env::args().collect();

    // tuples of arguments:
    // (-d, 100) = brightness down by 100
    // (-u, 200) = brightness up by 200
    let mut commands: HashMap<String, String> = HashMap::new();

    // Is there a better way of handling this? Probably not a panic
    if args.len() <= 1 {
        println!("No args given. \"-h\" or \"--help\" for help.");
        return Option::None;
    }

    for (index, arg) in (&args).iter().enumerate() {
        // skip first arg (its the program)
        // OR even arg (its the flag argument)
        if index == 0 || index % 2 == 0 {
            continue;
        }

        // regardless of other args, if -h or --help is passed
        // print the help message and cancel future operations.
        if arg == "-h" || arg == "--help" {
            print_help();
            return None;
        }

        // because we skip all even args, every odd arg should contain
        // '-' as its first char.
        if arg.chars().nth(0).unwrap() != '-' {
            println!("Improper arguments. \"-h\" or \"--help\" for help.");
            return None;
        }

        commands.insert(arg.to_string(), (&args)[index + 1].to_string());
    }

    Some(commands)
}

fn print_help() {
    println!(
        "
        This program adjusts the birghtness of your screen.\n
        * commands not implemented yet
        Commands: 
            -h              print help message
            -u              increase the brightness (followed by integer)
            -d              decrease the brightness (followed by integer)
            -mi            the minimum brightness allowed (default 0)
            -ma            the maximum brightness allowed (NO DEFAULT)
                                ex: On a T490 /sys/class/backlight/intel_backlight/max_brightness
                                determines the max
            -maf*           the file that determines the max brightness of your machine
                                See -ma
    "
    );
}
