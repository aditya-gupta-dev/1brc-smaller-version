use rand::RngExt;
use std::io::Write;
use std::io::{self};

const CITIES_SIZE: usize = 10;

pub struct Config {
    count: usize,
    buffer_size: usize,
    file_name: &'static str,
    cities: [&'static str; CITIES_SIZE],
}

impl Default for Config {
    fn default() -> Self {
        Config {
            buffer_size: 1024 * 1024,
            file_name: "measurements.txt",
            count: 10_000_000,
            cities: [
                "Jaipur",
                "Lucknow",
                "Hisar",
                "Amritsar",
                "Chennai",
                "Ahemdabad",
                "Shillong",
                "Gwalior",
                "Raipur",
                "Raygada",
            ],
        }
    }
}

pub fn generate_dataset(config: &Config) -> io::Result<()> {
    let file = std::fs::File::create(config.file_name)?;
    let mut buf_writer = std::io::BufWriter::with_capacity(config.buffer_size, file);
    let mut random_device = rand::rng();

    for _ in 0..config.count {
        let city = config.cities[random_device.random_range(0..CITIES_SIZE)];
        let temp = random_device.random_range(50u32..120);
        writeln!(buf_writer, "{};{}.{:01}", city, temp / 10, temp % 10)?;
    }

    buf_writer.flush()?;

    Ok(())
}
