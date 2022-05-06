use rand::Rng;
use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args()
        .nth(1)
        .ok_or("need input filename as first argument")?;
    let output_filename = env::args()
        .nth(2)
        .ok_or("need output filename as second argument")?;
    let mut bytes = fs::read(&filename)?;
    let mut rng = rand::thread_rng();

    for _ in 0..4 {
        let start_pos = rng.gen_range(0..bytes.len());
        let region_size = rng.gen_range(1..bytes.len() / 137);

        let region: Vec<_> = bytes
            .drain(start_pos..(start_pos + region_size).min(bytes.len()))
            .collect();
        let new_start_pos = rng.gen_range(0..bytes.len());
        bytes.splice(new_start_pos..new_start_pos, region);
    }

    let mut output_file = File::create(&output_filename)?;
    output_file.write_all(&bytes)?;

    Ok(())
}
