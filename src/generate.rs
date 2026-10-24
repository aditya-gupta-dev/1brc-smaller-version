use rand::{RngExt, SeedableRng};
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
            buffer_size: 256 * 1024,
            file_name: "measurements.txt",
            count: 10_000_000,
            cities: [
                "Jaipur;",
                "Lucknow;",
                "Hisar;",
                "Amritsar;",
                "Chennai;",
                "Ahemdabad;",
                "Shillong;",
                "Gwalior;",
                "Raipur;",
                "Raygada;",
            ],
        }
    }
}

pub fn generate_dataset(config: &Config) -> io::Result<()> {
    let file = std::fs::File::create(config.file_name)?;
    let mut buf_writer = std::io::BufWriter::with_capacity(config.buffer_size, file);
    let mut int_buf = itoa::Buffer::new();
    let mut random_device = rand::rngs::SmallRng::seed_from_u64(42);

    for _ in 0..config.count {
        let temp = random_device.random_range(50u32..120);
        buf_writer
            .write_all(config.cities[random_device.random_range(0..CITIES_SIZE)].as_bytes())?;
        buf_writer.write_all(int_buf.format(temp).as_bytes())?;
        buf_writer.write_all(b"\n")?;
    }

    buf_writer.flush()?;

    Ok(())
}
