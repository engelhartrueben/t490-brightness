use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Write};

/*
 * There is too much policy in this program. We should allow the freedom to break
 * things, but allow for cmd args to further control the program.
 *
 * This program is a MECAHNISM to control brightness, and the POLICY is found
 * in the cmd args.
 */

// Different generations of think pads / different computers, probable have different files
// HOWEVER, I am giving this file root:root access, WE CANNOT LET THIS PROGRAM WRITE TO ANYTHING
// ELSE. Some polcy required....
const BRIGHTNESS_FILE: &str = "/sys/class/backlight/intel_backlight/brightness";

struct Brightness {
    file_name: String,
    file: File,
    brightness: i32,
}

impl Brightness {
    pub fn up_brightness(&mut self, amt: i32, max: Option<&String>) -> Result<()> {
        self.brightness += amt;
        println!("{max:?}");
        // I feel this statment could be reduced in size
        match max {
            Some(max) => {
                // we allow max to start off as a string as other arguments might be a string,
                // like file names
                let max = str_to_i32(max);
                println!("max arg: {max}");
                if self.brightness > max {
                    self.brightness = max;
                }
            }
            None => (),
        }

        self.flush_and_resize()
            .expect("Failed to flush and resize brightness file");

        self.truncate_and_write().expect("Failed to write to file");

        Ok(())
    }

    pub fn down_brightness(&mut self, amt: i32, min: Option<&String>) -> Result<()> {
        self.brightness -= amt;

        match min {
            Some(min) => {
                let min = str_to_i32(min);
                if self.brightness < min {
                    self.brightness = min;
                }
            }
            // Is this too much policy? Really
            None => {
                if self.brightness < 0 {
                    self.brightness = 0;
                }
            }
        }

        self.flush_and_resize()
            .expect("Failed to flush and resize brightness file");

        self.truncate_and_write().expect("Faied to write to file");

        Ok(())
    }

    // Flush the buffer, and resize to 0.
    // Unsure if resizing is needed as we truncate the file later
    fn flush_and_resize(&mut self) -> Result<()> {
        self.file.set_len(0).expect("Could not truncate file.");
        self.file.flush()?;

        Ok(())
    }

    fn truncate_and_write(&mut self) -> Result<()> {
        /*
         * Have to rewrite self.file to truncate the file this way,
         * else a bunch of "0" bytes show up for some reason. This was not
         * caused by the writes, as I have checked the amount of bytes written.
         * Some other mechanism or rustism I don't understand.
         */
        self.file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.file_name)?;
        // do I need a .expect() here?

        // Write the new brighteness to the buffer
        self.file
            .write(&self.brightness.to_string().as_bytes())
            .expect("Could not write to file.");

        Ok(())
    }
}

fn main() -> Result<()> {
    let args: HashMap<String, String> = get_arg_pairs().unwrap();

    // This program used to allow the user to specify the file to change, thinking that other
    // systems could benefit. HOWEVER, this approach of chaning
    // /sys/class/backlight/intel_backlight/brightness requires this file to be root:root.
    // If an arg allowed for the changing of the file, then this file could write an integer to
    // any file on the machin.
    let mut brightness: Brightness = construct_brightness(BRIGHTNESS_FILE.to_string());

    if (&args).contains_key("-d") {
        // returns a Result, unser what to do with that at the moment
        brightness.down_brightness(str_to_i32((&args).get("-d").unwrap()), (&args).get("-mi"))?
    };

    if (&args).contains_key("-u") {
        brightness.up_brightness(str_to_i32((&args).get("-u").unwrap()), (&args).get("-ma"))?
    };

    // If the program makes it to here, do we notify?
    Ok(())
}

/*
 * No contructor in impl is odd tbh
 */
fn construct_brightness(file_name: String) -> Brightness {
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .open(&file_name)
        .expect(&format!("Failed to open file: {file_name}"));

    let mut current: String = Default::default();
    f.read_to_string(&mut current).unwrap();

    // to get rid of the new line character
    current.pop();

    Brightness {
        file_name,
        file: f,
        brightness: str_to_i32(&current),
    }
}

// returns arg pairs
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
        // OR if even arg (its the flag argument)
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

        // commands.add((arg.to_string(), (&args)[index + 1].to_string()));
        commands.insert(arg.to_string(), (&args)[index + 1].to_string());
    }

    Some(commands)
}

fn str_to_i32(s: &String) -> i32 {
    s.parse::<i32>().expect("Unable to parse string to i32")
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
            -f*             the file that determines the brightness of your machine 
                                ex: On a T490 /sys/class/backlight/intel_backlight/brightness
            -mi*            the minimum brightness allowed (default 0)
            -ma*            the maximum brightness allowed (NO DEFAULT)
                                ex: On a T490 /sys/class/backlight/intel_backlight/max_brightness
                                determines the max
            -maf*           the file that determines the max brightness of your machine
                                See -ma
    "
    );
}
