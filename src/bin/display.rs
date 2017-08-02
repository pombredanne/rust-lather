extern crate rather;
extern crate png;

fn main() {
    use rather::simulation::Simulation;
    let mut sim = Simulation::new("/home/ben/rather/sun.cfg");
    let mut pix_image = vec![0; 1000*1000*4];

    sim.star.draw_rgba(&mut pix_image);
    for time in rather::linspace::linspace(6.0, 8.0, 30) {
        sim.draw_rgba(time, &mut pix_image);
        save_png(&pix_image, &format!("{:.2}.png", time));
        sim.undraw_rgba(time, &mut pix_image);
    }
}

fn save_png(image: &[u8], filename: &str) {
    use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;
    use png::HasParameters;

    let path = Path::new(filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 1000, 1000);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&image).unwrap();
}