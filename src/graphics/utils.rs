use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn create_file(filepath: &str) -> BufWriter<File> {
    let path = Path::new(filepath);
    let display = path.display();
    match File::create(&path) {
        Err(why) => panic!("Could not create {}: {}", display, why),
        Ok(file) => BufWriter::new(file),
    }
}

pub fn polar_to_xy(mag: f64, angle_degrees: f64) -> (f64, f64) {
    let (dy, dx) = angle_degrees.to_radians().sin_cos();
    (dx * mag, dy * mag)
}