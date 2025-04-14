use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Write};

pub struct Brightness {
    file_name: String,
    file: File,
    brightness: i32,
}

impl Brightness {
    // pub or priv?
    pub fn new(file_name: String) -> Brightness {
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
            // Dangerous unwrap
            brightness: (&current).parse::<i32>().unwrap(),
        }
    }

    pub fn up_brightness(&mut self, amt: i32, max: Option<&String>) -> Result<()> {
        self.brightness += amt;
        println!("{max:?}");
        // I feel this statment could be reduced in size
        match max {
            Some(max) => {
                // Dangerous unwrap
                let max = max.parse::<i32>().unwrap();
                println!("max arg: {max}");
                if self.brightness > max {
                    self.brightness = max;
                }
            }
            None => (),
        }

        self.flush_and_resize()?;

        self.truncate_and_write()?;

        Ok(())
    }

    pub fn down_brightness(&mut self, amt: i32, min: Option<&String>) -> Result<()> {
        self.brightness -= amt;

        match min {
            Some(min) => {
                // Dangerous unwrap
                let min = min.parse::<i32>().unwrap();
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

        self.flush_and_resize()?;

        self.truncate_and_write()?;

        Ok(())
    }

    // Flush the buffer, and resize to 0.
    // Unsure if resizing is needed as we truncate the file later
    fn flush_and_resize(&mut self) -> Result<()> {
        self.file.set_len(0)?;
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

        // Write the new brighteness to the buffer
        self.file.write(&self.brightness.to_string().as_bytes())?;

        Ok(())
    }
}
