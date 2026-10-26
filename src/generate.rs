use rand::{RngExt, SeedableRng};
use std::io::{self, Write};

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
            buffer_size: 8 * 1024 * 1024,
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
    let mut file = std::fs::File::create(config.file_name)?;
    let mut buf: Vec<u8> = Vec::with_capacity(config.buffer_size + 1024);
    let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

    let mut temp_table = [[0u8; 4]; 70];
    let mut temp_len = [0usize; 70];
    let mut int_buf = itoa::Buffer::new();
    for i in 0..70 {
        let s = int_buf.format(50 + i as u32).as_bytes();
        temp_table[i][..s.len()].copy_from_slice(s);
        temp_table[i][s.len()] = b'\n';
        temp_len[i] = s.len() + 1;
    }

    let mut remaining = config.count;
    while remaining > 0 {
        let batch = remaining.min(100_000);
        remaining -= batch;

        if buf.len() + batch * 16 > buf.capacity() {
            file.write_all(&buf)?;
            buf.clear();
        }

        unsafe {
            let mut ptr = buf.as_mut_ptr().add(buf.len());
            for _ in 0..batch {
                let r: u64 = rng.random();
                let city_idx = (((r as u32) as u64 * (CITIES_SIZE as u64)) >> 32) as usize;
                let temp_idx = (((r >> 32) * 70) >> 32) as usize;

                let city = config.cities.get_unchecked(city_idx).as_bytes();
                let t_bytes = temp_table.get_unchecked(temp_idx);
                let t_len = *temp_len.get_unchecked(temp_idx);

                std::ptr::copy_nonoverlapping(city.as_ptr(), ptr, city.len());
                ptr = ptr.add(city.len());

                std::ptr::copy_nonoverlapping(t_bytes.as_ptr(), ptr, 4);
                ptr = ptr.add(t_len);
            }
            let new_len = ptr.offset_from(buf.as_ptr()) as usize;
            buf.set_len(new_len);
        }
    }

    if !buf.is_empty() {
        file.write_all(&buf)?;
    }

    Ok(())
}
